use oas3::Spec;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, PathArguments, Type};

/// Adds builders using the emitted field types, keeping their optionality and aliases intact.
pub(crate) fn generate(
    spec: &Spec,
    tokens: TokenStream,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<TokenStream, String> {
    let mut item = syn::parse2::<syn::ItemStruct>(tokens).map_err(|error| error.to_string())?;
    symbols.reserve(
        format!("{}Builder", item.ident),
        format!("builder for `{}`", item.ident),
    )?;
    item.attrs
        .push(syn::parse_quote!(#[derive(typed_builder::TypedBuilder)]));
    item.attrs.push(syn::parse_quote!(#[builder(doc)]));

    for field in &mut item.fields {
        let optional = option_inner(&field.ty);
        let ty = optional.unwrap_or(&field.ty);
        let mut settings = Vec::new();
        let mut setter = Vec::new();
        if optional.is_some() {
            settings.push(quote!(default));
            setter.push(quote!(strip_option));
        } else if field.attrs.iter().any(|attr| {
            attr.path().is_ident("serde")
                && attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .is_ok_and(|items| items.iter().any(|meta| meta.path().is_ident("flatten")))
        }) {
            settings.push(quote!(default));
        }
        if is_string(spec, ty, &mut Vec::new()) {
            setter.push(quote!(into));
        }
        let name = field
            .ident
            .as_ref()
            .expect("generated structs have named fields");
        let doc = format!("Sets [`{name}`]({}::{name}).", item.ident);
        setter.push(quote!(doc = #doc));
        settings.push(quote!(setter(#(#setter),*)));
        field
            .attrs
            .push(syn::parse_quote!(#[builder(#(#settings),*)]));
    }
    Ok(quote!(#item))
}

pub(crate) fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(path) = ty else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(ty) => Some(ty),
        _ => None,
    }
}

fn is_string(spec: &Spec, ty: &Type, visited: &mut Vec<String>) -> bool {
    use heck::ToUpperCamelCase;
    let Type::Path(path) = ty else { return false };
    let Some(segment) = path.path.segments.last() else {
        return false;
    };
    let name = segment.ident.to_string();
    if name == "String" {
        return true;
    }
    if !matches!(segment.arguments, PathArguments::None) || visited.contains(&name) {
        return false;
    }
    visited.push(name.clone());
    let Some(schema) = spec.components.as_ref().and_then(|components| {
        components.schemas.iter().find_map(|(schema_name, schema)| {
            (schema_name.to_upper_camel_case() == name).then_some(schema)
        })
    }) else {
        return false;
    };
    let Some(schema) = crate::oas::schema_object(schema) else {
        return false;
    };
    let schema = crate::oas::schema_reference(schema);
    if let oas3::spec::ObjectOrReference::Object(schema) = schema
        && !schema.enum_values.is_empty()
    {
        return false;
    }
    let inferred = crate::schema::infer_rust_type(true, false, None, schema);
    syn::parse2::<Type>(inferred).is_ok_and(|ty| is_string(spec, &ty, visited))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn builder_names_cannot_collide_with_model_names() {
        let spec: Spec = serde_json::from_value(serde_json::json!({
            "openapi": "3.1.0",
            "info": {"title": "Builders", "version": "1"},
            "paths": {},
            "components": {"schemas": {
                "Model": {"type": "object", "properties": {"name": {"type": "string"}}},
                "ModelBuilder": {"type": "string"}
            }}
        }))
        .unwrap();
        let names = HashSet::from(["Model".to_owned(), "ModelBuilder".to_owned()]);
        let error = crate::generate_structs_for_schemas(&spec, &names, &HashSet::new())
            .expect_err("model and builder names must be checked together");
        assert!(error.contains("builder for `Model`"));
        assert!(error.contains("component schema `ModelBuilder`"));
    }
}
