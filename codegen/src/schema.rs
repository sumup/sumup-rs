use heck::ToUpperCamelCase;
use openapiv3::OpenAPI;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

/// Generate documentation attributes from a description string.
/// Splits multi-line descriptions into separate doc attributes for better formatting.
pub fn generate_doc_comment(description: &str) -> TokenStream {
    let lines: Vec<&str> = description.lines().collect();

    if lines.is_empty() {
        return quote! {};
    }

    // Generate a separate #[doc = " line"] for each line (note the leading space)
    let doc_attrs: Vec<_> = lines
        .iter()
        .map(|line| {
            let trimmed = line.trim();
            let doc_line = if trimmed.is_empty() {
                // For empty lines, just use an empty doc comment
                String::new()
            } else {
                // Add a space before the content
                format!(" {}", trimmed)
            };
            quote! { #[doc = #doc_line] }
        })
        .collect();

    quote! { #(#doc_attrs)* }
}

pub fn generate_structs_for_schemas(
    spec: &OpenAPI,
    schema_names: &HashSet<String>,
    error_schema_names: &HashSet<String>,
) -> Result<TokenStream, String> {
    let mut items = Vec::new();
    let mut nested_schemas = Vec::new();

    let schemas = match &spec.components {
        Some(components) => &components.schemas,
        None => return Ok(quote! {}),
    };

    // Sort schemas alphabetically for deterministic output
    let mut sorted_names: Vec<_> = schema_names.iter().collect();
    sorted_names.sort();

    for name in sorted_names {
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
                collect_nested_schemas(name, &obj.properties, &mut nested_schemas);

                let fields = generate_struct_fields(name, &obj.properties, &obj.required);

                let can_derive_default = can_fields_derive_default(&obj.properties, &obj.required);

                let description = schema
                    .schema_data
                    .description
                    .as_ref()
                    .map(|d| generate_doc_comment(d));

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
            openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
                if !s.enumeration.is_empty() {
                    let variants_tokens: Vec<TokenStream> = s
                        .enumeration
                        .iter()
                        .filter_map(|v| v.as_ref())
                        .map(|variant| {
                            let variant_name = sanitize_enum_variant(variant);
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
                        let description = schema
                            .schema_data
                            .description
                            .as_ref()
                            .map(|d| generate_doc_comment(d));

                        let deprecation = generate_deprecation_attribute(&schema.schema_data);

                        items.push(quote! {
                            #description
                            #deprecation
                            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                            pub enum #struct_name {
                                #(#variants_tokens,)*
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
                let base_type = infer_rust_type(&schema.schema_kind, true, None);
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

pub fn collect_nested_schemas_public(
    parent_name: &str,
    properties: &indexmap::IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>,
    nested_schemas: &mut Vec<TokenStream>,
) {
    collect_nested_schemas(parent_name, properties, nested_schemas);
}

fn collect_nested_schemas(
    parent_name: &str,
    properties: &indexmap::IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>,
    nested_schemas: &mut Vec<TokenStream>,
) {
    for (field_name, prop_ref) in properties {
        if let openapiv3::ReferenceOr::Item(schema) = prop_ref {
            match &schema.schema_kind {
                openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) => {
                    // Generate nested struct
                    let nested_struct_name = format!(
                        "{}{}",
                        parent_name.to_upper_camel_case(),
                        field_name.to_upper_camel_case()
                    );
                    let struct_ident = Ident::new(&nested_struct_name, Span::call_site());

                    // Recursively collect nested schemas
                    collect_nested_schemas(&nested_struct_name, &obj.properties, nested_schemas);

                    let fields =
                        generate_struct_fields(&nested_struct_name, &obj.properties, &obj.required);
                    let can_derive_default =
                        can_fields_derive_default(&obj.properties, &obj.required);

                    let description = schema
                        .schema_data
                        .description
                        .as_ref()
                        .map(|d| generate_doc_comment(d));

                    let derives = if can_derive_default {
                        quote! { #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)] }
                    } else {
                        quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
                    };

                    nested_schemas.push(quote! {
                        #description
                        #derives
                        pub struct #struct_ident {
                            #(#fields)*
                        }
                    });
                }
                openapiv3::SchemaKind::Type(openapiv3::Type::Array(arr)) => {
                    if let Some(openapiv3::ReferenceOr::Item(item_schema)) = &arr.items {
                        if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) =
                            &item_schema.schema_kind
                        {
                            // Generate nested struct for array items
                            let nested_struct_name = format!(
                                "{}{}Item",
                                parent_name.to_upper_camel_case(),
                                field_name.to_upper_camel_case()
                            );
                            let struct_ident = Ident::new(&nested_struct_name, Span::call_site());

                            collect_nested_schemas(
                                &nested_struct_name,
                                &obj.properties,
                                nested_schemas,
                            );

                            let fields = generate_struct_fields(
                                &nested_struct_name,
                                &obj.properties,
                                &obj.required,
                            );
                            let can_derive_default =
                                can_fields_derive_default(&obj.properties, &obj.required);

                            let description = item_schema
                                .schema_data
                                .description
                                .as_ref()
                                .map(|d| generate_doc_comment(d));

                            let derives = if can_derive_default {
                                quote! { #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)] }
                            } else {
                                quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
                            };

                            nested_schemas.push(quote! {
                                #description
                                #derives
                                pub struct #struct_ident {
                                    #(#fields)*
                                }
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn can_fields_derive_default(
    properties: &indexmap::IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>,
    required: &[String],
) -> bool {
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
                | openapiv3::SchemaKind::Type(openapiv3::Type::Boolean(_)) => return false,
                _ => {}
            }
        }
    }
    true
}

pub fn generate_struct_fields(
    parent_name: &str,
    properties: &indexmap::IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>,
    required: &[String],
) -> Vec<TokenStream> {
    properties
        .iter()
        .map(|(name, prop_ref)| {
            let sanitized_name = match name.as_str() {
                "type" => "type_",
                "ref" => "ref_",
                "match" => "match_",
                "move" => "move_",
                other => other,
            };
            let field_name = Ident::new(
                &sanitized_name.to_lowercase().replace('-', "_"),
                Span::call_site(),
            );
            let is_required = required.contains(name);

            let prop = match prop_ref {
                openapiv3::ReferenceOr::Item(p) => p,
                openapiv3::ReferenceOr::Reference { reference } => {
                    let type_name = reference.split('/').next_back().unwrap_or("Unknown");
                    let type_ident =
                        Ident::new(&type_name.to_upper_camel_case(), Span::call_site());
                    let field_type = if is_required {
                        quote! { #type_ident }
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

            let rust_type =
                infer_rust_type(&prop.schema_kind, is_required, Some((parent_name, name)));

            let description = prop
                .schema_data
                .description
                .as_ref()
                .map(|d| generate_doc_comment(d));

            let deprecation = generate_deprecation_attribute(&prop.schema_data);

            let skip_if = if !is_required {
                quote! { #[serde(skip_serializing_if = "Option::is_none")] }
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
                #skip_if
                pub #field_name: #rust_type,
            }
        })
        .collect()
}

pub fn infer_rust_type(
    schema_kind: &openapiv3::SchemaKind,
    required: bool,
    parent_field: Option<(&str, &str)>,
) -> TokenStream {
    let base_type = match schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(string_type)) => {
            match &string_type.format {
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => {
                    quote! { crate::datetime::DateTime }
                }
                _ => quote! { String },
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(_)) => quote! { f64 },
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_)) => quote! { i64 },
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
                    openapiv3::ReferenceOr::Item(schema) => {
                        match &schema.schema_kind {
                            openapiv3::SchemaKind::Type(openapiv3::Type::Object(_)) => {
                                // Use nested struct name
                                if let Some((parent_name, field_name)) = parent_field {
                                    let nested_type = format!(
                                        "{}{}Item",
                                        parent_name.to_upper_camel_case(),
                                        field_name.to_upper_camel_case()
                                    );
                                    let type_ident = Ident::new(&nested_type, Span::call_site());
                                    quote! { #type_ident }
                                } else {
                                    quote! { serde_json::Value }
                                }
                            }
                            _ => infer_rust_type(&schema.schema_kind, true, None),
                        }
                    }
                };
                quote! { Vec<#item_type> }
            } else {
                quote! { Vec<serde_json::Value> }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(_)) => {
            // Use nested struct name
            if let Some((parent_name, field_name)) = parent_field {
                let nested_type = format!(
                    "{}{}",
                    parent_name.to_upper_camel_case(),
                    field_name.to_upper_camel_case()
                );
                let type_ident = Ident::new(&nested_type, Span::call_site());
                quote! { #type_ident }
            } else {
                quote! { serde_json::Value }
            }
        }
        _ => quote! { serde_json::Value },
    };

    if required {
        base_type
    } else {
        quote! { Option<#base_type> }
    }
}

fn sanitize_enum_variant(variant: &str) -> String {
    use heck::ToPascalCase;

    let sanitized = variant.replace(['-', '.', ':', '/'], "_");

    let pascal = sanitized.to_pascal_case();

    if pascal.chars().next().is_some_and(|c| c.is_numeric()) {
        format!("_{}", pascal)
    } else {
        pascal
    }
}

fn generate_error_impl(
    struct_name: &Ident,
    properties: &indexmap::IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>,
    required: &[String],
) -> TokenStream {
    // Helper function to check if a field is required
    let is_required = |field_name: &str| -> bool { required.contains(&field_name.to_string()) };

    // Try to find common error message fields
    let display_impl = if properties.contains_key("message") {
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
