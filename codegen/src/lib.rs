#![forbid(unsafe_code)]

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use heck::{ToSnakeCase, ToUpperCamelCase};
use oas3::{
    Spec as OpenAPI,
    spec::{MediaType, ObjectOrReference, ObjectSchema, Operation, Parameter, PathItem, Schema},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use operation::GeneratedClientMethods;

pub mod body;
pub mod client;
pub mod event;
mod oas;
pub mod operation;
pub mod samples;
pub mod schema;
mod symbol;
pub mod tag;

pub use body::generate_operation_bodies;
pub use client::generate_client_file;
pub use event::{generate_events_file, generate_tag_event_tokens};
pub use operation::generate_client_methods;
pub use samples::{CodeSample, CodeSampleCatalog, generate_code_samples};
pub use schema::{generate_module_doc_comment, generate_structs_for_schemas};
pub use tag::{SchemasByTag, TagSchemas, collect_schemas_by_tag};

/// A single operation selected for a given tag, along with traversal context.
#[derive(Clone, Copy)]
pub struct TaggedOperation<'a> {
    pub path: &'a str,
    pub http_method: &'static str,
    pub operation: &'a Operation,
    pub path_parameters: &'a [ObjectOrReference<Parameter>],
}

/// Returns the canonical operation name for code generation.
/// Prefers `x-codegen.method_name`, falling back to `operation_id` or "unknown".
pub fn operation_name(operation: &Operation) -> String {
    if let Some(name) = operation_codegen_method_name(operation) {
        return name.to_string();
    }

    operation
        .operation_id
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn operation_codegen_method_name(operation: &Operation) -> Option<&str> {
    operation
        .extensions
        .get("codegen")
        .and_then(serde_json::Value::as_object)
        .and_then(|codegen_obj| codegen_obj.get("method_name"))
        .and_then(serde_json::Value::as_str)
}

/// Collects all operations belonging to `tag`, sorted by path and HTTP method.
pub fn collect_tagged_operations<'a>(spec: &'a OpenAPI, tag: &str) -> Vec<TaggedOperation<'a>> {
    let mut operations = Vec::new();

    for (path, path_item) in spec.paths.iter().flat_map(|paths| paths.iter()) {
        for (http_method, operation) in operations_for_path_item(path_item) {
            if !operation
                .tags
                .iter()
                .any(|operation_tag| operation_tag == tag)
            {
                continue;
            }

            operations.push(TaggedOperation {
                path,
                http_method,
                operation,
                path_parameters: &path_item.parameters,
            });
        }
    }

    operations.sort_by(|a, b| {
        a.path
            .cmp(b.path)
            .then_with(|| a.http_method.cmp(b.http_method))
    });
    operations
}

pub(crate) fn operations_for_path_item(
    path_item: &PathItem,
) -> impl Iterator<Item = (&'static str, &Operation)> {
    [
        ("delete", path_item.delete.as_ref()),
        ("get", path_item.get.as_ref()),
        ("patch", path_item.patch.as_ref()),
        ("post", path_item.post.as_ref()),
        ("put", path_item.put.as_ref()),
    ]
    .into_iter()
    .filter_map(|(method, operation)| operation.map(|op| (method, op)))
}

