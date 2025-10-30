#![forbid(unsafe_code)]

use std::path::Path;

use heck::{ToSnakeCase, ToUpperCamelCase};
use openapiv3::OpenAPI;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub mod body;
pub mod client;
pub mod operation;
pub mod schema;
pub mod tag;

pub use body::generate_operation_bodies;
pub use client::generate_client_file;
pub use operation::generate_client_methods;
pub use schema::generate_structs_for_schemas;
pub use tag::{collect_schemas_by_tag, SchemasByTag, TagSchemas};

/// Generates `resources/mod.rs`, wiring up tag modules and common exports.
pub fn generate_mod_file(out_path: &Path, schemas_by_tag: &SchemasByTag) -> Result<(), String> {
    let tag_schemas = &schemas_by_tag.tag_schemas;
    let mut mod_path = out_path.to_path_buf();
    mod_path.push("resources");
    mod_path.push("mod.rs");

    let mut modules = Vec::new();
    let mut re_exports = Vec::new();

    // Add common module if there are common schemas
    if !schemas_by_tag.common_schemas.is_empty() {
        modules.push(quote! {
            pub mod common;
        });
        re_exports.push(quote! {
            pub use common::*;
        });
    }

    // Sort tags alphabetically for deterministic output
    let mut sorted_tags: Vec<_> = tag_schemas.keys().collect();
    sorted_tags.sort();

    // Add tag modules (schemas + clients)
    for tag in sorted_tags {
        let module_name = Ident::new(&tag.to_snake_case(), Span::call_site());

        modules.push(quote! {
            pub mod #module_name;
        });
        re_exports.push(quote! {
            pub use #module_name::*;
        });
    }

    let tokens = quote! {
        #(#modules)*

        #(#re_exports)*
    };

    let contents = format_generated_code(tokens);
    std::fs::write(&mod_path, &contents)
        .map_err(|e| format!("Failed to write resources/mod.rs: {}", e))?;

    Ok(())
}

/// Generates `resources/common.rs` containing schemas shared across multiple tags.
pub fn generate_common_file(
    out_path: &Path,
    spec: &OpenAPI,
    schemas_by_tag: &SchemasByTag,
) -> Result<(), String> {
    if schemas_by_tag.common_schemas.is_empty() {
        return Ok(());
    }

    let mut common_path = out_path.to_path_buf();
    common_path.push("resources");
    common_path.push("common.rs");

    let schema_tokens = generate_structs_for_schemas(
        spec,
        &schemas_by_tag.common_schemas,
        &schemas_by_tag.common_error_schemas,
    )?;

    let contents = format_generated_code(schema_tokens);
    std::fs::write(&common_path, &contents)
        .map_err(|e| format!("Failed to write resources/common.rs: {}", e))?;

    Ok(())
}

/// Formats generated tokens into Rust source and prepends the standard header.
pub fn format_generated_code(tokens: TokenStream) -> String {
    let header = "// The contents of this file are generated; do not modify them.\n\n";

    // First use prettyplease for basic formatting
    let file = syn::parse_file(&tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&file);

    let code_with_header = format!("{}{}\n", header, formatted);

    // Try to format with rustfmt for better results
    match format_with_rustfmt(&code_with_header) {
        Ok(rustfmt_output) => rustfmt_output,
        Err(_) => code_with_header, // Fall back to prettyplease output
    }
}

/// Runs `rustfmt` to polish already formatted source, falling back on failure.
fn format_with_rustfmt(code: &str) -> Result<String, std::io::Error> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("rustfmt")
        .arg("--edition=2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::other("rustfmt failed"))
    }
}

/// Builds the client struct and methods for a specific OpenAPI tag.
pub fn generate_tag_client(spec: &OpenAPI, tag: &str) -> Result<TokenStream, String> {
    let client_type = Ident::new(
        &format!("{}Client", tag.to_upper_camel_case()),
        Span::call_site(),
    );
    let doc_comment = format!("Client for the {} API endpoints.", tag);

    // Generate methods for operations with this tag
    let methods = generate_client_methods(spec, tag)?;

    Ok(quote! {
        use crate::client::Client;

        #[doc = #doc_comment]
        #[derive(Debug)]
        pub struct #client_type<'a> {
            client: &'a Client,
        }

        impl<'a> #client_type<'a> {
            pub(crate) fn new(client: &'a Client) -> Self {
                Self { client }
            }

            /// Returns a reference to the underlying client.
            pub fn client(&self) -> &Client {
                self.client
            }

            #methods
        }
    })
}

