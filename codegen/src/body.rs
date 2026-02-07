use heck::ToUpperCamelCase;
use openapiv3::OpenAPI;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Generates request and response body structs for every operation under the given tag.
pub fn generate_operation_bodies(spec: &OpenAPI, tag: &str) -> Result<TokenStream, String> {
    let mut body_structs = Vec::new();
    let mut generated_names = std::collections::HashSet::new();
    let mut nested_schemas = Vec::new();

    // Collect all operations with this tag and sort them by path and method
    let mut operations_to_process = Vec::new();

    // Iterate through all paths and operations
    for (path, path_item) in &spec.paths.paths {
        let path_item = match path_item {
            openapiv3::ReferenceOr::Item(item) => item,
            openapiv3::ReferenceOr::Reference { .. } => continue,
        };

        let operations = vec![
            ("delete", path_item.delete.as_ref()),
            ("get", path_item.get.as_ref()),
            ("patch", path_item.patch.as_ref()),
            ("post", path_item.post.as_ref()),
            ("put", path_item.put.as_ref()),
        ];

        for (http_method, operation) in operations {
            if let Some(op) = operation {
                // Check if this operation has the current tag
                if !op.tags.contains(&tag.to_string()) {
                    continue;
                }

                let operation_id = op.operation_id.as_ref().ok_or_else(|| {
                    format!("Operation {} {} missing operation_id", http_method, path)
                })?;
                let operation_name = crate::operation_name(op);

                operations_to_process.push((
                    path.clone(),
                    http_method,
                    operation_id.clone(),
                    operation_name,
                    op,
                ));
            }
        }
    }

    // Sort operations alphabetically by path, then method, then operation name
    operations_to_process.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.cmp(b.1))
            .then_with(|| a.3.cmp(&b.3))
    });

    for (_path, _http_method, _operation_id, operation_name, op) in operations_to_process {
        // Generate query params struct if present
        if let Some(params_struct) =
            generate_query_params_struct(&operation_name, op, &mut generated_names)?
        {
            body_structs.push(params_struct);
        }

        // Generate request body struct if present
        if let Some(request_body_ref) = &op.request_body {
            if let Some(body_struct) = generate_request_body_struct(
                spec,
                &operation_name,
                request_body_ref,
                &mut generated_names,
                &mut nested_schemas,
            )? {
                body_structs.push(body_struct);
            }
        }

        // Generate response body struct(s)
        if let Some(response_structs) = generate_response_body_structs(
            spec,
            &operation_name,
            &op.responses,
            &mut generated_names,
            &mut nested_schemas,
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
    operation_name: &str,
    operation: &openapiv3::Operation,
    generated_names: &mut std::collections::HashSet<String>,
) -> Result<Option<TokenStream>, String> {
    use heck::ToSnakeCase;

    // Collect query parameters
    let mut query_params = Vec::new();

    for param_ref in &operation.parameters {
        let param = match param_ref {
            openapiv3::ReferenceOr::Item(p) => p,
            openapiv3::ReferenceOr::Reference { .. } => continue,
        };

        if let openapiv3::Parameter::Query { parameter_data, .. } = param {
            query_params.push(parameter_data);
        }
    }

    if query_params.is_empty() {
        return Ok(None);
    }

    let params_struct_name = format!("{}Params", operation_name.to_upper_camel_case());

    // Check if already generated
    if generated_names.contains(&params_struct_name) {
        return Ok(None);
    }
    generated_names.insert(params_struct_name.clone());

    let struct_name = Ident::new(&params_struct_name, Span::call_site());

    // Generate fields
    let mut fields = Vec::new();
    for param_data in query_params {
        let field_name = crate::schema::make_rust_field_ident(&param_data.name.to_snake_case());
        let original_name = &param_data.name;

        // Determine field type based on schema
        let (field_type, is_nullable) =
            if let openapiv3::ParameterSchemaOrContent::Schema(schema_ref) = &param_data.format {
                infer_param_type(schema_ref, param_data.required)
            } else if param_data.required {
                (quote! { String }, false)
            } else {
                (quote! { Option<String> }, false)
            };

        let rename_attr = if original_name != &field_name.to_string() {
            quote! { #[serde(rename = #original_name)] }
        } else {
            quote! {}
        };

        let skip_attr = if !param_data.required {
            if is_nullable {
                quote! { #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::nullable::deserialize")] }
            } else {
                quote! { #[serde(skip_serializing_if = "Option::is_none")] }
            }
        } else {
            quote! {}
        };

        let description = match &param_data.format {
            openapiv3::ParameterSchemaOrContent::Schema(openapiv3::ReferenceOr::Item(schema)) => {
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
        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct #struct_name {
            #(#fields,)*
        }
    }))
}

/// Infers the Rust type for a query parameter schema and reports whether it is nullable.
/// Returns a tuple of (field_type, is_nullable).
fn infer_param_type(
    schema_ref: &openapiv3::ReferenceOr<openapiv3::Schema>,
    required: bool,
) -> (TokenStream, bool) {
    let (base_type, is_nullable) = match schema_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            let type_name = reference.split('/').next_back().unwrap_or("Unknown");
            let type_ident = Ident::new(&type_name.to_upper_camel_case(), Span::call_site());
            (quote! { #type_ident }, false)
        }
        openapiv3::ReferenceOr::Item(schema) => {
            let is_nullable = schema.schema_data.nullable;
            let base = match &schema.schema_kind {
                openapiv3::SchemaKind::Type(openapiv3::Type::String(string_type)) => {
                    match &string_type.format {
                        openapiv3::VariantOrUnknownOrEmpty::Item(
                            openapiv3::StringFormat::DateTime,
                        ) => {
                            quote! { crate::datetime::DateTime }
                        }
                        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => {
                            quote! { crate::datetime::Date }
                        }
                        openapiv3::VariantOrUnknownOrEmpty::Item(
                            openapiv3::StringFormat::Password,
                        ) => {
                            quote! { crate::secret::Secret }
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
                                let type_name =
                                    reference.split('/').next_back().unwrap_or("Unknown");
                                let type_ident =
                                    Ident::new(&type_name.to_upper_camel_case(), Span::call_site());
                                quote! { #type_ident }
                            }
                            openapiv3::ReferenceOr::Item(inner_schema) => {
                                match &inner_schema.schema_kind {
                                    openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                        string_type,
                                    )) => match &string_type.format {
                                        openapiv3::VariantOrUnknownOrEmpty::Item(
                                            openapiv3::StringFormat::DateTime,
                                        ) => {
                                            quote! { crate::datetime::DateTime }
                                        }
                                        openapiv3::VariantOrUnknownOrEmpty::Item(
                                            openapiv3::StringFormat::Date,
                                        ) => {
                                            quote! { crate::datetime::Date }
                                        }
                                        _ => quote! { String },
                                    },
                                    openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_)) => {
                                        quote! { i64 }
                                    }
                                    openapiv3::SchemaKind::Type(openapiv3::Type::Number(_)) => {
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

/// Emits a request body struct for inline request schemas referenced by the operation.
fn generate_request_body_struct(
    spec: &OpenAPI,
    operation_name: &str,
    request_body_ref: &openapiv3::ReferenceOr<openapiv3::RequestBody>,
    generated_names: &mut std::collections::HashSet<String>,
    nested_schemas: &mut Vec<TokenStream>,
) -> Result<Option<TokenStream>, String> {
    let request_body = match request_body_ref {
        openapiv3::ReferenceOr::Item(rb) => rb,
        openapiv3::ReferenceOr::Reference { .. } => {
            // If it's a reference, it should already be in the schemas
            return Ok(None);
        }
    };

    // Look for application/json content
    let media_type = request_body
        .content
        .get("application/json")
        .or_else(|| request_body.content.values().next());

    if let Some(mt) = media_type {
        if let Some(schema_ref) = &mt.schema {
            match schema_ref {
                openapiv3::ReferenceOr::Reference { .. } => {
                    // Already a schema reference, will be handled elsewhere
                    return Ok(None);
                }
                openapiv3::ReferenceOr::Item(schema) => {
                    // Inline schema - generate a struct
                    let struct_name_str = format!("{}Body", operation_name.to_upper_camel_case());

                    if !generated_names.insert(struct_name_str.clone()) {
                        // Already generated
                        return Ok(None);
                    }

                    let struct_name = Ident::new(&struct_name_str, Span::call_site());

                    let description = request_body
                        .description
                        .as_ref()
                        .map(|d| crate::schema::generate_doc_comment(d));

                    let body_tokens = generate_schema_struct(
                        spec,
                        &struct_name,
                        schema,
                        description,
                        nested_schemas,
                    )?;

                    return Ok(Some(body_tokens));
                }
            }
        }
    }

    Ok(None)
}

/// Creates response body representations for the operation's successful responses.
fn generate_response_body_structs(
    spec: &OpenAPI,
    operation_name: &str,
    responses: &openapiv3::Responses,
    generated_names: &mut std::collections::HashSet<String>,
    nested_schemas: &mut Vec<TokenStream>,
) -> Result<Option<Vec<TokenStream>>, String> {
    let mut response_structs = Vec::new();
    let mut success_responses = Vec::new();

    // Collect successful responses (2xx status codes)
    for (status, response_ref) in &responses.responses {
        if let openapiv3::StatusCode::Code(code) = status {
            if (200..300).contains(code) {
                success_responses.push((*code, response_ref));
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
            openapiv3::ReferenceOr::Item(r) => r,
            openapiv3::ReferenceOr::Reference { .. } => {
                return Ok(None);
            }
        };

        if let Some(media_type) = response
            .content
            .get("application/json")
            .or_else(|| response.content.values().next())
        {
            if let Some(schema_ref) = &media_type.schema {
                match schema_ref {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        // Already a schema reference
                        return Ok(None);
                    }
                    openapiv3::ReferenceOr::Item(schema) => {
                        // Inline schema - generate a struct
                        let struct_name_str =
                            format!("{}Response", operation_name.to_upper_camel_case());

                        if !generated_names.insert(struct_name_str.clone()) {
                            return Ok(None);
                        }

                        let struct_name = Ident::new(&struct_name_str, Span::call_site());

                        let description = Some(&response.description)
                            .map(|d| crate::schema::generate_doc_comment(d.as_str()));

                        let body_tokens = generate_schema_struct(
                            spec,
                            &struct_name,
                            schema,
                            description,
                            nested_schemas,
                        )?;

                        response_structs.push(body_tokens);
                    }
                }
            }
        }
    } else {
        // Multiple successful responses - create an enum
        let enum_name_str = format!("{}Response", operation_name.to_upper_camel_case());

        if generated_names.insert(enum_name_str.clone()) {
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
                    openapiv3::ReferenceOr::Reference { reference } => {
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
                    openapiv3::ReferenceOr::Item(response) => {
                        if let Some(media_type) = response
                            .content
                            .get("application/json")
                            .or_else(|| response.content.values().next())
                        {
                            if let Some(schema_ref) = &media_type.schema {
                                match schema_ref {
                                    openapiv3::ReferenceOr::Reference { reference } => {
                                        // Schema reference - use that type
                                        let schema_name = reference
                                            .strip_prefix("#/components/schemas/")
                                            .ok_or_else(|| {
                                                format!("Invalid schema reference: {}", reference)
                                            })?;
                                        let schema_type =
                                            Ident::new(schema_name, Span::call_site());

                                        variants.push(quote! {
                                            #variant_name(#schema_type)
                                        });
                                    }
                                    openapiv3::ReferenceOr::Item(schema) => {
                                        // Inline schema - generate struct
                                        if generated_names.insert(struct_name_str.clone()) {
                                            let description =
                                                Some(&response.description).map(|d| {
                                                    crate::schema::generate_doc_comment(d.as_str())
                                                });

                                            let body_tokens = generate_schema_struct(
                                                spec,
                                                &struct_name,
                                                schema,
                                                description,
                                                nested_schemas,
                                            )?;

                                            variant_structs.push(body_tokens);
                                        }

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
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                    #[serde(untagged)]
                    pub enum #enum_name {
                        #(#variants,)*
                    }
                });
            }
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
    schema: &openapiv3::Schema,
    description: Option<TokenStream>,
    nested_schemas: &mut Vec<TokenStream>,
) -> Result<TokenStream, String> {
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) => {
            let struct_name_str = struct_name.to_string();

            // Collect nested inline schemas
            crate::schema::collect_nested_schemas(
                spec,
                &struct_name_str,
                &obj.properties,
                nested_schemas,
            )?;

            let fields = crate::schema::generate_struct_fields(
                &struct_name_str,
                &obj.properties,
                &obj.required,
            );

            let can_derive_default =
                crate::schema::can_fields_derive_default(&obj.properties, &obj.required);

            let derives = if can_derive_default {
                quote! { #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)] }
            } else {
                quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
            };

            Ok(quote! {
                #description
                #derives
                pub struct #struct_name {
                    #(#fields)*
                }
            })
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            if let Some((combined_properties, combined_required)) =
                crate::schema::flatten_all_of_object(spec, all_of)?
            {
                let struct_name_str = struct_name.to_string();

                crate::schema::collect_nested_schemas(
                    spec,
                    &struct_name_str,
                    &combined_properties,
                    nested_schemas,
                )?;

                let fields = crate::schema::generate_struct_fields(
                    &struct_name_str,
                    &combined_properties,
                    &combined_required,
                );

                let can_derive_default = crate::schema::can_fields_derive_default(
                    &combined_properties,
                    &combined_required,
                );

                let derives = if can_derive_default {
                    quote! { #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)] }
                } else {
                    quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
                };

                Ok(quote! {
                    #description
                    #derives
                    pub struct #struct_name {
                        #(#fields)*
                    }
                })
            } else {
                let dummy_ref = openapiv3::ReferenceOr::Item(Box::new(schema.clone()));
                let base_type = crate::schema::infer_rust_type(
                    &schema.schema_kind,
                    true,
                    false,
                    None,
                    &dummy_ref,
                );
                Ok(quote! {
                    #description
                    pub type #struct_name = #base_type;
                })
            }
        }
        _ => {
            // For non-object types, create a type alias
            let dummy_ref = openapiv3::ReferenceOr::Item(Box::new(schema.clone()));
            let base_type =
                crate::schema::infer_rust_type(&schema.schema_kind, true, false, None, &dummy_ref);
            Ok(quote! {
                #description
                pub type #struct_name = #base_type;
            })
        }
    }
}