pub(crate) fn preferred_response_media_type(
    content: &std::collections::BTreeMap<String, MediaType>,
) -> Option<&MediaType> {
    content
        .get("application/problem+json")
        .or_else(|| content.get("application/json"))
}

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
        self.generate_api_version_file()?;
        self.generate_common_module()?;
        self.generate_tag_modules()?;
        self.generate_events_module()?;
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
        sorted_tags.sort_by_key(|(a, _)| *a);

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

        let use_common_schemas = self.should_import_common(tag, tag_data);
        let mut symbols = symbol::SymbolRegistry::new(tag.to_snake_case());
        symbols.reserve("Client", "import `crate::client::Client`")?;

        if use_common_schemas {
            for name in schema::schema_symbol_names(
                &self.spec,
                &self.schemas_by_tag.common_schemas,
                &self.schemas_by_tag.common_error_schemas,
            )? {
                symbols.reserve(name.clone(), format!("common schema import `{name}`"))?;
            }
        }

        let schema_tokens = schema::generate_structs_for_schemas_with_registry(
            &self.spec,
            &tag_data.all_schemas,
            &tag_data.error_schemas,
            &mut symbols,
        )?;
        let body_tokens =
            body::generate_operation_bodies_with_registry(&self.spec, tag, &mut symbols)?;
        let client_tokens = generate_tag_client_with_registry(&self.spec, tag, &mut symbols)?;
        let event_tokens = generate_tag_event_tokens(&self.spec, tag)?;
        let module_doc_comment = tag_description(&self.spec, tag)
            .map(generate_module_doc_comment)
            .unwrap_or_default();

        let use_common = if use_common_schemas {
            quote! {
                use super::common::*;
            }
        } else {
            quote! {}
        };

        let combined_tokens = quote! {
            #module_doc_comment

            #use_common

            #event_tokens

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
        generate_client_file(&self.out_path, &self.spec, &self.schemas_by_tag.tag_schemas)
    }

    fn generate_events_module(&self) -> Result<(), String> {
        Self::log("[generate sdk] generating events.rs ...");
        generate_events_file(&self.out_path, &self.spec)
    }

    fn generate_api_version_file(&self) -> Result<(), String> {
        let api_version = self.spec.info.version.trim().to_string();

        if api_version.is_empty() {
            return Err("OpenAPI spec version is empty".to_string());
        }

        let mut version_path = self.out_path.clone();
        version_path.push("api_version.rs");

        let api_version_literal = syn::LitStr::new(&api_version, Span::call_site());
        let tokens = quote! {
            /// The version of the SumUp API spec.
            pub const API_VERSION: &str = #api_version_literal;
        };

        let contents = format_generated_code(tokens);
        std::fs::write(&version_path, &contents)
            .map_err(|e| format!("Failed to write api_version.rs: {}", e))?;

        Ok(())
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

pub(crate) fn tag_description<'a>(spec: &'a OpenAPI, tag: &str) -> Option<&'a str> {
    spec.tags
        .iter()
        .find(|candidate| candidate.name == tag)
        .and_then(|candidate| candidate.description.as_deref())
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

        modules.push(quote! {
            pub mod #module_name;
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

    let mut symbols = symbol::SymbolRegistry::new("common");
    let schema_tokens = schema::generate_structs_for_schemas_with_registry(
        spec,
        &schemas_by_tag.common_schemas,
        &schemas_by_tag.common_error_schemas,
        &mut symbols,
    )?;

    let contents = format_generated_code(schema_tokens);
    std::fs::write(&common_path, &contents)
        .map_err(|e| format!("Failed to write resources/common.rs: {}", e))?;

    Ok(())
}

/// Formats generated tokens into Rust source and prepends the standard header.
pub fn format_generated_code(tokens: TokenStream) -> String {
    let header = "// The contents of this file are generated; do not modify them.\n\n";

    // First use prettyplease for basic formatting when the token stream parses as a full file.
    let tokens_str = tokens.to_string();
    let formatted = match syn::parse_file(&tokens_str) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => tokens_str,
    };

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
        .arg("--edition=2024")
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
    let mut symbols = symbol::SymbolRegistry::new(tag.to_snake_case());
    symbols.reserve("Client", "import `crate::client::Client`")?;
    generate_tag_client_with_registry(spec, tag, &mut symbols)
}

