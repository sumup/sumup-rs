use heck::ToUpperCamelCase;
use openapiv3::OpenAPI;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

type Properties = indexmap::IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>;
type FlattenedObject = Option<(Properties, Vec<String>)>;

/// Generates documentation attributes from a description string.
/// Splits multi-line descriptions into separate doc attributes for better formatting.
pub fn generate_doc_comment(description: &str) -> TokenStream {
    let lines: Vec<String> = description
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    generate_doc_comment_from_lines(lines)
}

/// Generates documentation attributes from description + schema constraints.
pub fn generate_schema_doc_comment(
    description: Option<&str>,
    schema: &openapiv3::Schema,
) -> TokenStream {
    let mut lines: Vec<String> = Vec::new();

    if let Some(description) = description {
        lines.extend(description.lines().map(|line| line.trim().to_string()));
    }

    let constraints = collect_schema_constraints(schema);
    if !constraints.is_empty() {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.push("Constraints:".to_string());
        lines.extend(constraints.into_iter().map(|line| format!("- {}", line)));
    }

    generate_doc_comment_from_lines(lines)
}

fn generate_doc_comment_from_lines(lines: Vec<String>) -> TokenStream {
    if lines.is_empty() {
        return quote! {};
    }

    let doc_attrs: Vec<_> = lines
        .into_iter()
        .map(|line| {
            let doc_line = if line.is_empty() {
                String::new()
            } else {
                format!(" {}", line)
            };
            quote! { #[doc = #doc_line] }
        })
        .collect();

    quote! { #(#doc_attrs)* }
}

fn collect_schema_constraints(schema: &openapiv3::Schema) -> Vec<String> {
    use openapiv3::VariantOrUnknownOrEmpty;

    let mut constraints = Vec::new();

    if schema.schema_data.read_only {
        constraints.push("read-only".to_string());
    }
    if schema.schema_data.write_only {
        constraints.push("write-only".to_string());
    }
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(string_type)) => {
            match &string_type.format {
                VariantOrUnknownOrEmpty::Item(format) => {
                    if !is_rust_mapped_string_format(format) {
                        constraints.push(format!(
                            "format: `{}`",
                            normalize_format_name(&format!("{:?}", format))
                        ));
                    }
                }
                VariantOrUnknownOrEmpty::Unknown(format) => {
                    constraints.push(format!("format: `{}`", format));
                }
                VariantOrUnknownOrEmpty::Empty => {}
            }

            if let Some(pattern) = &string_type.pattern {
                constraints.push(format!("pattern: `{}`", pattern));
            }
            if let Some(min_length) = string_type.min_length {
                constraints.push(format!("min length: {}", min_length));
            }
            if let Some(max_length) = string_type.max_length {
                constraints.push(format!("max length: {}", max_length));
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(number_type)) => {
            match &number_type.format {
                VariantOrUnknownOrEmpty::Item(format) => {
                    if !is_rust_mapped_number_format(format) {
                        constraints.push(format!(
                            "format: `{}`",
                            normalize_format_name(&format!("{:?}", format))
                        ));
                    }
                }
                VariantOrUnknownOrEmpty::Unknown(format) => {
                    constraints.push(format!("format: `{}`", format));
                }
                VariantOrUnknownOrEmpty::Empty => {}
            }

            if let Some(multiple_of) = number_type.multiple_of {
                constraints.push(format!("multiple of: {}", multiple_of));
            }
            if let Some(minimum) = number_type.minimum {
                if number_type.exclusive_minimum {
                    constraints.push(format!("value > {}", minimum));
                } else {
                    constraints.push(format!("value >= {}", minimum));
                }
            }
            if let Some(maximum) = number_type.maximum {
                if number_type.exclusive_maximum {
                    constraints.push(format!("value < {}", maximum));
                } else {
                    constraints.push(format!("value <= {}", maximum));
                }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(integer_type)) => {
            match &integer_type.format {
                VariantOrUnknownOrEmpty::Item(format) => {
                    if !is_rust_mapped_integer_format(format) {
                        constraints.push(format!(
                            "format: `{}`",
                            normalize_format_name(&format!("{:?}", format))
                        ));
                    }
                }
                VariantOrUnknownOrEmpty::Unknown(format) => {
                    constraints.push(format!("format: `{}`", format));
                }
                VariantOrUnknownOrEmpty::Empty => {}
            }

            if let Some(multiple_of) = integer_type.multiple_of {
                constraints.push(format!("multiple of: {}", multiple_of));
            }
            if let Some(minimum) = integer_type.minimum {
                if integer_type.exclusive_minimum {
                    constraints.push(format!("value > {}", minimum));
                } else {
                    constraints.push(format!("value >= {}", minimum));
                }
            }
            if let Some(maximum) = integer_type.maximum {
                if integer_type.exclusive_maximum {
                    constraints.push(format!("value < {}", maximum));
                } else {
                    constraints.push(format!("value <= {}", maximum));
                }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(array_type)) => {
            if let Some(min_items) = array_type.min_items {
                constraints.push(format!("min items: {}", min_items));
            }
            if let Some(max_items) = array_type.max_items {
                constraints.push(format!("max items: {}", max_items));
            }
            if array_type.unique_items {
                constraints.push("items must be unique".to_string());
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(object_type)) => {
            if let Some(min_properties) = object_type.min_properties {
                constraints.push(format!("min properties: {}", min_properties));
            }
            if let Some(max_properties) = object_type.max_properties {
                constraints.push(format!("max properties: {}", max_properties));
            }
        }
        _ => {}
    }

    constraints
}

fn normalize_format_name(raw: &str) -> String {
    let mut normalized = String::with_capacity(raw.len());
    let mut prev_is_lower_or_digit = false;

    for ch in raw.chars() {
        if ch.is_ascii_uppercase() {
            if prev_is_lower_or_digit {
                normalized.push('-');
            }
            normalized.push(ch.to_ascii_lowercase());
            prev_is_lower_or_digit = false;
        } else {
            normalized.push(ch);
            prev_is_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        }
    }

    normalized
}

/// Converts a generated field name into a valid Rust identifier, using raw identifiers for
/// reserved keywords when possible.
pub fn make_rust_field_ident(name: &str) -> Ident {
    let normalized = name.to_lowercase().replace('-', "_");
    if is_rust_keyword(&normalized) {
        return Ident::new_raw(&normalized, Span::call_site());
    }

    Ident::new(&normalized, Span::call_site())
}

fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
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

fn is_rust_mapped_string_format(format: &openapiv3::StringFormat) -> bool {
    matches!(
        format,
        openapiv3::StringFormat::DateTime
            | openapiv3::StringFormat::Date
            | openapiv3::StringFormat::Password
            | openapiv3::StringFormat::Byte
            | openapiv3::StringFormat::Binary
    )
}

fn is_rust_mapped_number_format(format: &openapiv3::NumberFormat) -> bool {
    matches!(
        format,
        openapiv3::NumberFormat::Float | openapiv3::NumberFormat::Double
    )
}

fn is_rust_mapped_integer_format(format: &openapiv3::IntegerFormat) -> bool {
    matches!(
        format,
        openapiv3::IntegerFormat::Int32 | openapiv3::IntegerFormat::Int64
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringEncodedNumericKind {
    F32,
    F64,
    I32,
    I64,
}

fn string_encoded_numeric_kind(
    format: &openapiv3::VariantOrUnknownOrEmpty<openapiv3::StringFormat>,
) -> Option<StringEncodedNumericKind> {
    let openapiv3::VariantOrUnknownOrEmpty::Unknown(raw) = format else {
        return None;
    };

    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "float" => Some(StringEncodedNumericKind::F32),
        "double" | "number" => Some(StringEncodedNumericKind::F64),
        "int32" | "integer" => Some(StringEncodedNumericKind::I32),
        "int64" => Some(StringEncodedNumericKind::I64),
        _ => None,
    }
}

fn string_schema_numeric_kind(
    schema_kind: &openapiv3::SchemaKind,
) -> Option<StringEncodedNumericKind> {
    let openapiv3::SchemaKind::Type(openapiv3::Type::String(string_type)) = schema_kind else {
        return None;
    };
    string_encoded_numeric_kind(&string_type.format)
}

fn numeric_kind_rust_type(kind: StringEncodedNumericKind) -> TokenStream {
    match kind {
        StringEncodedNumericKind::F32 => quote! { f32 },
        StringEncodedNumericKind::F64 => quote! { f64 },
        StringEncodedNumericKind::I32 => quote! { i32 },
        StringEncodedNumericKind::I64 => quote! { i64 },
    }
}

/// Generates struct definitions for the selected component schemas.
pub fn generate_structs_for_schemas(
    spec: &OpenAPI,
    schema_names: &HashSet<String>,
    error_schema_names: &HashSet<String>,
) -> Result<TokenStream, String> {
    let mut items = Vec::new();
    let mut nested_schemas = Vec::new();

    let components = match &spec.components {
        Some(components) => components,
        None => return Ok(quote! {}),
    };

    let schemas = &components.schemas;

    let skip_all_of_refs = collect_mixin_all_of_references(spec, schema_names);

    // Sort schemas alphabetically for deterministic output
    let mut sorted_names: Vec<_> = schema_names.iter().collect();
    sorted_names.sort();

    for name in sorted_names {
        if skip_all_of_refs.contains(name.as_str()) {
            continue;
        }

        let schema_ref = match schemas.get(name) {
            Some(s) => s,
            None => continue,
        };

        let struct_name = Ident::new(&name.to_upper_camel_case(), Span::call_site());
        let is_error_schema = error_schema_names.contains(name);

        let schema = match schema_ref {
            openapiv3::ReferenceOr::Item(s) => s,
            openapiv3::ReferenceOr::Reference { .. } => continue,
        };

        match &schema.schema_kind {
            openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) => {
                // Collect nested inline schemas
                collect_nested_schemas(spec, name, &obj.properties, &mut nested_schemas)?;

                let fields = generate_struct_fields(name, &obj.properties, &obj.required);

                let can_derive_default = can_fields_derive_default(&obj.properties, &obj.required);

                let description = schema
                    .schema_data
                    .description
                    .as_ref()
                    .map(|d| generate_schema_doc_comment(Some(d), schema));

                let deprecation = generate_deprecation_attribute(&schema.schema_data);

                let derives = if can_derive_default {
                    quote! { #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)] }
                } else {
                    quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
                };

                let struct_def = quote! {
                    #description
                    #deprecation
                    #derives
                    pub struct #struct_name {
                        #(#fields)*
                    }
                };

                items.push(struct_def);

                // If this is an error schema, implement Error trait
                if is_error_schema {
                    let error_impl =
                        generate_error_impl(&struct_name, &obj.properties, &obj.required);
                    items.push(error_impl);
                }
            }
            openapiv3::SchemaKind::AllOf { all_of } => {
                if let Some((combined_properties, combined_required)) =
                    flatten_all_of_object(spec, all_of)?
                {
                    collect_nested_schemas(spec, name, &combined_properties, &mut nested_schemas)?;

                    let fields =
                        generate_struct_fields(name, &combined_properties, &combined_required);

                    let can_derive_default =
                        can_fields_derive_default(&combined_properties, &combined_required);

                    let description = schema
                        .schema_data
                        .description
                        .as_ref()
                        .map(|d| generate_schema_doc_comment(Some(d), schema));

                    let deprecation = generate_deprecation_attribute(&schema.schema_data);

                    let derives = if can_derive_default {
                        quote! { #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)] }
                    } else {
                        quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
                    };

                    let struct_def = quote! {
                        #description
                        #deprecation
                        #derives
                        pub struct #struct_name {
                            #(#fields)*
                        }
                    };

                    items.push(struct_def);

                    if is_error_schema {
                        let error_impl = generate_error_impl(
                            &struct_name,
                            &combined_properties,
                            &combined_required,
                        );
                        items.push(error_impl);
                    }
                } else {
                    let dummy_ref = openapiv3::ReferenceOr::Item(Box::new(schema.clone()));
                    let base_type =
                        infer_rust_type(&schema.schema_kind, true, false, None, &dummy_ref);
                    items.push(quote! {
                        pub type #struct_name = #base_type;
                    });
                }
            }
            openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
                if !s.enumeration.is_empty() {
                    let mut variant_names: HashSet<String> = HashSet::new();
                    let variants_tokens: Vec<TokenStream> = s
                        .enumeration
                        .iter()
                        .filter_map(|v| v.as_ref())
                        .map(|variant| {
                            let variant_name = sanitize_enum_variant(variant);
                            variant_names.insert(variant_name.clone());
                            let variant_ident = Ident::new(&variant_name, Span::call_site());
                            if variant != &variant_name {
                                quote! {
                                    #[serde(rename = #variant)]
                                    #variant_ident
                                }
                            } else {
                                quote! { #variant_ident }
                            }
                        })
                        .collect();

                    if !variants_tokens.is_empty() {
                        let other_variant_ident = if variant_names.contains("Other") {
                            Ident::new("OtherValue", Span::call_site())
                        } else {
                            Ident::new("Other", Span::call_site())
                        };

                        let description = schema
                            .schema_data
                            .description
                            .as_ref()
                            .map(|d| generate_schema_doc_comment(Some(d), schema));

                        let deprecation = generate_deprecation_attribute(&schema.schema_data);

                        items.push(quote! {
                            #description
                            #deprecation
                            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                            pub enum #struct_name {
                                #(#variants_tokens,)*
                                #[serde(untagged)]
                                #other_variant_ident(String),
                            }
                        });
                    } else {
                        items.push(quote! {
                            pub type #struct_name = String;
                        });
                    }
                } else {
                    items.push(quote! {
                        pub type #struct_name = String;
                    });
                }
            }
            _ => {
                let dummy_ref = openapiv3::ReferenceOr::Item(Box::new(schema.clone()));
                let base_type = infer_rust_type(&schema.schema_kind, true, false, None, &dummy_ref);
                items.push(quote! {
                    pub type #struct_name = #base_type;
                });
            }
        }
    }

    // Add nested schemas
    items.extend(nested_schemas);

    Ok(quote! {
        #(#items)*
    })
}

/// Produces a `#[deprecated]` attribute when the schema marks itself as deprecated.
fn generate_deprecation_attribute(schema_data: &openapiv3::SchemaData) -> TokenStream {
    if !schema_data.deprecated {
        return quote! {};
    }

    if let Some(notice) = schema_data.extensions.get("x-deprecation-notice") {
        if let Some(notice_str) = notice.as_str() {
            let message = notice_str.trim();
            return quote! {
                #[deprecated(note = #message)]
            };
        }
    }

    quote! {
        #[deprecated]
    }
}

struct NestedStructGenerator<'spec, 'schemas> {
    spec: &'spec OpenAPI,
    nested_schemas: &'schemas mut Vec<TokenStream>,
}

impl<'spec, 'schemas> NestedStructGenerator<'spec, 'schemas> {
    /// Creates a helper that appends generated nested structs to the shared buffer.
    fn new(spec: &'spec OpenAPI, nested_schemas: &'schemas mut Vec<TokenStream>) -> Self {
        Self {
            spec,
            nested_schemas,
        }
    }

    /// Generates a nested struct for an inline object schema.
    fn generate_for_object(
        &mut self,
        parent_name: &str,
        field_name: &str,
        schema: &openapiv3::Schema,
        obj: &openapiv3::ObjectType,
    ) -> Result<(), String> {
        self.generate_for_object_like(
            parent_name,
            field_name,
            schema,
            &obj.properties,
            &obj.required,
            "",
        )
    }

    /// Generates a nested struct for an object-like schema, including `allOf` composites.
    fn generate_for_object_like(
        &mut self,
        parent_name: &str,
        field_name: &str,
        schema: &openapiv3::Schema,
        properties: &Properties,
        required: &[String],
        fallback_suffix: &str,
    ) -> Result<(), String> {
        let nested_struct_name = schema
            .schema_data
            .title
            .as_ref()
            .map(|t| t.to_upper_camel_case())
            .unwrap_or_else(|| {
                format!(
                    "{}{}{}",
                    parent_name.to_upper_camel_case(),
                    field_name.to_upper_camel_case(),
                    fallback_suffix
                )
            });
        let struct_ident = Ident::new(&nested_struct_name, Span::call_site());

        collect_nested_schemas(
            self.spec,
            &nested_struct_name,
            properties,
            &mut *self.nested_schemas,
        )?;

        let fields = generate_struct_fields(&nested_struct_name, properties, required);
        let can_derive_default = can_fields_derive_default(properties, required);

        let description = schema
            .schema_data
            .description
            .as_ref()
            .map(|d| generate_schema_doc_comment(Some(d), schema));

        let derives = if can_derive_default {
            quote! { #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)] }
        } else {
            quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
        };

        self.nested_schemas.push(quote! {
            #description
            #derives
            pub struct #struct_ident {
                #(#fields)*
            }
        });

        Ok(())
    }
}

fn collect_mixin_all_of_references(
    spec: &OpenAPI,
    schema_names: &HashSet<String>,
) -> HashSet<String> {
    let mut referenced = HashSet::new();

    if let Some(components) = &spec.components {
        for schema_ref in components.schemas.values() {
            if let openapiv3::ReferenceOr::Item(schema) = schema_ref {
                collect_all_of_references_from_schema(schema, schema_names, &mut referenced);
            }
        }
    }

    referenced
}

fn collect_all_of_references_from_schema(
    schema: &openapiv3::Schema,
    schema_names: &HashSet<String>,
    referenced: &mut HashSet<String>,
) {
    use openapiv3::ReferenceOr;

    if let openapiv3::SchemaKind::AllOf { all_of } = &schema.schema_kind {
        for entry in all_of {
            match entry {
                ReferenceOr::Reference { reference } => {
                    if let Some(name) = reference.strip_prefix("#/components/schemas/") {
                        if schema_names.contains(name) && name.contains("Mixin") {
                            referenced.insert(name.to_string());
                        }
                    }
                }
                ReferenceOr::Item(inner) => {
                    collect_all_of_references_from_schema(inner, schema_names, referenced);
                }
            }
        }
    }
}

/// Collects nested inline schemas for a parent type so callers can emit them later.
pub fn collect_nested_schemas(
    spec: &OpenAPI,
    parent_name: &str,
    properties: &Properties,
    nested_schemas: &mut Vec<TokenStream>,
) -> Result<(), String> {
    let mut generator = NestedStructGenerator::new(spec, nested_schemas);

    for (field_name, prop_ref) in properties {
        if let openapiv3::ReferenceOr::Item(schema) = prop_ref {
            match &schema.schema_kind {
                openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) => {
                    generator.generate_for_object(parent_name, field_name, schema, obj)?;
                }
                openapiv3::SchemaKind::AllOf { all_of } => {
                    if let Some((combined_properties, combined_required)) =
                        flatten_all_of_object(spec, all_of)?
                    {
                        generator.generate_for_object_like(
                            parent_name,
                            field_name,
                            schema,
                            &combined_properties,
                            &combined_required,
                            "",
                        )?;
                    }
                }
                openapiv3::SchemaKind::Type(openapiv3::Type::Array(arr)) => {
                    if let Some(openapiv3::ReferenceOr::Item(item_schema)) = &arr.items {
                        match &item_schema.schema_kind {
                            openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) => {
                                generator.generate_for_object_like(
                                    parent_name,
                                    field_name,
                                    item_schema,
                                    &obj.properties,
                                    &obj.required,
                                    "Item",
                                )?;
                            }
                            openapiv3::SchemaKind::AllOf { all_of } => {
                                if let Some((combined_properties, combined_required)) =
                                    flatten_all_of_object(spec, all_of)?
                                {
                                    generator.generate_for_object_like(
                                        parent_name,
                                        field_name,
                                        item_schema,
                                        &combined_properties,
                                        &combined_required,
                                        "Item",
                                    )?;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

/// Flattens an `allOf` object hierarchy into a single property map and required field list.
pub(crate) fn flatten_all_of_object(
    spec: &OpenAPI,
    all_of: &[openapiv3::ReferenceOr<openapiv3::Schema>],
) -> Result<FlattenedObject, String> {
    use openapiv3::{SchemaKind, Type};

    let mut combined_properties = Properties::new();
    let mut combined_required: Vec<String> = Vec::new();
    let mut found_object = false;

    for schema_ref in all_of {
        let schema = dereference_schema(spec, schema_ref)?;
        match &schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => {
                found_object = true;
                for (name, prop) in &obj.properties {
                    combined_properties.insert(name.clone(), prop.clone());
                }
                for req in &obj.required {
                    if !combined_required.contains(req) {
                        combined_required.push(req.clone());
                    }
                }
            }
            SchemaKind::AllOf { all_of: nested } => {
                if let Some((nested_properties, nested_required)) =
                    flatten_all_of_object(spec, nested)?
                {
                    found_object = true;
                    for (name, prop) in nested_properties {
                        combined_properties.insert(name, prop);
                    }
                    for req in nested_required {
                        if !combined_required.contains(&req) {
                            combined_required.push(req);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if found_object {
        Ok(Some((combined_properties, combined_required)))
    } else {
        Ok(None)
    }
}

/// Resolves a schema reference to the concrete schema definition within `components`.
fn dereference_schema<'a>(
    spec: &'a OpenAPI,
    schema_ref: &'a openapiv3::ReferenceOr<openapiv3::Schema>,
) -> Result<&'a openapiv3::Schema, String> {
    match schema_ref {
        openapiv3::ReferenceOr::Item(schema) => Ok(schema),
        openapiv3::ReferenceOr::Reference { reference } => {
            let schema_name = reference
                .strip_prefix("#/components/schemas/")
                .ok_or_else(|| format!("Unsupported schema reference: {}", reference))?;

            let components = spec
                .components
                .as_ref()
                .ok_or_else(|| "OpenAPI spec is missing components section".to_string())?;

            let target = components
                .schemas
                .get(schema_name)
                .ok_or_else(|| format!("Referenced schema '{}' not found", schema_name))?;

            dereference_schema(spec, target)
        }
    }
}

/// Checks whether all required fields support deriving `Default`.
pub fn can_fields_derive_default(properties: &Properties, required: &[String]) -> bool {
    for (name, prop_ref) in properties {
        if required.contains(name) {
            let prop = match prop_ref {
                openapiv3::ReferenceOr::Item(p) => p,
                openapiv3::ReferenceOr::Reference { .. } => return false,
            };

            match &prop.schema_kind {
                openapiv3::SchemaKind::Type(openapiv3::Type::String(_))
                | openapiv3::SchemaKind::Type(openapiv3::Type::Number(_))
                | openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_))
                | openapiv3::SchemaKind::Type(openapiv3::Type::Object(_))
                | openapiv3::SchemaKind::Type(openapiv3::Type::Boolean(_))
                | openapiv3::SchemaKind::AllOf { .. } => return false,
                _ => {}
            }
        }
    }
    true
}

/// Generates struct field declarations for the given property map.
pub fn generate_struct_fields(
    parent_name: &str,
    properties: &Properties,
    required: &[String],
) -> Vec<TokenStream> {
    properties
        .iter()
        .map(|(name, prop_ref)| {
            let field_name = make_rust_field_ident(name);
            let is_required = required.contains(name);

            let prop = match prop_ref {
                openapiv3::ReferenceOr::Item(p) => p,
                openapiv3::ReferenceOr::Reference { reference } => {
                    let type_name = reference.split('/').next_back().unwrap_or("Unknown");
                    let type_ident =
                        Ident::new(&type_name.to_upper_camel_case(), Span::call_site());
                    let is_nullable = false; // References don't carry nullable info directly
                    let field_type = if is_required {
                        quote! { #type_ident }
                    } else if is_nullable {
                        quote! { Option<crate::Nullable<#type_ident>> }
                    } else {
                        quote! { Option<#type_ident> }
                    };

                    let rename = if name != &field_name.to_string() {
                        quote! { #[serde(rename = #name)] }
                    } else {
                        quote! {}
                    };

                    let skip_if = if !is_required {
                        quote! { #[serde(skip_serializing_if = "Option::is_none")] }
                    } else {
                        quote! {}
                    };

                    return quote! {
                        #rename
                        #skip_if
                        pub #field_name: #field_type,
                    };
                }
            };

            let is_nullable = prop.schema_data.nullable;
            let uses_string_number_deserializer = string_schema_numeric_kind(&prop.schema_kind).is_some();
            let rust_type = infer_rust_type(
                &prop.schema_kind,
                is_required,
                is_nullable,
                Some((parent_name, name)),
                prop_ref,
            );

            let description = prop
                .schema_data
                .description
                .as_ref()
                .map(|d| generate_schema_doc_comment(Some(d), prop));

            let deprecation = generate_deprecation_attribute(&prop.schema_data);

            let serde_attrs = if !is_required {
                if is_nullable {
                    if uses_string_number_deserializer {
                        quote! {
                            #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::nullable::deserialize_string_or_number")]
                        }
                    } else {
                        quote! {
                            #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::nullable::deserialize")]
                        }
                    }
                } else if uses_string_number_deserializer {
                    quote! {
                        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::string_or_number::deserialize_option")]
                    }
                } else {
                    quote! {
                        #[serde(skip_serializing_if = "Option::is_none")]
                    }
                }
            } else if uses_string_number_deserializer {
                quote! {
                    #[serde(deserialize_with = "crate::string_or_number::deserialize")]
                }
            } else {
                quote! {}
            };

            let rename = if name != &field_name.to_string() {
                quote! { #[serde(rename = #name)] }
            } else {
                quote! {}
            };

            quote! {
                #description
                #deprecation
                #rename
                #serde_attrs
                pub #field_name: #rust_type,
            }
        })
        .collect()
}

/// Infers an appropriate Rust type for the provided schema kind.
pub fn infer_rust_type(
    schema_kind: &openapiv3::SchemaKind,
    required: bool,
    nullable: bool,
    parent_field: Option<(&str, &str)>,
    schema_ref: &openapiv3::ReferenceOr<Box<openapiv3::Schema>>,
) -> TokenStream {
    let base_type = match schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(string_type)) => {
            if let Some(kind) = string_encoded_numeric_kind(&string_type.format) {
                numeric_kind_rust_type(kind)
            } else {
                match &string_type.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => {
                        quote! { crate::datetime::DateTime }
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => {
                        quote! { crate::datetime::Date }
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Password) => {
                        quote! { crate::secret::Secret }
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Byte) => {
                        quote! { Vec<u8> }
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Binary) => {
                        quote! { Vec<u8> }
                    }
                    _ => quote! { String },
                }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(number_type)) => {
            match &number_type.format {
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Float) => {
                    quote! { f32 }
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) => {
                    quote! { f64 }
                }
                _ => quote! { f64 },
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(integer_type)) => {
            match &integer_type.format {
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int32) => {
                    quote! { i32 }
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => {
                    quote! { i64 }
                }
                _ => quote! { i64 },
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean(_)) => quote! { bool },
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(arr)) => {
            if let Some(items) = &arr.items {
                let item_type = match items {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        let type_name = reference.split('/').next_back().unwrap_or("Unknown");
                        let type_ident =
                            Ident::new(&type_name.to_upper_camel_case(), Span::call_site());
                        quote! { #type_ident }
                    }
                    openapiv3::ReferenceOr::Item(schema) => match &schema.schema_kind {
                        openapiv3::SchemaKind::Type(openapiv3::Type::Object(_)) => {
                            // Prefer title if available, otherwise use nested struct name
                            let nested_type = schema
                                .schema_data
                                .title
                                .as_ref()
                                .map(|t| t.to_upper_camel_case())
                                .or_else(|| {
                                    parent_field.map(|(parent_name, field_name)| {
                                        format!(
                                            "{}{}Item",
                                            parent_name.to_upper_camel_case(),
                                            field_name.to_upper_camel_case()
                                        )
                                    })
                                })
                                .unwrap_or_else(|| "serde_json::Value".to_string());

                            if nested_type == "serde_json::Value" {
                                quote! { serde_json::Value }
                            } else {
                                let type_ident = Ident::new(&nested_type, Span::call_site());
                                quote! { #type_ident }
                            }
                        }
                        openapiv3::SchemaKind::AllOf { .. } => {
                            let nested_type = schema
                                .schema_data
                                .title
                                .as_ref()
                                .map(|t| t.to_upper_camel_case())
                                .or_else(|| {
                                    parent_field.map(|(parent_name, field_name)| {
                                        format!(
                                            "{}{}Item",
                                            parent_name.to_upper_camel_case(),
                                            field_name.to_upper_camel_case()
                                        )
                                    })
                                })
                                .unwrap_or_else(|| "serde_json::Value".to_string());

                            if nested_type == "serde_json::Value" {
                                quote! { serde_json::Value }
                            } else {
                                let type_ident = Ident::new(&nested_type, Span::call_site());
                                quote! { #type_ident }
                            }
                        }
                        _ => {
                            // Create a dummy reference for recursive call
                            let dummy_ref = openapiv3::ReferenceOr::Item(schema.clone());
                            infer_rust_type(&schema.schema_kind, true, false, None, &dummy_ref)
                        }
                    },
                };
                quote! { Vec<#item_type> }
            } else {
                quote! { Vec<serde_json::Value> }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(_)) => {
            // Prefer title if available from the schema_ref
            let nested_type = if let openapiv3::ReferenceOr::Item(schema) = schema_ref {
                schema
                    .schema_data
                    .title
                    .as_ref()
                    .map(|t| t.to_upper_camel_case())
                    .or_else(|| {
                        parent_field.map(|(parent_name, field_name)| {
                            format!(
                                "{}{}",
                                parent_name.to_upper_camel_case(),
                                field_name.to_upper_camel_case()
                            )
                        })
                    })
            } else {
                parent_field.map(|(parent_name, field_name)| {
                    format!(
                        "{}{}",
                        parent_name.to_upper_camel_case(),
                        field_name.to_upper_camel_case()
                    )
                })
            };

            if let Some(type_name) = nested_type {
                let type_ident = Ident::new(&type_name, Span::call_site());
                quote! { #type_ident }
            } else {
                quote! { serde_json::Value }
            }
        }
        openapiv3::SchemaKind::AllOf { .. } => {
            let nested_type = if let openapiv3::ReferenceOr::Item(schema) = schema_ref {
                schema
                    .schema_data
                    .title
                    .as_ref()
                    .map(|t| t.to_upper_camel_case())
                    .or_else(|| {
                        parent_field.map(|(parent_name, field_name)| {
                            format!(
                                "{}{}",
                                parent_name.to_upper_camel_case(),
                                field_name.to_upper_camel_case()
                            )
                        })
                    })
            } else {
                parent_field.map(|(parent_name, field_name)| {
                    format!(
                        "{}{}",
                        parent_name.to_upper_camel_case(),
                        field_name.to_upper_camel_case()
                    )
                })
            };

            if let Some(type_name) = nested_type {
                let type_ident = Ident::new(&type_name, Span::call_site());
                quote! { #type_ident }
            } else {
                quote! { serde_json::Value }
            }
        }
        _ => quote! { serde_json::Value },
    };

    if required {
        base_type
    } else if nullable {
        quote! { Option<crate::Nullable<#base_type>> }
    } else {
        quote! { Option<#base_type> }
    }
}

/// Normalizes raw enum variant strings into valid Rust identifiers.
fn sanitize_enum_variant(variant: &str) -> String {
    use heck::ToPascalCase;

    // Handle well-known abbreviations that should remain uppercase
    match variant {
        // Currency codes (ISO 4217)
        "EUR" | "USD" | "GBP" | "CHF" | "JPY" | "CAD" | "AUD" | "NZD" | "SEK" | "NOK" | "DKK"
        | "PLN" | "CZK" | "HUF" | "RON" | "BGN" | "HRK" | "BRL" | "CLP" => {
            return variant.to_string()
        }

        // HTTP status codes and methods
        "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS" => {
            return variant.to_string()
        }

        // Common tech abbreviations
        "API" | "SDK" | "HTTP" | "HTTPS" | "URL" | "URI" | "JSON" | "XML" | "HTML" | "CSS"
        | "SQL" | "TCP" | "UDP" | "DNS" | "SSL" | "TLS" | "JWT" | "UUID" | "ID" => {
            return variant.to_string()
        }

        _ => {}
    }

    let sanitized = variant.replace(['-', '.', ':', '/'], "_");

    let pascal = sanitized.to_pascal_case();

    if pascal.chars().next().is_some_and(|c| c.is_numeric()) {
        format!("_{}", pascal)
    } else {
        pascal
    }
}

/// Generates a `std::error::Error` implementation for schemas marked as error types.
fn generate_error_impl(
    struct_name: &Ident,
    properties: &Properties,
    required: &[String],
) -> TokenStream {
    let struct_name_str = struct_name.to_string();

    // Helper function to check if a field is required
    let is_required = |field_name: &str| -> bool { required.contains(&field_name.to_string()) };

    // Try to find common error message fields
    let display_impl = if struct_name_str == "Problem" {
        // RFC 9457 problem responses expose `title` and `detail` fields.
        quote! {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match (&self.title, &self.detail) {
                    (Some(title), Some(detail)) => write!(f, "{}: {}", title, detail),
                    (Some(title), None) => write!(f, "{}", title),
                    (None, Some(detail)) => write!(f, "{}", detail),
                    (None, None) => write!(f, "{:?}", self),
                }
            }
        }
    } else if properties.contains_key("message") {
        if is_required("message") {
            quote! {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.message)
                }
            }
        } else {
            quote! {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if let Some(message) = &self.message {
                        write!(f, "{}", message)
                    } else {
                        write!(f, "{:?}", self)
                    }
                }
            }
        }
    } else if properties.contains_key("title") {
        if is_required("title") {
            quote! {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if let Some(details) = &self.details {
                        write!(f, "{}: {}", self.title, details)
                    } else {
                        write!(f, "{}", self.title)
                    }
                }
            }
        } else {
            quote! {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match (&self.title, &self.details) {
                        (Some(title), Some(details)) => write!(f, "{}: {}", title, details),
                        (Some(title), None) => write!(f, "{}", title),
                        (None, Some(details)) => write!(f, "{}", details),
                        (None, None) => write!(f, "{:?}", self),
                    }
                }
            }
        }
    } else if properties.contains_key("error_message") {
        if is_required("error_message") {
            quote! {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.error_message)
                }
            }
        } else {
            quote! {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if let Some(error_message) = &self.error_message {
                        write!(f, "{}", error_message)
                    } else {
                        write!(f, "{:?}", self)
                    }
                }
            }
        }
    } else {
        // Fallback to Debug formatting
        quote! {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };

    quote! {
        impl std::fmt::Display for #struct_name {
            #display_impl
        }

        impl std::error::Error for #struct_name {}
    }
}
