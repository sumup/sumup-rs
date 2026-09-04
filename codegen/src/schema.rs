use std::collections::{BTreeMap, HashSet};

use heck::ToUpperCamelCase;
use oas3::{
    Spec as OpenAPI,
    spec::{BooleanSchema, ObjectOrReference, ObjectSchema, Schema, SchemaType},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

type Properties = BTreeMap<String, ObjectOrReference<ObjectSchema>>;
type FlattenedObject = Option<(Properties, Vec<String>, Option<Schema>)>;

pub(crate) fn should_emit_free_form_object_alias(p: &Properties, a: Option<&Schema>) -> bool {
    p.is_empty() && a.is_none()
}

pub fn generate_doc_comment(description: &str) -> TokenStream {
    generate_doc_comment_from_lines(
        description
            .lines()
            .map(|line| line.trim().to_owned())
            .collect(),
    )
}

pub fn generate_module_doc_comment(description: &str) -> TokenStream {
    let attrs = description.lines().map(|line| {
        let line = line.trim();
        let line = if line.is_empty() {
            String::new()
        } else {
            format!(" {line}")
        };
        quote! { #![doc = #line] }
    });
    quote! { #(#attrs)* }
}

pub fn generate_schema_doc_comment(
    description: Option<&str>,
    schema: &ObjectSchema,
) -> TokenStream {
    let mut lines = description
        .into_iter()
        .flat_map(str::lines)
        .map(|line| line.trim().to_owned())
        .collect::<Vec<_>>();
    let constraints = constraints(schema);
    if !constraints.is_empty() {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.push("Constraints:".into());
        lines.extend(constraints.into_iter().map(|v| format!("- {v}")));
    }
    if let Some(value) = crate::oas::schema_example(schema).and_then(format_json_example) {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.push(format!("Example: `{value}`"));
    }
    generate_doc_comment_from_lines(lines)
}

pub(crate) fn format_json_example(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null => Some("null".into()),
        serde_json::Value::Bool(v) => Some(v.to_string()),
        serde_json::Value::Number(v) => Some(v.to_string()),
        serde_json::Value::String(v) => Some(v.clone()),
        _ => None,
    }
}

pub(crate) fn generate_doc_comment_from_lines(lines: Vec<String>) -> TokenStream {
    let attrs = lines.into_iter().map(|line| {
        let line = if line.is_empty() {
            String::new()
        } else {
            format!(" {line}")
        };
        quote! { #[doc = #line] }
    });
    quote! { #(#attrs)* }
}

fn constraints(s: &ObjectSchema) -> Vec<String> {
    let mut out = Vec::new();
    if s.read_only.unwrap_or(false) {
        out.push("read-only".into());
    }
    if s.write_only.unwrap_or(false) {
        out.push("write-only".into());
    }
    if let Some(v) = s.format.as_deref()
        && !matches!(
            v,
            "date-time"
                | "date"
                | "password"
                | "byte"
                | "binary"
                | "float"
                | "double"
                | "int32"
                | "int64"
        )
    {
        out.push(format!("format: `{v}`"));
    }
    if let Some(v) = &s.pattern {
        out.push(format!("pattern: `{v}`"));
    }
    if let Some(v) = s.min_length {
        out.push(format!("min length: {v}"));
    }
    if let Some(v) = s.max_length {
        out.push(format!("max length: {v}"));
    }
    if let Some(v) = &s.multiple_of {
        out.push(format!("multiple of: {v}"));
    }
    if let Some(v) = &s.minimum {
        out.push(format!("value >= {v}"));
    }
    if let Some(v) = &s.exclusive_minimum {
        out.push(format!("value > {v}"));
    }
    if let Some(v) = &s.maximum {
        out.push(format!("value <= {v}"));
    }
    if let Some(v) = &s.exclusive_maximum {
        out.push(format!("value < {v}"));
    }
    if let Some(v) = s.min_items {
        out.push(format!("min items: {v}"));
    }
    if let Some(v) = s.max_items {
        out.push(format!("max items: {v}"));
    }
    if s.unique_items.unwrap_or(false) {
        out.push("items must be unique".into());
    }
    if let Some(v) = s.min_properties {
        out.push(format!("min properties: {v}"));
    }
    if let Some(v) = s.max_properties {
        out.push(format!("max properties: {v}"));
    }
    out
}

pub fn make_rust_field_ident(name: &str) -> Ident {
    let name = name.to_lowercase().replace('-', "_");
    if is_keyword(&name) {
        Ident::new_raw(&name, Span::call_site())
    } else {
        Ident::new(&name, Span::call_site())
    }
}
fn is_keyword(v: &str) -> bool {
    matches!(
        v,
        "as" | "break"
            | "const"
            | "continue"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "static"
            | "struct"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

#[derive(Clone, Copy)]
enum Numeric {
    F32,
    F64,
    I32,
    I64,
}
fn string_numeric(s: &ObjectSchema) -> Option<Numeric> {
    if crate::oas::schema_type(s) != Some(SchemaType::String) {
        return None;
    }
    match s.format.as_deref()?.trim().to_ascii_lowercase().as_str() {
        "float" => Some(Numeric::F32),
        "double" | "number" => Some(Numeric::F64),
        "int32" | "integer" => Some(Numeric::I32),
        "int64" => Some(Numeric::I64),
        _ => None,
    }
}
fn numeric_type(v: Numeric) -> TokenStream {
    match v {
        Numeric::F32 => quote! {f32},
        Numeric::F64 => quote! {f64},
        Numeric::I32 => quote! {i32},
        Numeric::I64 => quote! {i64},
    }
}

pub fn generate_structs_for_schemas(
    spec: &OpenAPI,
    names: &HashSet<String>,
    errors: &HashSet<String>,
) -> Result<TokenStream, String> {
    let mut symbols = crate::symbol::SymbolRegistry::new("standalone schemas");
    generate_structs_for_schemas_with_registry(spec, names, errors, &mut symbols)
}

pub(crate) fn generate_structs_for_schemas_with_registry(
    spec: &OpenAPI,
    names: &HashSet<String>,
    errors: &HashSet<String>,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<TokenStream, String> {
    let Some(c) = &spec.components else {
        return Ok(quote! {});
    };
    let skipped = collect_mixins(spec, names);
    let mut names = names.iter().collect::<Vec<_>>();
    names.sort();
    let mut items = Vec::new();
    let mut nested = Vec::new();
    for name in names {
        if skipped.contains(name.as_str()) {
            continue;
        }
        let Some(ObjectOrReference::Object(s)) = c.schemas.get(name) else {
            continue;
        };
        let type_name = name.to_upper_camel_case();
        symbols.reserve(type_name.clone(), format!("component schema `{name}`"))?;
        let id = Ident::new(&type_name, Span::call_site());
        let doc = s
            .description
            .as_deref()
            .map(|v| generate_schema_doc_comment(Some(v), s));
        let dep = deprecation(s);
        let obj = object_parts(spec, s)?;
        if let Some((p, r, a)) = obj {
            if should_emit_free_form_object_alias(&p, a.as_ref()) {
                items.push(quote! {#doc #dep pub type #id=serde_json::Value;});
                continue;
            }
            collect_nested_schemas_with_registry(spec, name, &p, &mut nested, symbols)?;
            let fields = generate_struct_fields(name, &p, &r, a.as_ref());
            let derive = if can_fields_derive_default(&p, &r) {
                quote! {#[derive(Debug,Clone,Default,PartialEq,serde::Serialize,serde::Deserialize)]}
            } else {
                quote! {#[derive(Debug,Clone,PartialEq,serde::Serialize,serde::Deserialize)]}
            };
            items.push(quote! {#doc #dep #derive pub struct #id{#(#fields)*}});
            if errors.contains(name) {
                items.push(error_impl(&id, &p, &r));
            }
        } else if crate::oas::schema_type(s) == Some(SchemaType::String)
            && !s.enum_values.is_empty()
        {
            items.push(string_enum(
                &id,
                &s.enum_values,
                s.description.as_deref(),
                s,
            )?)
        } else {
            let sr = ObjectOrReference::Object(s.clone());
            let ty = infer_rust_type(true, false, None, &sr);
            items.push(quote! {#doc #dep pub type #id=#ty;});
        }
    }
    items.extend(nested);
    Ok(quote! {#(#items)*})
}

pub(crate) fn schema_symbol_names(
    spec: &OpenAPI,
    n: &HashSet<String>,
    e: &HashSet<String>,
) -> Result<Vec<String>, String> {
    let mut s = crate::symbol::SymbolRegistry::new("schema symbol discovery");
    generate_structs_for_schemas_with_registry(spec, n, e, &mut s)?;
    Ok(s.names().map(str::to_owned).collect())
}

fn deprecation(s: &ObjectSchema) -> TokenStream {
    if !s.deprecated.unwrap_or(false) {
        return quote! {};
    }
    if let Some(v) = s
        .extensions
        .get("deprecation-notice")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
    {
        quote! {#[deprecated(note=#v)]}
    } else {
        quote! {#[deprecated]}
    }
}

fn collect_mixins(spec: &OpenAPI, names: &HashSet<String>) -> HashSet<String> {
    let mut out = HashSet::new();
    if let Some(c) = &spec.components {
        for s in c.schemas.values() {
            mixin_refs(s, names, &mut out)
        }
    }
    out
}
fn mixin_refs(
    sr: &ObjectOrReference<ObjectSchema>,
    names: &HashSet<String>,
    out: &mut HashSet<String>,
) {
    let ObjectOrReference::Object(s) = sr else {
        return;
    };
    for v in &s.all_of {
        match v {
            ObjectOrReference::Ref { ref_path, .. } => {
                if let Some(n) = ref_path.strip_prefix("#/components/schemas/")
                    && names.contains(n)
                    && n.contains("Mixin")
                {
                    out.insert(n.into());
                }
            }
            ObjectOrReference::Object(s) => {
                mixin_refs(&ObjectOrReference::Object(s.clone()), names, out)
            }
        }
    }
}

fn object_parts(spec: &OpenAPI, s: &ObjectSchema) -> Result<FlattenedObject, String> {
    if crate::oas::schema_type(s) == Some(SchemaType::Object) || !s.properties.is_empty() {
        Ok(Some((
            s.properties.clone(),
            s.required.clone(),
            s.additional_properties.clone(),
        )))
    } else if !s.all_of.is_empty() {
        flatten_all_of_object(spec, &s.all_of)
    } else {
        Ok(None)
    }
}

pub fn collect_nested_schemas(
    spec: &OpenAPI,
    parent: &str,
    p: &Properties,
    out: &mut Vec<TokenStream>,
) -> Result<(), String> {
    let mut symbols = crate::symbol::SymbolRegistry::new(parent);
    collect_nested_schemas_with_registry(spec, parent, p, out, &mut symbols)
}
pub(crate) fn collect_nested_schemas_with_registry(
    spec: &OpenAPI,
    parent: &str,
    p: &Properties,
    out: &mut Vec<TokenStream>,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<(), String> {
    for (field, sr) in p {
        let ObjectOrReference::Object(s) = sr else {
            continue;
        };
        if crate::oas::schema_type(s) == Some(SchemaType::Array) {
            if let Some(Schema::Object(item)) = s.items.as_deref()
                && let ObjectOrReference::Object(item) = item.as_ref()
            {
                nested(spec, parent, field, "Item", item, out, symbols)?
            }
        } else {
            nested(spec, parent, field, "", s, out, symbols)?
        }
    }
    Ok(())
}
fn nested(
    spec: &OpenAPI,
    parent: &str,
    field: &str,
    suffix: &str,
    s: &ObjectSchema,
    out: &mut Vec<TokenStream>,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<(), String> {
    let name = nested_name(parent, field, suffix);
    let id = Ident::new(&name, Span::call_site());
    if crate::oas::schema_type(s) == Some(SchemaType::String) && !s.enum_values.is_empty() {
        symbols.reserve(name, format!("inline enum for field `{parent}.{field}`"))?;
        out.push(string_enum(
            &id,
            &s.enum_values,
            s.description.as_deref(),
            s,
        )?);
        return Ok(());
    }
    let Some((p, r, a)) = object_parts(spec, s)? else {
        return Ok(());
    };
    if should_emit_free_form_object_alias(&p, a.as_ref()) {
        return Ok(());
    }
    symbols.reserve(
        name.clone(),
        format!("inline schema for field `{parent}.{field}`"),
    )?;
    collect_nested_schemas_with_registry(spec, &name, &p, out, symbols)?;
    let fields = generate_struct_fields(&name, &p, &r, a.as_ref());
    let doc = s
        .description
        .as_deref()
        .map(|v| generate_schema_doc_comment(Some(v), s));
    let derive = if can_fields_derive_default(&p, &r) {
        quote! {#[derive(Debug,Clone,Default,PartialEq,serde::Serialize,serde::Deserialize)]}
    } else {
        quote! {#[derive(Debug,Clone,PartialEq,serde::Serialize,serde::Deserialize)]}
    };
    out.push(quote! {#doc #derive pub struct #id{#(#fields)*}});
    Ok(())
}

pub(crate) fn flatten_all_of_object(
    spec: &OpenAPI,
    all: &[ObjectOrReference<ObjectSchema>],
) -> Result<FlattenedObject, String> {
    let mut p = Properties::new();
    let mut r = Vec::new();
    let mut a = None;
    let mut found = false;
    for sr in all {
        let s = dereference_schema(spec, sr)?;
        if let Some((np, nr, na)) = object_parts(spec, s)? {
            found = true;
            p.extend(np);
            for v in nr {
                if !r.contains(&v) {
                    r.push(v)
                }
            }
            if let Some(v) = na {
                merge_additional(&mut a, v)
            }
        }
    }
    Ok(found.then_some((p, r, a)))
}
fn merge_additional(target: &mut Option<Schema>, v: Schema) {
    match &v {
        Schema::Boolean(BooleanSchema(true)) => *target = Some(v),
        Schema::Object(_) if !matches!(target, Some(Schema::Boolean(BooleanSchema(true)))) => {
            *target = Some(v)
        }
        Schema::Boolean(BooleanSchema(false)) if target.is_none() => *target = Some(v),
        _ => {}
    }
}

pub(crate) fn dereference_schema<'a>(
    spec: &'a OpenAPI,
    sr: &'a ObjectOrReference<ObjectSchema>,
) -> Result<&'a ObjectSchema, String> {
    match sr {
        ObjectOrReference::Object(s) => Ok(s),
        ObjectOrReference::Ref { ref_path, .. } => {
            let n = ref_path
                .strip_prefix("#/components/schemas/")
                .ok_or_else(|| format!("Unsupported schema reference: {ref_path}"))?;
            let t = spec
                .components
                .as_ref()
                .ok_or_else(|| "OpenAPI spec is missing components section".to_owned())?
                .schemas
                .get(n)
                .ok_or_else(|| format!("Referenced schema '{n}' not found"))?;
            dereference_schema(spec, t)
        }
    }
}

pub fn can_fields_derive_default(p: &Properties, r: &[String]) -> bool {
    p.iter().all(|(n, sr)| {
        if !r.contains(n) {
            return true;
        }
        let ObjectOrReference::Object(s) = sr else {
            return false;
        };
        matches!(
            crate::oas::schema_type(s),
            Some(SchemaType::Array | SchemaType::Null) | None
        ) && s.all_of.is_empty()
    })
}

pub fn generate_struct_fields(
    parent: &str,
    p: &Properties,
    r: &[String],
    additional: Option<&Schema>,
) -> Vec<TokenStream> {
    let mut fields=p.iter().map(|(name,sr)|{let id=make_rust_field_ident(name);let req=r.contains(name);let(ty,doc,dep,nullable,num)=match sr{ObjectOrReference::Ref{ref_path,..}=>{let n=ref_path.split('/').next_back().unwrap_or("Unknown").to_upper_camel_case();let t=Ident::new(&n,Span::call_site());(if req{quote!{#t}}else{quote!{Option<#t>}},None,quote!{},false,false)},ObjectOrReference::Object(s)=>{let nul=s.is_nullable().unwrap_or(false);(infer_rust_type(req,nul,Some((parent,name)),sr),s.description.as_deref().map(|v|generate_schema_doc_comment(Some(v),s)),deprecation(s),nul,string_numeric(s).is_some())}};let rename=(name!=&id.to_string()).then(||quote!{#[serde(rename=#name)]});let attrs=if !req&&nullable{if num{quote!{#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="crate::nullable::deserialize_string_or_number")]}}else{quote!{#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="crate::nullable::deserialize")]}}}else if !req&&num{quote!{#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="crate::string_or_number::deserialize_option")]}}else if !req{quote!{#[serde(skip_serializing_if="Option::is_none")]}}else if num{quote!{#[serde(deserialize_with="crate::string_or_number::deserialize")]}}else{quote!{}};quote!{#doc #dep #rename #attrs pub #id:#ty,}}).collect::<Vec<_>>();
    if let Some(v) = additional_field(p, additional) {
        fields.push(v)
    }
    fields
}

fn additional_field(p: &Properties, a: Option<&Schema>) -> Option<TokenStream> {
    let ty = match a? {
        Schema::Boolean(BooleanSchema(true)) => quote! {serde_json::Value},
        Schema::Boolean(BooleanSchema(false)) => return None,
        Schema::Object(sr) => match sr.as_ref() {
            ObjectOrReference::Ref { ref_path, .. } => {
                let n = ref_path
                    .split('/')
                    .next_back()
                    .unwrap_or("Unknown")
                    .to_upper_camel_case();
                let id = Ident::new(&n, Span::call_site());
                quote! {#id}
            }
            ObjectOrReference::Object(s)
                if crate::oas::schema_type(s) == Some(SchemaType::Object)
                    || !s.all_of.is_empty() =>
            {
                quote! {serde_json::Value}
            }
            ObjectOrReference::Object(s) => {
                infer_rust_type(true, false, None, &ObjectOrReference::Object(s.clone()))
            }
        },
    };
    let id = additional_id(p);
    Some(
        quote! {#[serde(flatten,default,skip_serializing_if="std::collections::HashMap::is_empty")]pub #id:std::collections::HashMap<String,#ty>,},
    )
}
fn additional_id(p: &Properties) -> Ident {
    let e = p
        .keys()
        .map(|n| make_rust_field_ident(n).to_string())
        .collect::<HashSet<_>>();
    [
        "additional_properties".to_owned(),
        "extra_properties".to_owned(),
        "extra".to_owned(),
        format!("extra_properties_{}", p.len()),
    ]
    .into_iter()
    .find(|n| !e.contains(n))
    .map(|n| Ident::new(&n, Span::call_site()))
    .unwrap_or_else(|| Ident::new("extra_properties_fallback", Span::call_site()))
}

pub fn infer_rust_type(
    required: bool,
    nullable: bool,
    parent: Option<(&str, &str)>,
    sr: &ObjectOrReference<ObjectSchema>,
) -> TokenStream {
    let base = match sr {
        ObjectOrReference::Ref { ref_path, .. } => {
            let n = ref_path
                .split('/')
                .next_back()
                .unwrap_or("Unknown")
                .to_upper_camel_case();
            let id = Ident::new(&n, Span::call_site());
            quote! {#id}
        }
        ObjectOrReference::Object(s) => match crate::oas::schema_type(s) {
            Some(SchemaType::String) if !s.enum_values.is_empty() && parent.is_some() => {
                let (p, f) = parent.unwrap();
                let id = Ident::new(&nested_name(p, f, ""), Span::call_site());
                quote! {#id}
            }
            Some(SchemaType::String) => match string_numeric(s) {
                Some(v) => numeric_type(v),
                None => match s.format.as_deref() {
                    Some("date-time") => quote! {crate::datetime::DateTime},
                    Some("date") => quote! {crate::datetime::Date},
                    Some("password") => quote! {crate::secret::Secret},
                    Some("byte" | "binary") => quote! {Vec<u8>},
                    _ => quote! {String},
                },
            },
            Some(SchemaType::Number) => {
                if s.format.as_deref() == Some("float") {
                    quote! {f32}
                } else {
                    quote! {f64}
                }
            }
            Some(SchemaType::Integer) => {
                if s.format.as_deref() == Some("int32") {
                    quote! {i32}
                } else {
                    quote! {i64}
                }
            }
            Some(SchemaType::Boolean) => quote! {bool},
            Some(SchemaType::Array) => {
                let item = match s.items.as_deref() {
                    Some(Schema::Object(v)) => array_item(v, parent),
                    _ => quote! {serde_json::Value},
                };
                quote! {Vec<#item>}
            }
            Some(SchemaType::Object) | None if !s.all_of.is_empty() || !s.properties.is_empty() => {
                parent
                    .map(|(p, f)| {
                        let id = Ident::new(&nested_name(p, f, ""), Span::call_site());
                        quote! {#id}
                    })
                    .unwrap_or_else(|| quote! {serde_json::Value})
            }
            Some(SchemaType::Object) | None | Some(SchemaType::Null) => quote! {serde_json::Value},
        },
    };
    if required {
        base
    } else if nullable {
        quote! {Option<crate::Nullable<#base>>}
    } else {
        quote! {Option<#base>}
    }
}
fn array_item(sr: &ObjectOrReference<ObjectSchema>, parent: Option<(&str, &str)>) -> TokenStream {
    match sr {
        ObjectOrReference::Ref { ref_path, .. } => {
            let n = ref_path
                .split('/')
                .next_back()
                .unwrap_or("Unknown")
                .to_upper_camel_case();
            let id = Ident::new(&n, Span::call_site());
            quote! {#id}
        }
        ObjectOrReference::Object(s) => {
            if crate::oas::schema_type(s) == Some(SchemaType::Object)
                && s.properties.is_empty()
                && s.additional_properties.is_none()
            {
                return quote! {serde_json::Value};
            }
            if let Some((p, f)) = parent.filter(|_| {
                !s.properties.is_empty() || !s.all_of.is_empty() || !s.enum_values.is_empty()
            }) {
                let id = Ident::new(&nested_name(p, f, "Item"), Span::call_site());
                return quote! {#id};
            }
            infer_rust_type(true, false, None, sr)
        }
    }
}

pub fn sanitize_enum_variant(v: &str) -> String {
    use heck::ToPascalCase;
    if matches!(
        v,
        "EUR"
            | "USD"
            | "GBP"
            | "CHF"
            | "JPY"
            | "CAD"
            | "COP"
            | "AUD"
            | "NZD"
            | "SEK"
            | "NOK"
            | "DKK"
            | "PLN"
            | "CZK"
            | "HUF"
            | "RON"
            | "BGN"
            | "HRK"
            | "BRL"
            | "CLP"
            | "GET"
            | "POST"
            | "PUT"
            | "DELETE"
            | "PATCH"
            | "HEAD"
            | "OPTIONS"
            | "API"
            | "SDK"
            | "HTTP"
            | "HTTPS"
            | "URL"
            | "URI"
            | "JSON"
            | "XML"
            | "HTML"
            | "CSS"
            | "SQL"
            | "TCP"
            | "UDP"
            | "DNS"
            | "SSL"
            | "TLS"
            | "JWT"
            | "UUID"
            | "ID"
    ) {
        return v.into();
    }
    let out = v.replace(['-', '.', ':', '/'], "_").to_pascal_case();
    if out.chars().next().is_some_and(|c| c.is_numeric()) {
        format!("_{out}")
    } else {
        out
    }
}
fn nested_name(p: &str, f: &str, s: &str) -> String {
    format!(
        "{}{}{}",
        p.to_upper_camel_case(),
        f.to_upper_camel_case(),
        s
    )
}
fn string_enum(
    id: &Ident,
    values: &[serde_json::Value],
    description: Option<&str>,
    s: &ObjectSchema,
) -> Result<TokenStream, String> {
    let mut names = HashSet::new();
    let mut variants = Vec::new();
    for value in values.iter().filter_map(serde_json::Value::as_str) {
        let n = sanitize_enum_variant(value);
        if !names.insert(n.clone()) {
            return Err(format!(
                "Duplicate enum variant name generated for inline enum type: {n}"
            ));
        }
        let v = Ident::new(&n, Span::call_site());
        variants.push(if value == n {
            quote! {#v}
        } else {
            quote! {#[serde(rename=#value)]#v}
        })
    }
    if variants.is_empty() {
        return Ok(quote! {pub type #id=String;});
    }
    let other = Ident::new(
        if names.contains("Other") {
            "OtherValue"
        } else {
            "Other"
        },
        Span::call_site(),
    );
    let doc = generate_schema_doc_comment(description, s);
    Ok(
        quote! {#doc #[derive(Debug,Clone,PartialEq,Eq,serde::Serialize,serde::Deserialize)]pub enum #id{#(#variants,)*#[serde(untagged)]#other(String),}},
    )
}
fn error_impl(id: &Ident, properties: &Properties, required: &[String]) -> TokenStream {
    let is_required = |name: &str| required.iter().any(|field| field == name);
    let display = if id == "Problem" {
        quote! {
            match (&self.title, &self.detail) {
                (Some(title), Some(detail)) => write!(f, "{}: {}", title, detail),
                (Some(title), None) => write!(f, "{}", title),
                (None, Some(detail)) => write!(f, "{}", detail),
                (None, None) => write!(f, "{:?}", self),
            }
        }
    } else if properties.contains_key("message") {
        if is_required("message") {
            quote! { write!(f, "{}", self.message) }
        } else {
            quote! { match self.message.as_deref() { Some(value) => write!(f, "{}", value), None => write!(f, "{:?}", self) } }
        }
    } else if properties.contains_key("error") {
        if is_required("error") {
            quote! { write!(f, "{}", self.error) }
        } else {
            quote! { match self.error.as_deref() { Some(value) => write!(f, "{}", value), None => write!(f, "{:?}", self) } }
        }
    } else {
        quote! { write!(f, "{:?}", self) }
    };
    quote! {
        impl std::fmt::Display for #id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { #display }
        }
        impl std::error::Error for #id {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_openapi_31_nullable_type_union() {
        let schema: ObjectSchema = serde_json::from_value(serde_json::json!({
            "type": ["string", "null"]
        }))
        .expect("parse schema");
        let schema_ref = ObjectOrReference::Object(schema);

        assert_eq!(
            infer_rust_type(false, true, None, &schema_ref).to_string(),
            "Option < crate :: Nullable < String >>"
        );
    }

    #[test]
    fn uses_json_schema_examples_keyword() {
        let schema: ObjectSchema = serde_json::from_value(serde_json::json!({
            "type": "string",
            "examples": ["example value"]
        }))
        .expect("parse schema");

        assert_eq!(
            crate::oas::schema_example(&schema),
            Some(&serde_json::json!("example value"))
        );
    }
}
