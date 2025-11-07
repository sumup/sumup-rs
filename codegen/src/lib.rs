#![forbid(unsafe_code)]

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use heck::{ToSnakeCase, ToUpperCamelCase};
use openapiv3::OpenAPI;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use operation::GeneratedClientMethods;

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

/// Coordinates SDK generation for a given OpenAPI spec and output location.
pub struct Generator {
    spec: OpenAPI,
    out_path: PathBuf,
    schemas_by_tag: SchemasByTag,
}

impl Generator {
    /// Prepares a generator by loading derived schema metadata for later use.
    pub fn new(spec: OpenAPI, out_path: impl Into<PathBuf>) -> Result<Self, String> {
        let mut out_path = out_path.into();
        out_path.push("src");
        let schemas_by_tag = collect_schemas_by_tag(&spec)?;
        Ok(Self {
            spec,
            out_path,
            schemas_by_tag,
        })
    }

    /// Generates the full SDK into the configured output directory.
    pub fn generate(&self) -> Result<(), String> {
        Self::log("[generate sdk] analyzing tags and operations ...");
        Self::log(&format!(
            "[generate sdk] found {} tags, {} common schemas",
            self.schemas_by_tag.tag_schemas.len(),
            self.schemas_by_tag.common_schemas.len()
        ));

        self.ensure_directories()?;
        self.generate_common_module()?;
        self.generate_tag_modules()?;
        self.generate_client_module()?;
        self.generate_mod_rs()?;

        Self::log("[generate sdk] ... done");
        Ok(())
    }

    fn ensure_directories(&self) -> Result<(), String> {
        let resources_path = self.resources_dir();
        std::fs::create_dir_all(&resources_path)
            .map_err(|e| format!("Failed to create resources directory: {}", e))
    }

    fn generate_common_module(&self) -> Result<(), String> {
        if self.schemas_by_tag.common_schemas.is_empty() {
            return Ok(());
        }

        Self::log(&format!(
            "[generate sdk] generating common.rs with {} shared schemas ({} error schemas) ...",
            self.schemas_by_tag.common_schemas.len(),
            self.schemas_by_tag.common_error_schemas.len()
        ));

        generate_common_file(&self.out_path, &self.spec, &self.schemas_by_tag)
    }

    fn generate_tag_modules(&self) -> Result<(), String> {
        let mut sorted_tags: Vec<_> = self.schemas_by_tag.tag_schemas.iter().collect();
        sorted_tags.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (tag, tag_data) in sorted_tags {
            self.generate_tag_module(tag, tag_data)?;
        }

        Ok(())
    }

    fn generate_tag_module(&self, tag: &str, tag_data: &TagSchemas) -> Result<(), String> {
        Self::log(&format!(
            "[generate sdk] generating {} with {} schemas ({} error schemas) ...",
            tag,
            tag_data.all_schemas.len(),
            tag_data.error_schemas.len()
        ));

        let schema_tokens = generate_structs_for_schemas(
            &self.spec,
            &tag_data.all_schemas,
            &tag_data.error_schemas,
        )?;
        let body_tokens = generate_operation_bodies(&self.spec, tag)?;
        let client_tokens =
            generate_tag_client(&self.spec, tag, tag_data.deprecation_notice.as_deref())?;

        let use_common = if self.should_import_common(tag, tag_data) {
            quote! {
                use super::common::*;
            }
        } else {
            quote! {}
        };

        let combined_tokens = quote! {
            #use_common

            #schema_tokens

            #body_tokens

            #client_tokens
        };

        let contents = format_generated_code(combined_tokens);

        let file_name = format!("{}.rs", tag.to_snake_case());
        let mut tag_out_path = self.resources_dir();
        tag_out_path.push(&file_name);

        std::fs::write(&tag_out_path, &contents)
            .map_err(|e| format!("Failed to write {}: {}", file_name, e))?;

        Ok(())
    }

    fn should_import_common(&self, tag: &str, tag_data: &TagSchemas) -> bool {
        if self.schemas_by_tag.common_schemas.is_empty() {
            return false;
        }

        does_reference_common_schemas(
            &self.spec,
            &tag_data.all_schemas,
            &self.schemas_by_tag.common_schemas,
        ) || does_tag_operations_reference_common(
            &self.spec,
            tag,
            &self.schemas_by_tag.common_schemas,
        )
    }

    fn generate_client_module(&self) -> Result<(), String> {
        Self::log("[generate sdk] generating client.rs ...");
        generate_client_file(&self.out_path, &self.schemas_by_tag.tag_schemas)
    }

    fn generate_mod_rs(&self) -> Result<(), String> {
        generate_mod_file(&self.out_path, &self.schemas_by_tag)
    }

    fn resources_dir(&self) -> PathBuf {
        let mut resources_path = self.out_path.clone();
        resources_path.push("resources");
        resources_path
    }

    fn log(message: &str) {
        println!("{}", message);
        let _ = std::io::stdout().flush();
    }
}

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
        let tag_data = &tag_schemas[tag];
        let is_deprecated = tag_data.deprecation_notice.is_some();

        if is_deprecated {
            modules.push(quote! {
                #[cfg(feature = "deprecated-resources")]
                pub mod #module_name;
            });
            re_exports.push(quote! {
                #[cfg(feature = "deprecated-resources")]
                pub use #module_name::*;
            });
        } else {
            modules.push(quote! {
                pub mod #module_name;
            });
            re_exports.push(quote! {
                pub use #module_name::*;
            });
        }
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
pub fn generate_tag_client(
    spec: &OpenAPI,
    tag: &str,
    deprecation_notice: Option<&str>,
) -> Result<TokenStream, String> {
    let client_type = Ident::new(
        &format!("{}Client", tag.to_upper_camel_case()),
        Span::call_site(),
    );
    let doc_comment = format!("Client for the {} API endpoints.", tag);

    // Generate methods for operations with this tag
    let GeneratedClientMethods {
        methods,
        extra_items,
    } = generate_client_methods(spec, tag)?;
    let methods_tokens = quote! { #(#methods)* };
    let extra_items_tokens = if extra_items.is_empty() {
        quote! {}
    } else {
        quote! { #(#extra_items)* }
    };

    let deprecation_attr = if let Some(notice) = deprecation_notice {
        quote! {
            #[deprecated(note = #notice)]
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        use crate::client::Client;

        #extra_items_tokens

        #[doc = #doc_comment]
        #deprecation_attr
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

            #methods_tokens
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
