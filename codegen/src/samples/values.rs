use std::collections::HashMap;

use heck::ToSnakeCase;
use oas3::Spec;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde_json::Value;
use syn::{GenericArgument, Item, PathArguments, Type};

/// Read the SDK's generated declarations so sample types and serde field names
/// follow the same rules as the SDK, including inline types and shared schemas.
pub(super) struct SampleTypes {
    modules: HashMap<String, HashMap<String, Item>>,
}

impl SampleTypes {
    pub(super) fn new(spec: &Spec) -> Result<Self, String> {
        let schemas = crate::collect_schemas_by_tag(spec)?;
        let mut types = Self {
            modules: HashMap::new(),
        };
        types.insert(
            "common",
            crate::generate_structs_for_schemas(
                spec,
                &schemas.common_schemas,
                &schemas.common_error_schemas,
            )?,
        )?;
        for (tag, schemas) in &schemas.tag_schemas {
            let mut tokens = crate::generate_structs_for_schemas(
                spec,
                &schemas.all_schemas,
                &schemas.error_schemas,
            )?;
            tokens.extend(crate::generate_operation_bodies(spec, tag)?);
            types.insert(&tag.to_snake_case(), tokens)?;
        }
        Ok(types)
    }

    fn insert(&mut self, module: &str, tokens: TokenStream) -> Result<(), String> {
        let file = syn::parse2::<syn::File>(tokens).map_err(|error| error.to_string())?;
        let declarations = file
            .items
            .into_iter()
            .filter_map(|item| {
                let name = match &item {
                    Item::Struct(item) => &item.ident,
                    Item::Enum(item) => &item.ident,
                    Item::Type(item) => &item.ident,
                    _ => return None,
                }
                .to_string();
                Some((name, item))
            })
            .collect();
        self.modules.insert(module.to_string(), declarations);
        Ok(())
    }