fn generate_tag_client_with_registry(
    spec: &OpenAPI,
    tag: &str,
    symbols: &mut symbol::SymbolRegistry,
) -> Result<TokenStream, String> {
    let client_type = Ident::new(
        &format!("{}Client", tag.to_upper_camel_case()),
        Span::call_site(),
    );
    symbols.reserve(client_type.to_string(), format!("client for tag `{tag}`"))?;
    let doc_comment =
        schema::generate_doc_comment(&format!("Client for the {} API endpoints.", tag));

    // Generate methods for operations with this tag
    let GeneratedClientMethods {
        methods,
        extra_items,
    } = operation::generate_client_methods_with_registry(spec, tag, symbols)?;
    let methods_tokens = quote! { #(#methods)* };
    let extra_items_tokens = if extra_items.is_empty() {
        quote! {}
    } else {
        quote! { #(#extra_items)* }
    };

    Ok(quote! {
        use crate::client::Client;

        #extra_items_tokens

        #doc_comment
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
                ObjectOrReference::Object(schema) => schema,
                ObjectOrReference::Ref { .. } => continue,
            };

            if references_common_in_schema(schema, common_schemas) {
                return true;
            }
        }
    }

    false
}

/// Reports whether operations with the given tag mention common schemas in their bodies.
pub fn does_tag_operations_reference_common(
    spec: &OpenAPI,
    tag: &str,
    common_schemas: &std::collections::HashSet<String>,
) -> bool {
    for tagged_operation in collect_tagged_operations(spec, tag) {
        if let Some(request_body_ref) = &tagged_operation.operation.request_body
            && let Ok(request_body) = body::resolve_request_body(spec, request_body_ref)
            && let Some(schema_ref) = body::request_body_schema(request_body)
        {
            if references_common_schema_ref(schema_ref, common_schemas) {
                return true;
            }

            if let Ok(schema) = schema::dereference_schema(spec, schema_ref)
                && references_common_in_schema(schema, common_schemas)
            {
                return true;
            }
        }

        for response_ref in tagged_operation
            .operation
            .responses
            .iter()
            .flat_map(|responses| responses.values())
        {
            let response = match response_ref {
                ObjectOrReference::Object(response) => response,
                ObjectOrReference::Ref { .. } => continue,
            };

            if let Some(media_type) = preferred_response_media_type(&response.content)
                && let Some(schema_ref) = &media_type.schema
                && references_common_schema_ref(schema_ref, common_schemas)
            {
                return true;
            }
        }
    }

    false
}

/// Returns true when the schema reference resolves to one of the common schemas.
fn references_common_schema_ref(
    schema_ref: &ObjectOrReference<ObjectSchema>,
    common_schemas: &std::collections::HashSet<String>,
) -> bool {
    match schema_ref {
        ObjectOrReference::Ref { ref_path, .. } => {
            if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
                return common_schemas.contains(schema_name);
            }
        }
        ObjectOrReference::Object(schema) => {
            return references_common_in_schema(schema, common_schemas);
        }
    }
    false
}