/// Checks whether any schema in the given set references a schema marked as common.
pub fn does_reference_common_schemas(
    spec: &OpenAPI,
    schemas: &std::collections::HashSet<String>,
    common_schemas: &std::collections::HashSet<String>,
) -> bool {
    let all_schemas = match &spec.components {
        Some(components) => &components.schemas,
        None => return false,
    };

    for schema_name in schemas {
        if let Some(schema_ref) = all_schemas.get(schema_name) {
            let schema = match schema_ref {
                openapiv3::ReferenceOr::Item(s) => s,
                openapiv3::ReferenceOr::Reference { .. } => continue,
            };

            if references_common_in_schema(schema, common_schemas) {
                return true;
            }
        }
    }

    false
}

/// Reports whether operations with the given tag mention common schemas in their responses.
pub fn does_tag_operations_reference_common(
    spec: &OpenAPI,
    tag: &str,
    common_schemas: &std::collections::HashSet<String>,
) -> bool {
    // Iterate through all paths and operations
    for (_path, path_item) in &spec.paths.paths {
        let path_item = match path_item {
            openapiv3::ReferenceOr::Item(item) => item,
            openapiv3::ReferenceOr::Reference { .. } => continue,
        };

        let operations = vec![
            path_item.get.as_ref(),
            path_item.post.as_ref(),
            path_item.put.as_ref(),
            path_item.patch.as_ref(),
            path_item.delete.as_ref(),
        ];

        for operation in operations.into_iter().flatten() {
            // Check if this operation has the current tag
            if !operation.tags.contains(&tag.to_string()) {
                continue;
            }

            // Check responses for common schema references
            for (_status, response_ref) in &operation.responses.responses {
                let response = match response_ref {
                    openapiv3::ReferenceOr::Item(r) => r,
                    openapiv3::ReferenceOr::Reference { .. } => continue,
                };

                for (_content_type, media_type) in &response.content {
                    if let Some(schema_ref) = &media_type.schema {
                        if references_common_schema_ref(schema_ref, common_schemas) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

/// Returns true when the schema reference resolves to one of the common schemas.
fn references_common_schema_ref(
    schema_ref: &openapiv3::ReferenceOr<openapiv3::Schema>,
    common_schemas: &std::collections::HashSet<String>,
) -> bool {
    match schema_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                return common_schemas.contains(schema_name);
            }
        }
        openapiv3::ReferenceOr::Item(schema) => {
            return references_common_in_schema(schema, common_schemas);
        }
    }
    false
}

/// Walks the schema tree to determine whether it references any common schema.
fn references_common_in_schema(
    schema: &openapiv3::Schema,
    common_schemas: &std::collections::HashSet<String>,
) -> bool {
    use openapiv3::{ReferenceOr, SchemaKind, Type};

    match &schema.schema_kind {
        SchemaKind::Type(Type::Object(obj)) => {
            for (_name, prop_ref) in &obj.properties {
                match prop_ref {
                    ReferenceOr::Reference { reference } => {
                        if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                            if common_schemas.contains(schema_name) {
                                return true;
                            }
                        }
                    }
                    ReferenceOr::Item(s) => {
                        if references_common_in_schema(s, common_schemas) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        SchemaKind::Type(Type::Array(arr)) => {
            if let Some(items) = &arr.items {
                match items {
                    ReferenceOr::Reference { reference } => {
                        if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                            return common_schemas.contains(schema_name);
                        }
                    }
                    ReferenceOr::Item(s) => {
                        return references_common_in_schema(s, common_schemas);
                    }
                }
            }
            false
        }
        SchemaKind::OneOf { one_of } | SchemaKind::AnyOf { any_of: one_of } => {
            for schema_ref in one_of {
                match schema_ref {
                    ReferenceOr::Reference { reference } => {
                        if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                            if common_schemas.contains(schema_name) {
                                return true;
                            }
                        }
                    }
                    ReferenceOr::Item(s) => {
                        if references_common_in_schema(s, common_schemas) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        SchemaKind::AllOf { all_of } => {
            for schema_ref in all_of {
                match schema_ref {
                    ReferenceOr::Reference { reference } => {
                        if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                            if common_schemas.contains(schema_name) {
                                return true;
                            }
                        }
                    }
                    ReferenceOr::Item(s) => {
                        if references_common_in_schema(s, common_schemas) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        _ => false,
    }
}