    pub(super) fn render(
        &self,
        module: &str,
        name: &Ident,
        value: &Value,
    ) -> Result<String, String> {
        self.value(module, &syn::parse_quote!(#name), Some(value), 0)
    }

    fn value(
        &self,
        module: &str,
        ty: &Type,
        value: Option<&Value>,
        depth: usize,
    ) -> Result<String, String> {
        if depth > 40 {
            return Err("recursive sample type exceeds maximum depth".into());
        }
        let Type::Path(path) = ty else {
            return Err(format!("unsupported sample type: {}", quote!(#ty)));
        };
        let segment = path.path.segments.last().ok_or("empty type path")?;
        let name = segment.ident.to_string();
        let args = match &segment.arguments {
            PathArguments::AngleBracketed(args) => args
                .args
                .iter()
                .filter_map(|arg| match arg {
                    GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        };
        let nested = |ty, value| self.value(module, ty, value, depth + 1);
        match name.as_str() {
            "Option" => {
                let Some(value) = value else {
                    return Ok("None".into());
                };
                if value.is_null()
                    && !matches!(args[0], Type::Path(path) if path.path.segments.last().is_some_and(|s| s.ident == "Nullable"))
                {
                    return Ok("None".into());
                }
                return Ok(format!("Some({})", nested(args[0], Some(value))?));
            }
            "Nullable" => {
                return if value.is_none_or(Value::is_null) {
                    Ok("sumup::Nullable::Null".into())
                } else {
                    Ok(format!(
                        "sumup::Nullable::Value({})",
                        nested(args[0], value)?
                    ))
                };
            }
            "Vec" => {
                let values = match value {
                    Some(Value::Array(values)) => values
                        .iter()
                        .map(|value| nested(args[0], Some(value)))
                        .collect::<Result<Vec<_>, _>>()?,
                    Some(Value::String(value)) if matches!(args[0], Type::Path(path) if path.path.is_ident("u8")) => {
                        value.bytes().map(|byte| byte.to_string()).collect()
                    }
                    _ => Vec::new(),
                };
                return Ok(format!("vec![{}]", values.join(", ")));
            }
            "HashMap" => {
                let mut entries = Vec::new();
                if let Some(Value::Object(values)) = value {
                    for (key, value) in values.iter().collect::<std::collections::BTreeMap<_, _>>()
                    {
                        entries.push(format!(
                            "({key:?}.to_string(), {})",
                            nested(args[1], Some(value))?
                        ));
                    }
                }
                return Ok(if entries.is_empty() {
                    "std::collections::HashMap::new()".into()
                } else {
                    format!("std::collections::HashMap::from([{}])", entries.join(", "))
                });
            }
            "String" => {
                return Ok(format!(
                    "{:?}.to_string()",
                    value.and_then(Value::as_str).unwrap_or_default()
                ));
            }
            "Secret" => {
                return Ok(format!(
                    "sumup::secret::Secret::new({:?})",
                    value.and_then(Value::as_str).unwrap_or_default()
                ));
            }
            "Date" | "DateTime" => {
                let fallback = if name == "Date" {
                    "2024-01-01"
                } else {
                    "2024-01-01T00:00:00Z"
                };
                return Ok(format!(
                    "{:?}.parse().expect(\"valid date\")",
                    value.and_then(Value::as_str).unwrap_or(fallback)
                ));
            }
            "bool" => return Ok(value.and_then(Value::as_bool).unwrap_or(false).to_string()),
            "f32" | "f64" | "i32" | "i64" | "u8" => {
                let number = match value {
                    Some(Value::Number(number)) => number.to_string(),
                    Some(Value::String(number)) => number.clone(),
                    _ => "0".into(),
                };
                return Ok(format!("{number}{name}"));
            }
            "Value"
                if path
                    .path
                    .segments
                    .first()
                    .is_some_and(|s| s.ident == "serde_json") =>
            {
                return Ok(format!(
                    "serde_json::json!({})",
                    value.unwrap_or(&Value::Null)
                ));
            }
            _ => {}
        }
        let (module, item) = self
            .modules
            .get(module)
            .and_then(|items| items.get(&name))
            .map(|item| (module, item))
            .or_else(|| {
                self.modules
                    .get("common")?
                    .get(&name)
                    .map(|item| ("common", item))
            })
            .ok_or_else(|| format!("unknown sample type {module}::{name}"))?;
        let type_path = format!("sumup::resources::{module}::{name}");
        match item {
            Item::Type(item) => self.value(module, &item.ty, value, depth + 1),
            Item::Struct(item) => {
                let mut fields = Vec::new();
                let mut omitted = false;
                let has_default = item.attrs.iter().any(|attr| {
                    attr.path().is_ident("derive") && quote!(#attr).to_string().contains("Default")
                });
                let names = item
                    .fields
                    .iter()
                    .map(|field| {
                        wire_name(&field.attrs, &field.ident.as_ref().unwrap().to_string())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                for (field, name) in item.fields.iter().zip(&names) {
                    let ident = field.ident.as_ref().ok_or("unnamed struct field")?;
                    let flatten = has_serde_flag(&field.attrs, "flatten")?;
                    let extra = value.and_then(Value::as_object).map(|values| {
                        Value::Object(
                            values
                                .iter()
                                .filter(|(key, _)| !names.contains(key))
                                .map(|(key, value)| (key.clone(), value.clone()))
                                .collect(),
                        )
                    });
                    let field_value = if flatten {
                        extra.as_ref()
                    } else {
                        value.and_then(|value| value.get(name))
                    };
                    if field_value.is_none() && has_default {
                        omitted = true;
                        continue;
                    }
                    fields.push(format!(
                        "{ident}: {}",
                        self.value(module, &field.ty, field_value, depth + 1)?
                    ));
                }
                if omitted {
                    fields.push("..Default::default()".into());
                }
                Ok(format!("{type_path} {{ {} }}", fields.join(", ")))
            }
            Item::Enum(item) => {
                let text = value.and_then(Value::as_str);
                for variant in &item.variants {
                    if matches!(variant.fields, syn::Fields::Unit)
                        && (text.is_none()
                            || text
                                == Some(
                                    wire_name(&variant.attrs, &variant.ident.to_string())?.as_str(),
                                ))
                    {
                        return Ok(format!("{type_path}::{}", variant.ident));
                    }
                }
                if let Some(variant) = item.variants.iter().find(|variant| matches!(&variant.fields, syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1)) {
                    return Ok(format!("{type_path}::{}({})", variant.ident, self.value(module, &variant.fields.iter().next().unwrap().ty, value, depth + 1)?));
                }
                Err(format!("no matching variant for {type_path}: {value:?}"))
            }
            _ => Err(format!("unsupported declaration {type_path}")),
        }
    }
}

fn wire_name(attrs: &[syn::Attribute], fallback: &str) -> Result<String, String> {
    let mut name = fallback.trim_start_matches("r#").to_string();
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("serde")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                name = meta.value()?.parse::<syn::LitStr>()?.value();
            } else if meta.input.peek(syn::Token![=]) {
                let _ = meta.value()?.parse::<syn::Expr>()?;
            }
            Ok(())
        })
        .map_err(|error| error.to_string())?;
    }
    Ok(name)
}

fn has_serde_flag(attrs: &[syn::Attribute], flag: &str) -> Result<bool, String> {
    let mut found = false;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("serde")) {
        attr.parse_nested_meta(|meta| {
            found |= meta.path.is_ident(flag);
            if meta.input.peek(syn::Token![=]) {
                let _ = meta.value()?.parse::<syn::Expr>()?;
            }
            Ok(())
        })
        .map_err(|error| error.to_string())?;
    }
    Ok(found)
}