/// Walks the schema tree to determine whether it references any common schema.
fn references_common_in_schema(
    schema: &ObjectSchema,
    common_schemas: &std::collections::HashSet<String>,
) -> bool {
    fn reference_matches(
        schema_ref: &ObjectOrReference<ObjectSchema>,
        common_schemas: &std::collections::HashSet<String>,
    ) -> bool {
        match schema_ref {
            ObjectOrReference::Ref { ref_path, .. } => ref_path
                .strip_prefix("#/components/schemas/")
                .is_some_and(|name| common_schemas.contains(name)),
            ObjectOrReference::Object(schema) => {
                references_common_in_schema(schema, common_schemas)
            }
        }
    }

    schema
        .properties
        .values()
        .chain(&schema.one_of)
        .chain(&schema.any_of)
        .chain(&schema.all_of)
        .chain(&schema.prefix_items)
        .any(|schema_ref| reference_matches(schema_ref, common_schemas))
        || schema.items.as_deref().is_some_and(|schema| match schema {
            Schema::Boolean(_) => false,
            Schema::Object(schema_ref) => reference_matches(schema_ref, common_schemas),
        })
        || schema
            .additional_properties
            .as_ref()
            .is_some_and(|schema| match schema {
                Schema::Boolean(_) => false,
                Schema::Object(schema_ref) => reference_matches(schema_ref, common_schemas),
            })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use serde_json::json;
    use std::{collections::HashSet, str::FromStr};

    fn parse_spec(value: serde_json::Value) -> OpenAPI {
        serde_json::from_value(value).expect("failed to parse OpenAPI fixture")
    }

    fn generate_tag_surface(spec: &OpenAPI, tag: &str) -> Result<(), String> {
        let grouped = collect_schemas_by_tag(spec)?;
        let tag_data = grouped
            .tag_schemas
            .get(tag)
            .ok_or_else(|| format!("tag `{tag}` not found"))?;
        let mut symbols = symbol::SymbolRegistry::new(tag.to_snake_case());
        symbols.reserve("Client", "import `crate::client::Client`")?;

        let uses_common =
            does_reference_common_schemas(spec, &tag_data.all_schemas, &grouped.common_schemas)
                || does_tag_operations_reference_common(spec, tag, &grouped.common_schemas);
        if uses_common {
            for name in schema::schema_symbol_names(
                spec,
                &grouped.common_schemas,
                &grouped.common_error_schemas,
            )? {
                symbols.reserve(name.clone(), format!("common schema import `{name}`"))?;
            }
        }

        schema::generate_structs_for_schemas_with_registry(
            spec,
            &tag_data.all_schemas,
            &tag_data.error_schemas,
            &mut symbols,
        )?;
        body::generate_operation_bodies_with_registry(spec, tag, &mut symbols)?;
        generate_tag_client_with_registry(spec, tag, &mut symbols)?;
        Ok(())
    }

    #[test]
    fn collect_tagged_operations_filters_and_sorts_operations() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/z-endpoint": {
                    "get": {
                        "operationId": "listZ",
                        "tags": ["Target"],
                        "responses": { "200": { "description": "ok" } }
                    }
                },
                "/a-endpoint": {
                    "post": {
                        "operationId": "createA",
                        "tags": ["Target"],
                        "responses": { "200": { "description": "ok" } }
                    },
                    "get": {
                        "operationId": "listA",
                        "tags": ["Other"],
                        "responses": { "200": { "description": "ok" } }
                    }
                }
            }
        }));

        let operations = collect_tagged_operations(&spec, "Target");
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].path, "/a-endpoint");
        assert_eq!(operations[0].http_method, "post");
        assert_eq!(operations[1].path, "/z-endpoint");
        assert_eq!(operations[1].http_method, "get");
    }

    #[test]
    fn component_and_operation_request_collision_reports_both_origins() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/checkouts": {
                    "post": {
                        "operationId": "CreateReaderCheckout",
                        "x-codegen": { "method_name": "create_checkout" },
                        "tags": ["Readers"],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": { "amount": { "type": "number" } }
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/CreateCheckoutRequest"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "CreateCheckoutRequest": { "type": "object" }
                }
            }
        }));

        let error = generate_tag_surface(&spec, "Readers")
            .expect_err("component and request names should collide");
        assert!(error.contains("component schema `CreateCheckoutRequest`"));
        assert!(error.contains("request body for operation `create_checkout`"));
    }

    #[test]
    fn duplicate_operation_request_names_report_both_operations() {
        let request_body = json!({
            "required": true,
            "content": {
                "application/json": {
                    "schema": {
                        "type": "object",
                        "properties": { "value": { "type": "string" } }
                    }
                }
            }
        });
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/a": {
                    "post": {
                        "operationId": "CreateA",
                        "x-codegen": { "method_name": "create_checkout" },
                        "tags": ["Readers"],
                        "requestBody": request_body.clone(),
                        "responses": { "204": { "description": "ok" } }
                    }
                },
                "/b": {
                    "post": {
                        "operationId": "CreateB",
                        "x-codegen": { "method_name": "create_checkout" },
                        "tags": ["Readers"],
                        "requestBody": request_body,
                        "responses": { "204": { "description": "ok" } }
                    }
                }
            }
        }));

        let error = generate_tag_surface(&spec, "Readers")
            .expect_err("duplicate operation request names should collide");
        assert!(error.contains("post /a, operationId `CreateA`"));
        assert!(error.contains("post /b, operationId `CreateB`"));
    }

    #[test]
    fn common_import_and_operation_request_collision_is_rejected() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/target": {
                    "post": {
                        "operationId": "CreateTarget",
                        "x-codegen": { "method_name": "create" },
                        "tags": ["Target"],
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": { "type": "object" }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/CreateRequest" }
                                    }
                                }
                            }
                        }
                    }
                },
                "/other": {
                    "get": {
                        "operationId": "GetOther",
                        "tags": ["Other"],
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/CreateRequest" }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "CreateRequest": { "type": "object" }
                }
            }
        }));

        let error = generate_tag_surface(&spec, "Target")
            .expect_err("common import and request names should collide");
        assert!(error.contains("common schema import `CreateRequest`"));
        assert!(error.contains("request body for operation `create`"));
    }

    #[test]
    fn component_and_operation_error_collision_is_rejected() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/create": {
                    "post": {
                        "operationId": "CreateThing",
                        "x-codegen": { "method_name": "create" },
                        "tags": ["Things"],
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/CreateErrorBody" }
                                    }
                                }
                            },
                            "400": { "description": "bad request" }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "CreateErrorBody": { "type": "object" }
                }
            }
        }));

        let error = generate_tag_surface(&spec, "Things")
            .expect_err("component and error names should collide");
        assert!(error.contains("component schema `CreateErrorBody`"));
        assert!(error.contains("error response body for operation `create`"));
    }

    #[test]
    fn component_and_operation_params_collision_is_rejected() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/things": {
                    "get": {
                        "operationId": "ListThings",
                        "x-codegen": { "method_name": "list" },
                        "tags": ["Things"],
                        "parameters": [
                            {
                                "name": "query",
                                "in": "query",
                                "schema": { "type": "string" }
                            }
                        ],
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/ListParams" }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "ListParams": { "type": "object" }
                }
            }
        }));

        let error = generate_tag_surface(&spec, "Things")
            .expect_err("component and params names should collide");
        assert!(error.contains("component schema `ListParams`"));
        assert!(error.contains("query parameters for operation `list`"));
    }

    #[test]
    fn component_and_inline_response_collision_is_rejected() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/inline": {
                    "get": {
                        "operationId": "GetInline",
                        "x-codegen": { "method_name": "get" },
                        "tags": ["Things"],
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "type": "object",
                                            "properties": { "value": { "type": "string" } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/component": {
                    "get": {
                        "operationId": "GetComponent",
                        "tags": ["Things"],
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/GetResponse" }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "GetResponse": { "type": "object" }
                }
            }
        }));

        let error = generate_tag_surface(&spec, "Things")
            .expect_err("component and response names should collide");
        assert!(error.contains("component schema `GetResponse`"));
        assert!(error.contains("success response body for operation `get`"));
    }

    #[test]
    fn component_and_nested_inline_collision_is_rejected() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/create": {
                    "post": {
                        "operationId": "CreateThing",
                        "x-codegen": { "method_name": "create" },
                        "tags": ["Things"],
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "details": {
                                                "title": "IgnoredTitle",
                                                "type": "object",
                                                "properties": {
                                                    "value": { "type": "string" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/CreateRequestDetails"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "CreateRequestDetails": { "type": "object" }
                }
            }
        }));

        let error = generate_tag_surface(&spec, "Things")
            .expect_err("component and nested names should collide");
        assert!(error.contains("component schema `CreateRequestDetails`"));
        assert!(error.contains("inline schema for field `CreateRequest.details`"));
    }

    #[test]
    fn format_generated_code_falls_back_for_non_file_token_streams() {
        let tokens = TokenStream::from_str("not valid rust syntax").expect("valid token stream");
        let formatted = format_generated_code(tokens);
        assert!(
            formatted
                .starts_with("// The contents of this file are generated; do not modify them.")
        );
        assert!(formatted.contains("not valid rust syntax"));
    }

    #[test]
    fn does_tag_operations_reference_common_checks_only_selected_tag() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/target": {
                    "get": {
                        "operationId": "targetOp",
                        "tags": ["Target"],
                        "responses": {
                            "200": {
                                "description": "ok",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/Common" }
                                    }
                                }
                            }
                        }
                    }
                },
                "/other": {
                    "get": {
                        "operationId": "otherOp",
                        "tags": ["Other"],
                        "responses": { "200": { "description": "ok" } }
                    }
                }
            }
        }));

        let common_schemas = HashSet::from(["Common".to_string()]);
        assert!(does_tag_operations_reference_common(
            &spec,
            "Target",
            &common_schemas
        ));
        assert!(!does_tag_operations_reference_common(
            &spec,
            "Other",
            &common_schemas
        ));
    }

    #[test]
    fn operation_name_prefers_codegen_method_name() {
        let operation = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/demo": {
                    "get": {
                        "operationId": "listDemo",
                        "x-codegen": { "method_name": "customDemoMethod" },
                        "responses": { "200": { "description": "ok" } }
                    }
                }
            }
        }))
        .paths
        .expect("fixture should contain paths")
        .values()
        .find_map(|path_item| path_item.get.as_ref())
        .expect("fixture should contain operation")
        .clone();

        assert_eq!(operation_name(&operation), "customDemoMethod");
    }

    #[test]
    fn operation_name_falls_back_to_operation_id_and_unknown() {
        let with_operation_id = oas3::spec::Operation {
            operation_id: Some("listDemo".to_string()),
            ..Default::default()
        };
        assert_eq!(operation_name(&with_operation_id), "listDemo");

        let without_name = oas3::spec::Operation::default();
        assert_eq!(operation_name(&without_name), "unknown");
    }

    #[test]
    fn tag_description_returns_matching_tag_description() {
        let spec = parse_spec(json!({
            "openapi": "3.0.0",
            "info": { "title": "test", "version": "1.0.0" },
            "tags": [
                {
                    "name": "Checkouts",
                    "description": "Checkout operations."
                }
            ],
            "paths": {}
        }));

        assert_eq!(
            tag_description(&spec, "Checkouts"),
            Some("Checkout operations.")
        );
        assert_eq!(tag_description(&spec, "Transactions"), None);
    }
}
