use heck::{ToSnakeCase, ToUpperCamelCase};
use oas3::{Spec as OpenAPI, spec as openapiv3};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Generates request and response body structs for every operation under the given tag.
pub fn generate_operation_bodies(spec: &OpenAPI, tag: &str) -> Result<TokenStream, String> {
    let mut symbols = crate::symbol::SymbolRegistry::new(tag.to_snake_case());
    generate_operation_bodies_with_registry(spec, tag, &mut symbols)
}

pub(crate) fn generate_operation_bodies_with_registry(
    spec: &OpenAPI,
    tag: &str,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<TokenStream, String> {
    let mut body_structs = Vec::new();
    let mut nested_schemas = Vec::new();

    // Collect all operations with this tag and sort them by path, method, and operation name.
    let mut operations_to_process = Vec::new();

    for tagged_operation in crate::collect_tagged_operations(spec, tag) {
        let operation_id = tagged_operation
            .operation
            .operation_id
            .as_ref()
            .ok_or_else(|| {
                format!(
                    "Operation {} {} missing operation_id",
                    tagged_operation.http_method, tagged_operation.path
                )
            })?;
        let operation_name = crate::operation_name(tagged_operation.operation);

        operations_to_process.push((
            tagged_operation.path,
            tagged_operation.http_method,
            operation_id.clone(),
            operation_name,
            tagged_operation.operation,
        ));
    }

    // Sort operations alphabetically by path, then method, then operation name
    operations_to_process.sort_by(|a, b| {
        a.0.cmp(b.0)
            .then_with(|| a.1.cmp(b.1))
            .then_with(|| a.3.cmp(&b.3))
    });

    for (path, http_method, operation_id, operation_name, op) in operations_to_process {
        let operation_origin = format!(
            "operation `{operation_name}` ({http_method} {path}, operationId `{operation_id}`)"
        );

        // Generate query params struct if present
        let query_types =
            generate_query_param_types(&operation_name, op, &operation_origin, symbols)?;
        nested_schemas.extend(query_types);

        if let Some(params_struct) =
            generate_query_params_struct(op, &operation_name, &operation_origin, symbols)?
        {
            body_structs.push(params_struct);
        }

        // Generate request body struct if present
        if let Some(request_body_ref) = &op.request_body {
            if let Some(body_struct) = generate_request_body_struct(
                spec,
                &operation_name,
                &operation_origin,
                request_body_ref,
                &mut nested_schemas,
                symbols,
            )? {
                body_structs.push(body_struct);
            }
        }

        // Generate response body struct(s)
        if let Some(response_structs) = generate_response_body_structs(
            spec,
            &operation_name,
            &operation_origin,
            op.responses.as_ref(),
            &mut nested_schemas,
            symbols,
        )? {
            body_structs.extend(response_structs);
        }
    }

    // Add nested schemas before body structs
    let mut all_structs = nested_schemas;
    all_structs.extend(body_structs);

    Ok(quote! {
        #(#all_structs)*
    })
}

/// Creates a query params struct for the operation when query parameters are defined.
fn generate_query_params_struct(
    operation: &openapiv3::Operation,
    operation_name: &str,
    operation_origin: &str,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<Option<TokenStream>, String> {
    // Collect query parameters
    let mut query_params = Vec::new();

    for param_ref in &operation.parameters {
        let param = match param_ref {
            openapiv3::ObjectOrReference::Object(p) => p,
            openapiv3::ObjectOrReference::Ref { .. } => continue,
        };

        if param.location == openapiv3::ParameterIn::Query {
            query_params.push(param);
        }
    }

    if query_params.is_empty() {
        return Ok(None);
    }

    let params_struct_name = format!("{}Params", operation_name.to_upper_camel_case());
    symbols.reserve(
        params_struct_name.clone(),
        format!("query parameters for {operation_origin}"),
    )?;
    let struct_name = Ident::new(&params_struct_name, Span::call_site());

    // Generate fields
    let mut fields = Vec::new();
    for param_data in query_params {
        let field_name = crate::schema::make_rust_field_ident(&param_data.name.to_snake_case());
        let original_name = &param_data.name;

        // Determine field type based on schema
        let (field_type, is_nullable) = if let Some(schema_ref) = &param_data.schema {
            infer_param_type(
                operation_name,
                &field_name.to_string(),
                schema_ref,
                param_data.required.unwrap_or(false),
            )
        } else if param_data.required.unwrap_or(false) {
            (quote! { String }, false)
        } else {
            (quote! { Option<String> }, false)
        };

        let rename_attr = if original_name != &field_name.to_string() {
            quote! { #[serde(rename = #original_name)] }
        } else {
            quote! {}
        };

        let skip_attr = if !param_data.required.unwrap_or(false) {
            if is_nullable {
                quote! { #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::nullable::deserialize")] }
            } else {
                quote! { #[serde(skip_serializing_if = "Option::is_none")] }
            }
        } else {
            quote! {}
        };

        let description = match &param_data.schema {
            Some(openapiv3::ObjectOrReference::Object(schema)) => {
                let doc = crate::schema::generate_schema_doc_comment(
                    param_data.description.as_deref(),
                    schema,
                );
                quote! { #doc }
            }
            _ => {
                if let Some(desc) = &param_data.description {
                    let doc = crate::schema::generate_doc_comment(desc);
                    quote! { #doc }
                } else {
                    quote! {}
                }
            }
        };

        fields.push(quote! {
            #description
            #rename_attr
            #skip_attr
            pub #field_name: #field_type
        });
    }

    Ok(Some(quote! {
        #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct #struct_name {
            #(#fields,)*
        }
    }))
}

/// Infers the Rust type for a query parameter schema and reports whether it is nullable.
/// Returns a tuple of (field_type, is_nullable).
fn infer_param_type(
    operation_name: &str,
    field_name: &str,
    schema_ref: &openapiv3::ObjectOrReference<openapiv3::ObjectSchema>,
    required: bool,
) -> (TokenStream, bool) {
    let (base_type, is_nullable) = match schema_ref {
        openapiv3::ObjectOrReference::Ref {
            ref_path: reference,
            ..
        } => {
            let type_name = reference.split('/').next_back().unwrap_or("Unknown");
            let type_ident = Ident::new(&type_name.to_upper_camel_case(), Span::call_site());
            (quote! { #type_ident }, false)
        }
        openapiv3::ObjectOrReference::Object(schema) => {
            let is_nullable = schema.is_nullable().unwrap_or(false);
            let base = match crate::oas::schema_type(schema) {
                Some(openapiv3::SchemaType::String) => {
                    if !schema.enum_values.is_empty() {
                        let type_ident = query_param_type_ident(operation_name, field_name);
                        quote! { #type_ident }
                    } else {
                        match schema.format.as_deref() {
                            Some("date-time") => {
                                quote! { crate::datetime::DateTime }
                            }
                            Some("date") => {
                                quote! { crate::datetime::Date }
                            }
                            Some("password") => {
                                quote! { crate::secret::Secret }
                            }
                            _ => quote! { String },
                        }
                    }
                }
                Some(openapiv3::SchemaType::Number) => quote! { f64 },
                Some(openapiv3::SchemaType::Integer) => quote! { i64 },
                Some(openapiv3::SchemaType::Boolean) => quote! { bool },
                Some(openapiv3::SchemaType::Array) => {
                    if let Some(openapiv3::Schema::Object(items)) = schema.items.as_deref() {
                        let item_type = match items.as_ref() {
                            openapiv3::ObjectOrReference::Ref {
                                ref_path: reference,
                                ..
                            } => {
                                let type_name =
                                    reference.split('/').next_back().unwrap_or("Unknown");
                                let type_ident =
                                    Ident::new(&type_name.to_upper_camel_case(), Span::call_site());
                                quote! { #type_ident }
                            }
                            openapiv3::ObjectOrReference::Object(inner_schema) => {
                                match crate::oas::schema_type(inner_schema) {
                                    Some(openapiv3::SchemaType::String) => {
                                        if !inner_schema.enum_values.is_empty() {
                                            let type_ident = query_param_item_type_ident(
                                                operation_name,
                                                field_name,
                                            );
                                            quote! { #type_ident }
                                        } else {
                                            match inner_schema.format.as_deref() {
                                                Some("date-time") => {
                                                    quote! { crate::datetime::DateTime }
                                                }
                                                Some("date") => {
                                                    quote! { crate::datetime::Date }
                                                }
                                                _ => quote! { String },
                                            }
                                        }
                                    }
                                    Some(openapiv3::SchemaType::Integer) => {
                                        quote! { i64 }
                                    }
                                    Some(openapiv3::SchemaType::Number) => {
                                        quote! { f64 }
                                    }
                                    _ => quote! { String },
                                }
                            }
                        };
                        quote! { Vec<#item_type> }
                    } else {
                        quote! { Vec<String> }
                    }
                }
                _ => quote! { String },
            };
            (base, is_nullable)
        }
    };

    let field_type = if required {
        base_type
    } else if is_nullable {
        quote! { Option<crate::Nullable<#base_type>> }
    } else {
        quote! { Option<#base_type> }
    };

    (field_type, is_nullable)
}

fn generate_query_param_types(
    operation_name: &str,
    operation: &openapiv3::Operation,
    operation_origin: &str,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<Vec<TokenStream>, String> {
    let mut generated = Vec::new();

    for param_ref in &operation.parameters {
        let param = match param_ref {
            openapiv3::ObjectOrReference::Object(p) => p,
            openapiv3::ObjectOrReference::Ref { .. } => continue,
        };

        if param.location != openapiv3::ParameterIn::Query {
            continue;
        }

        let Some(schema_ref) = &param.schema else {
            continue;
        };

        let field_name = param.name.to_snake_case();
        generated.extend(generate_query_param_type_definition(
            operation_name,
            &field_name,
            schema_ref,
            operation_origin,
            symbols,
        )?);
    }

    Ok(generated)
}

fn generate_query_param_type_definition(
    operation_name: &str,
    field_name: &str,
    schema_ref: &openapiv3::ObjectOrReference<openapiv3::ObjectSchema>,
    operation_origin: &str,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<Vec<TokenStream>, String> {
    let openapiv3::ObjectOrReference::Object(schema) = schema_ref else {
        return Ok(Vec::new());
    };

    match crate::oas::schema_type(schema) {
        Some(openapiv3::SchemaType::String) => {
            if schema.enum_values.is_empty() {
                return Ok(Vec::new());
            }

            let type_ident = query_param_type_ident(operation_name, field_name);
            symbols.reserve(
                type_ident.to_string(),
                format!("query parameter `{field_name}` for {operation_origin}"),
            )?;

            Ok(vec![build_string_enum(
                type_ident,
                &schema.enum_values,
                schema.description.as_deref(),
            )?])
        }
        Some(openapiv3::SchemaType::Array) => {
            let Some(openapiv3::Schema::Object(items)) = schema.items.as_deref() else {
                return Ok(Vec::new());
            };
            let openapiv3::ObjectOrReference::Object(item_schema) = items.as_ref() else {
                return Ok(Vec::new());
            };

            if crate::oas::schema_type(item_schema) != Some(openapiv3::SchemaType::String) {
                return Ok(Vec::new());
            }

            if item_schema.enum_values.is_empty() {
                return Ok(Vec::new());
            }

            let type_ident = query_param_item_type_ident(operation_name, field_name);
            symbols.reserve(
                type_ident.to_string(),
                format!("query parameter `{field_name}` item for {operation_origin}"),
            )?;

            Ok(vec![build_string_enum(
                type_ident,
                &item_schema.enum_values,
                item_schema.description.as_deref(),
            )?])
        }
        _ => Ok(Vec::new()),
    }
}

fn build_string_enum(
    type_ident: Ident,
    enumeration: &[serde_json::Value],
    description: Option<&str>,
) -> Result<TokenStream, String> {
    let mut variant_names = std::collections::HashSet::new();
    let mut variants_tokens = Vec::new();

    for variant in enumeration.iter().filter_map(serde_json::Value::as_str) {
        let variant_name = crate::schema::sanitize_enum_variant(variant);
        if !variant_names.insert(variant_name.clone()) {
            return Err(format!(
                "Duplicate enum variant name generated for query param enum: {variant_name}"
            ));
        }

        let variant_ident = Ident::new(&variant_name, Span::call_site());
        if variant != variant_name {
            variants_tokens.push(quote! {
                #[serde(rename = #variant)]
                #variant_ident
            });
        } else {
            variants_tokens.push(quote! { #variant_ident });
        }
    }

    if variants_tokens.is_empty() {
        return Ok(quote! { pub type #type_ident = String; });
    }

    let other_variant_ident = if variant_names.contains("Other") {
        Ident::new("OtherValue", Span::call_site())
    } else {
        Ident::new("Other", Span::call_site())
    };

    let description = description.map(crate::schema::generate_doc_comment);

    Ok(quote! {
        #description
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum #type_ident {
            #(#variants_tokens,)*
            #[serde(untagged)]
            #other_variant_ident(String),
        }
    })
}

fn query_param_type_ident(operation_name: &str, field_name: &str) -> Ident {
    Ident::new(
        &format!(
            "{}Params{}",
            operation_name.to_upper_camel_case(),
            field_name.to_upper_camel_case()
        ),
        Span::call_site(),
    )
}

fn query_param_item_type_ident(operation_name: &str, field_name: &str) -> Ident {
    Ident::new(
        &format!(
            "{}Params{}Item",
            operation_name.to_upper_camel_case(),
            field_name.to_upper_camel_case()
        ),
        Span::call_site(),
    )
}

/// Emits a request body type named after the operation.
fn generate_request_body_struct(
    spec: &OpenAPI,
    operation_name: &str,
    operation_origin: &str,
    request_body_ref: &openapiv3::ObjectOrReference<openapiv3::RequestBody>,
    nested_schemas: &mut Vec<TokenStream>,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<Option<TokenStream>, String> {
    let request_body = resolve_request_body(spec, request_body_ref)?;
    let Some(schema_ref) = request_body_schema(request_body) else {
        return Ok(None);
    };
    let schema = crate::schema::dereference_schema(spec, schema_ref)?;
    let struct_name = operation_request_type_ident(operation_name);
    let struct_name_str = struct_name.to_string();

    symbols.reserve(
        struct_name_str,
        format!("request body for {operation_origin}"),
    )?;

    let description = request_body
        .description
        .as_deref()
        .or(schema.description.as_deref())
        .map(|description| crate::schema::generate_schema_doc_comment(Some(description), schema));

    generate_schema_struct(
        spec,
        &struct_name,
        schema,
        description,
        nested_schemas,
        symbols,
    )
    .map(Some)
}

pub(crate) fn operation_request_type_ident(operation_name: &str) -> Ident {
    Ident::new(
        &format!("{}Request", operation_name.to_upper_camel_case()),
        Span::call_site(),
    )
}

pub(crate) fn resolve_request_body<'a>(
    spec: &'a OpenAPI,
    request_body_ref: &'a openapiv3::ObjectOrReference<openapiv3::RequestBody>,
) -> Result<&'a openapiv3::RequestBody, String> {
    match request_body_ref {
        openapiv3::ObjectOrReference::Object(request_body) => Ok(request_body),
        openapiv3::ObjectOrReference::Ref {
            ref_path: reference,
            ..
        } => {
            let request_body_name = reference
                .strip_prefix("#/components/requestBodies/")
                .ok_or_else(|| format!("Unsupported request body reference: {reference}"))?;
            let components = spec
                .components
                .as_ref()
                .ok_or_else(|| "OpenAPI spec is missing components section".to_string())?;
            let target = components
                .request_bodies
                .get(request_body_name)
                .ok_or_else(|| {
                    format!("Referenced request body '{request_body_name}' not found")
                })?;

            resolve_request_body(spec, target)
        }
    }
}

pub(crate) fn request_body_schema(
    request_body: &openapiv3::RequestBody,
) -> Option<&openapiv3::ObjectOrReference<openapiv3::ObjectSchema>> {
    request_body
        .content
        .get("application/json")
        .or_else(|| request_body.content.values().next())
        .and_then(|media_type| media_type.schema.as_ref())
}

/// Creates response body representations for the operation's successful responses.
fn generate_response_body_structs(
    spec: &OpenAPI,
    operation_name: &str,
    operation_origin: &str,
    responses: Option<
        &std::collections::BTreeMap<String, openapiv3::ObjectOrReference<openapiv3::Response>>,
    >,
    nested_schemas: &mut Vec<TokenStream>,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<Option<Vec<TokenStream>>, String> {
    let mut response_structs = Vec::new();
    let mut success_responses = Vec::new();

    // Collect successful responses (2xx status codes)
    for (status, response_ref) in responses.into_iter().flatten() {
        if let Ok(code) = status.parse::<u16>() {
            if (200..300).contains(&code) {
                success_responses.push((code, response_ref));
            }
        }
    }

    // Sort responses by status code for deterministic output
    success_responses.sort_by_key(|(code, _)| *code);

    if success_responses.is_empty() {
        return Ok(None);
    }

    // If there's only one successful response
    if success_responses.len() == 1 {
        let (_, response_ref) = &success_responses[0];

        let response = match response_ref {
            openapiv3::ObjectOrReference::Object(r) => r,
            openapiv3::ObjectOrReference::Ref { .. } => {
                return Ok(None);
            }
        };

        if let Some(media_type) = crate::preferred_response_media_type(&response.content) {
            if let Some(schema_ref) = &media_type.schema {
                match schema_ref {
                    openapiv3::ObjectOrReference::Ref { .. } => {
                        // Already a schema reference
                        return Ok(None);
                    }
                    openapiv3::ObjectOrReference::Object(schema) => {
                        // Inline schema - generate a struct
                        let struct_name_str =
                            format!("{}Response", operation_name.to_upper_camel_case());

                        symbols.reserve(
                            struct_name_str.clone(),
                            format!("success response body for {operation_origin}"),
                        )?;

                        let struct_name = Ident::new(&struct_name_str, Span::call_site());

                        let description = response
                            .description
                            .as_deref()
                            .map(crate::schema::generate_doc_comment);

                        let body_tokens = generate_schema_struct(
                            spec,
                            &struct_name,
                            schema,
                            description,
                            nested_schemas,
                            symbols,
                        )?;

                        response_structs.push(body_tokens);
                    }
                }
            }
        }
    } else {
        // Multiple successful responses - create an enum
        let enum_name_str = format!("{}Response", operation_name.to_upper_camel_case());

        symbols.reserve(
            enum_name_str.clone(),
            format!("success response enum for {operation_origin}"),
        )?;
        let enum_name = Ident::new(&enum_name_str, Span::call_site());
        let mut variants = Vec::new();
        let mut variant_structs = Vec::new();

        for (status, response_ref) in success_responses.iter() {
            let variant_name_str = format!("Status{}", status);
            let variant_name = Ident::new(&variant_name_str, Span::call_site());

            let struct_name_str =
                format!("{}Response{}", operation_name.to_upper_camel_case(), status);
            let struct_name = Ident::new(&struct_name_str, Span::call_site());

            match response_ref {
                openapiv3::ObjectOrReference::Ref {
                    ref_path: reference,
                    ..
                } => {
                    // For referenced responses, use the schema name directly
                    let schema_name = reference
                        .strip_prefix("#/components/responses/")
                        .or_else(|| reference.strip_prefix("#/components/schemas/"))
                        .ok_or_else(|| format!("Invalid response reference: {}", reference))?;
                    let schema_type = Ident::new(schema_name, Span::call_site());

                    variants.push(quote! {
                        #variant_name(#schema_type)
                    });
                }
                openapiv3::ObjectOrReference::Object(response) => {
                    if let Some(media_type) =
                        crate::preferred_response_media_type(&response.content)
                    {
                        if let Some(schema_ref) = &media_type.schema {
                            match schema_ref {
                                openapiv3::ObjectOrReference::Ref {
                                    ref_path: reference,
                                    ..
                                } => {
                                    // Schema reference - use that type
                                    let schema_name = reference
                                        .strip_prefix("#/components/schemas/")
                                        .ok_or_else(|| {
                                            format!("Invalid schema reference: {}", reference)
                                        })?;
                                    let schema_type = Ident::new(schema_name, Span::call_site());

                                    variants.push(quote! {
                                        #variant_name(#schema_type)
                                    });
                                }
                                openapiv3::ObjectOrReference::Object(schema) => {
                                    // Inline schema - generate struct
                                    symbols.reserve(
                                        struct_name_str.clone(),
                                        format!(
                                            "{status} success response body for {operation_origin}"
                                        ),
                                    )?;
                                    let description = response
                                        .description
                                        .as_deref()
                                        .map(crate::schema::generate_doc_comment);

                                    let body_tokens = generate_schema_struct(
                                        spec,
                                        &struct_name,
                                        schema,
                                        description,
                                        nested_schemas,
                                        symbols,
                                    )?;

                                    variant_structs.push(body_tokens);

                                    variants.push(quote! {
                                        #variant_name(#struct_name)
                                    });
                                }
                            }
                        } else {
                            // No schema - unit variant
                            variants.push(quote! {
                                #variant_name
                            });
                        }
                    } else {
                        // No content - unit variant
                        variants.push(quote! {
                            #variant_name
                        });
                    }
                }
            }
        }

        if !variants.is_empty() {
            response_structs.extend(variant_structs);
            response_structs.push(quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                #[serde(untagged)]
                pub enum #enum_name {
                    #(#variants,)*
                }
            });
        }
    }

    if response_structs.is_empty() {
        Ok(None)
    } else {
        Ok(Some(response_structs))
    }
}

/// Converts an inline schema into a concrete struct or type alias and tracks nested schemas.
fn generate_schema_struct(
    spec: &OpenAPI,
    struct_name: &Ident,
    schema: &openapiv3::ObjectSchema,
    description: Option<TokenStream>,
    nested_schemas: &mut Vec<TokenStream>,
    symbols: &mut crate::symbol::SymbolRegistry,
) -> Result<TokenStream, String> {
    if crate::oas::schema_type(schema) == Some(openapiv3::SchemaType::Object) {
        let obj = schema;
        if crate::schema::should_emit_free_form_object_alias(
            &obj.properties,
            obj.additional_properties.as_ref(),
        ) {
            return Ok(quote! {
                #description
                pub type #struct_name = serde_json::Value;
            });
        }

        let struct_name_str = struct_name.to_string();

        // Collect nested inline schemas
        crate::schema::collect_nested_schemas_with_registry(
            spec,
            &struct_name_str,
            &obj.properties,
            nested_schemas,
            symbols,
        )?;

        let fields = crate::schema::generate_struct_fields(
            &struct_name_str,
            &obj.properties,
            &obj.required,
            obj.additional_properties.as_ref(),
        );

        let can_derive_default =
            crate::schema::can_fields_derive_default(&obj.properties, &obj.required);

        let derives = if can_derive_default {
            quote! { #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)] }
        } else {
            quote! { #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)] }
        };

        Ok(quote! {
            #description
            #derives
            pub struct #struct_name {
                #(#fields)*
            }
        })
    } else if !schema.all_of.is_empty() {
        if let Some((combined_properties, combined_required, combined_additional_properties)) =
            crate::schema::flatten_all_of_object(spec, &schema.all_of)?
        {
            let struct_name_str = struct_name.to_string();

            crate::schema::collect_nested_schemas_with_registry(
                spec,
                &struct_name_str,
                &combined_properties,
                nested_schemas,
                symbols,
            )?;

            let fields = crate::schema::generate_struct_fields(
                &struct_name_str,
                &combined_properties,
                &combined_required,
                combined_additional_properties.as_ref(),
            );

            let can_derive_default =
                crate::schema::can_fields_derive_default(&combined_properties, &combined_required);

            let derives = if can_derive_default {
                quote! { #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)] }
            } else {
                quote! { #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)] }
            };

            Ok(quote! {
                #description
                #derives
                pub struct #struct_name {
                    #(#fields)*
                }
            })
        } else {
            let dummy_ref = openapiv3::ObjectOrReference::Object(schema.clone());
            let base_type = crate::schema::infer_rust_type(true, false, None, &dummy_ref);
            Ok(quote! {
                #description
                pub type #struct_name = #base_type;
            })
        }
    } else {
        // For non-object types, create a type alias
        let dummy_ref = openapiv3::ObjectOrReference::Object(schema.clone());
        let base_type = crate::schema::infer_rust_type(true, false, None, &dummy_ref);
        Ok(quote! {
            #description
            pub type #struct_name = #base_type;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn parse_spec(value: serde_json::Value) -> OpenAPI {
        serde_json::from_value(value).expect("failed to parse OpenAPI fixture")
    }

    #[test]
    fn request_struct_uses_operation_name_for_referenced_schema() {
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
                                        "$ref": "#/components/schemas/ReaderCheckoutBody"
                                    }
                                }
                            }
                        },
                        "responses": { "204": { "description": "ok" } }
                    }
                }
            },
            "components": {
                "schemas": {
                    "ReaderCheckoutBody": {
                        "type": "object",
                        "properties": {
                            "description": { "type": "string" },
                            "inline_money": {
                                "title": "Money",
                                "type": "object",
                                "properties": {
                                    "value": { "type": "integer" }
                                }
                            },
                            "shared_money": {
                                "$ref": "#/components/schemas/Money"
                            }
                        },
                        "required": ["description"]
                    },
                    "Money": {
                        "type": "object",
                        "properties": {
                            "value": { "type": "integer" }
                        }
                    }
                }
            }
        }));

        let body_tokens = generate_operation_bodies(&spec, "Readers")
            .expect("request body generation should succeed");
        let body_code = crate::format_generated_code(body_tokens);
        assert!(body_code.contains("pub struct CreateCheckoutRequest {"));
        assert!(body_code.contains("pub struct CreateCheckoutRequestInlineMoney {"));
        assert!(body_code.contains("pub inline_money: Option<CreateCheckoutRequestInlineMoney>,"));
        assert!(body_code.contains("pub shared_money: Option<Money>,"));
        assert!(!body_code.contains("pub struct Money {"));

        let client_tokens =
            crate::generate_tag_client(&spec, "Readers").expect("client generation should succeed");
        let client_code = crate::format_generated_code(client_tokens);
        assert!(client_code.contains("body: CreateCheckoutRequest,"));
    }
}
