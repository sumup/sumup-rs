use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use openapiv3::OpenAPI;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

struct OperationResponse {
    return_type: TokenStream,
    response_handling: TokenStream,
    error_type: TokenStream,
    error_definition: Option<TokenStream>,
}

struct ErrorGeneration {
    match_arms: Vec<TokenStream>,
    body_type: TokenStream,
    body_definition: Option<TokenStream>,
}

#[derive(Clone)]
enum BodyKind {
    Schema(Ident),
    Text,
}

struct ErrorEntry {
    status_code: u16,
    status_const: TokenStream,
    body_kind: BodyKind,
}

impl BodyKind {
    fn key(&self) -> String {
        match self {
            BodyKind::Schema(ident) => ident.to_string(),
            BodyKind::Text => "String".to_string(),
        }
    }
}

pub struct GeneratedClientMethods {
    pub methods: Vec<TokenStream>,
    pub extra_items: Vec<TokenStream>,
}

struct GeneratedOperation {
    method: TokenStream,
    extra_items: Vec<TokenStream>,
}

/// Generates documentation attributes from a description string.
/// Splits multi-line descriptions into separate doc attributes for better formatting.
fn generate_doc_comment(description: &str) -> TokenStream {
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

/// Produces client method implementations for all operations labeled with the supplied tag.
pub fn generate_client_methods(
    spec: &OpenAPI,
    tag: &str,
) -> Result<GeneratedClientMethods, String> {
    let mut methods = Vec::new();
    let mut extra_items = Vec::new();

    // Collect all operations with this tag and sort them
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

        for (http_method, operation) in operations.into_iter() {
            if let Some(op) = operation {
                // Check if this operation has the current tag
                if !op.tags.contains(&tag.to_string()) {
                    continue;
                }

                operations_to_process.push((
                    path.clone(),
                    http_method,
                    op,
                    path_item.parameters.clone(),
                ));
            }
        }
    }

    // Sort operations alphabetically by path, then method
    operations_to_process.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(b.1)));

    for (path, http_method, operation, path_parameters) in operations_to_process {
        let generated =
            generate_operation_method(spec, &path, http_method, operation, &path_parameters)?;
        methods.push(generated.method);
        extra_items.extend(generated.extra_items);
    }

    Ok(GeneratedClientMethods {
        methods,
        extra_items,
    })
}

/// Generates a concrete client method for the provided HTTP operation and path.
fn generate_operation_method(
    spec: &OpenAPI,
    path: &str,
    http_method: &str,
    operation: &openapiv3::Operation,
    path_parameters: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<GeneratedOperation, String> {
    // Determine method name
    let method_name = if let Some(codegen) = operation.extensions.get("x-codegen") {
        if let Some(codegen_obj) = codegen.as_object() {
            if let Some(method_name_value) = codegen_obj.get("method_name") {
                if let Some(name_str) = method_name_value.as_str() {
                    name_str.to_snake_case()
                } else {
                    operation
                        .operation_id
                        .as_ref()
                        .map(|s| s.to_snake_case())
                        .unwrap_or_else(|| "unknown".to_string())
                }
            } else {
                operation
                    .operation_id
                    .as_ref()
                    .map(|s| s.to_snake_case())
                    .unwrap_or_else(|| "unknown".to_string())
            }
        } else {
            operation
                .operation_id
                .as_ref()
                .map(|s| s.to_snake_case())
                .unwrap_or_else(|| "unknown".to_string())
        }
    } else {
        operation
            .operation_id
            .as_ref()
            .map(|s| s.to_snake_case())
            .unwrap_or_else(|| "unknown".to_string())
    };

    let method_ident = Ident::new(&method_name, Span::call_site());

    // Collect path parameters from both path-level and operation-level
    let mut path_params = Vec::new();
    let mut path_param_names = Vec::new();

    // Process path-level parameters first
    for param_ref in path_parameters {
        let param = match param_ref {
            openapiv3::ReferenceOr::Item(p) => p,
            openapiv3::ReferenceOr::Reference { .. } => continue,
        };

        if let openapiv3::Parameter::Path { parameter_data, .. } = param {
            if parameter_data.required {
                let param_name = parameter_data.name.to_snake_case();
                let param_ident = Ident::new(&param_name, Span::call_site());

                path_params.push(quote! { #param_ident: impl Into<String> });
                path_param_names.push((parameter_data.name.clone(), param_ident));
            }
        }
    }

    // Then process operation-level parameters
    for param_ref in &operation.parameters {
        let param = match param_ref {
            openapiv3::ReferenceOr::Item(p) => p,
            openapiv3::ReferenceOr::Reference { .. } => continue,
        };

        if let openapiv3::Parameter::Path { parameter_data, .. } = param {
            if parameter_data.required {
                let param_name = parameter_data.name.to_snake_case();
                let param_ident = Ident::new(&param_name, Span::call_site());

                path_params.push(quote! { #param_ident: impl Into<String> });
                path_param_names.push((parameter_data.name.clone(), param_ident));
            }
        }
    }

    // Handle request body parameter
    let (body_param, has_optional_body, has_body) = if let Some(request_body_ref) =
        &operation.request_body
    {
        match request_body_ref {
            openapiv3::ReferenceOr::Item(rb) => {
                // Check if there's actually a schema in the body
                let has_schema = if let Some(media_type) = rb
                    .content
                    .get("application/json")
                    .or_else(|| rb.content.values().next())
                {
                    media_type.schema.is_some()
                } else {
                    false
                };

                // Only generate body parameter if we have a schema
                if has_schema {
                    // Determine the concrete body type name
                    let operation_id = operation
                        .operation_id
                        .as_ref()
                        .ok_or_else(|| "Operation missing operation_id".to_string())?;

                    use heck::ToUpperCamelCase;

                    // Check if there's a referenced schema in the body
                    let body_type = if let Some(media_type) = rb
                        .content
                        .get("application/json")
                        .or_else(|| rb.content.values().next())
                    {
                        if let Some(schema_ref) = &media_type.schema {
                            match schema_ref {
                                openapiv3::ReferenceOr::Reference { reference } => {
                                    // Use the referenced schema name
                                    let schema_name = reference
                                        .strip_prefix("#/components/schemas/")
                                        .ok_or_else(|| {
                                            format!("Invalid schema reference: {}", reference)
                                        })?;
                                    Ident::new(schema_name, Span::call_site())
                                }
                                openapiv3::ReferenceOr::Item(_) => {
                                    // Inline schema - use generated Body type
                                    let body_type_name =
                                        format!("{}Body", operation_id.to_upper_camel_case());
                                    Ident::new(&body_type_name, Span::call_site())
                                }
                            }
                        } else {
                            // No schema - shouldn't happen but fall back to Body name
                            let body_type_name =
                                format!("{}Body", operation_id.to_upper_camel_case());
                            Ident::new(&body_type_name, Span::call_site())
                        }
                    } else {
                        // No JSON content - fall back to Body name
                        let body_type_name = format!("{}Body", operation_id.to_upper_camel_case());
                        Ident::new(&body_type_name, Span::call_site())
                    };

                    let body_param = if rb.required {
                        quote! { body: #body_type }
                    } else {
                        quote! { body: Option<#body_type> }
                    };

                    (Some(body_param), !rb.required, true)
                } else {
                    // No schema - don't add body parameter
                    (None, false, false)
                }
            }
            openapiv3::ReferenceOr::Reference { reference } => {
                // Extract the schema name from the reference
                let schema_name = reference
                    .strip_prefix("#/components/requestBodies/")
                    .or_else(|| reference.strip_prefix("#/components/schemas/"))
                    .ok_or_else(|| format!("Invalid reference: {}", reference))?;

                let body_type = Ident::new(schema_name, Span::call_site());

                // For referenced bodies, we can't easily determine if they're required
                // Default to required
                let body_param = quote! { body: #body_type };

                (Some(body_param), false, true)
            }
        }
    } else {
        (None, false, false)
    };

    // Collect query parameters
    let mut query_params = Vec::new();

    for param_ref in &operation.parameters {
        let param = match param_ref {
            openapiv3::ReferenceOr::Item(p) => p,
            openapiv3::ReferenceOr::Reference { .. } => continue,
        };

        if let openapiv3::Parameter::Query { parameter_data, .. } = param {
            query_params.push(parameter_data.clone());
        }
    }

    // Add body parameter after path parameters
    if let Some(body_param) = body_param {
        path_params.push(body_param);
    }

    // Add query params parameter if there are query parameters
    let has_query_params = !query_params.is_empty();
    if has_query_params {
        let operation_id = operation
            .operation_id
            .as_ref()
            .ok_or_else(|| "Operation missing operation_id".to_string())?;
        let params_type_name = format!("{}Params", operation_id.to_upper_camel_case());
        let params_type = Ident::new(&params_type_name, Span::call_site());
        path_params.push(quote! { params: #params_type });
    }

    // Build the path with parameter substitution using format!
    let path_construction = if path_param_names.is_empty() {
        quote! {
            let path = #path;
        }
    } else {
        // Convert path template to format string and collect arguments
        let mut format_str = path.to_string();
        let mut format_args = Vec::new();

        for (original_name, param_ident) in &path_param_names {
            let placeholder = format!("{{{}}}", original_name);
            format_str = format_str.replace(&placeholder, "{}");
            format_args.push(quote! { #param_ident.into() });
        }

        quote! {
            let path = format!(#format_str, #(#format_args),*);
        }
    };

    // Generate HTTP method call
    let http_method_ident = Ident::new(http_method, Span::call_site());

    // Determine response type and error handling
    let OperationResponse {
        return_type,
        response_handling,
        error_type,
        error_definition,
    } = generate_response_handling(operation, spec)?;

    // Add doc comment if available - combine summary and description
    let doc_comment = match (&operation.summary, &operation.description) {
        (Some(summary), Some(description)) if summary != description => {
            // Both available and different - combine them
            let combined = format!("{}\n\n{}", summary.trim(), description.trim());
            Some(generate_doc_comment(&combined))
        }
        (Some(summary), _) => {
            // Only summary, or both are the same
            Some(generate_doc_comment(summary))
        }
        (None, Some(description)) => {
            // Only description
            Some(generate_doc_comment(description))
        }
        (None, None) => None,
    };

    // Build query parameter additions
    let query_additions = if has_query_params {
        let mut query_field_additions = Vec::new();
        for query_param in &query_params {
            let field_name = Ident::new(&query_param.name.to_snake_case(), Span::call_site());
            let param_name = &query_param.name;

            if query_param.required {
                query_field_additions.push(quote! {
                    request = request.query(&[(#param_name, &params.#field_name)]);
                });
            } else {
                query_field_additions.push(quote! {
                    if let Some(ref value) = params.#field_name {
                        request = request.query(&[(#param_name, value)]);
                    }
                });
            }
        }
        quote! { #(#query_field_additions)* }
    } else {
        quote! {}
    };

    // Build the request with or without body
    let request_send = if has_body {
        if has_optional_body {
            // Optional body - use a let mut and conditionally add json
            quote! {
                let mut request = self.client.http_client().#http_method_ident(&url)
                    .header("User-Agent", crate::version::user_agent())
                    .timeout(self.client.timeout());
                if let Some(token) = self.client.authorization_token() {
                    request = request.header("Authorization", format!("Bearer {}", token));
                }
                #query_additions
                if let Some(body) = body {
                    request = request.json(&body);
                }
                let response = request.send().await?;
            }
        } else {
            // Required body
            quote! {
                let mut request = self.client.http_client()
                    .#http_method_ident(&url)
                    .header("User-Agent", crate::version::user_agent())
                    .timeout(self.client.timeout())
                    .json(&body);
                if let Some(token) = self.client.authorization_token() {
                    request = request.header("Authorization", format!("Bearer {}", token));
                }
                #query_additions
                let response = request.send().await?;
            }
        }
    } else {
        // No body
        quote! {
            let mut request = self.client.http_client()
                .#http_method_ident(&url)
                .header("User-Agent", crate::version::user_agent())
                .timeout(self.client.timeout());
            if let Some(token) = self.client.authorization_token() {
                request = request.header("Authorization", format!("Bearer {}", token));
            }
            #query_additions
            let response = request.send().await?;
        }
    };

    let method_tokens = quote! {
        #doc_comment
        pub async fn #method_ident(&self, #(#path_params),*) -> crate::error::SdkResult<#return_type, #error_type> {
            #path_construction
            let url = format!("{}{}", self.client.base_url(), path);
            #request_send
            #response_handling
        }
    };

    let mut extra_items = Vec::new();
    if let Some(definition) = error_definition {
        extra_items.push(definition);
    }

    Ok(GeneratedOperation {
        method: method_tokens,
        extra_items,
    })
}

/// Builds response handling logic and determines the method's return type.
fn generate_response_handling(
    operation: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<OperationResponse, String> {
    use heck::ToUpperCamelCase;

    let operation_id = operation
        .operation_id
        .as_ref()
        .ok_or_else(|| "Operation missing operation_id".to_string())?;

    let mut success_responses = Vec::new();
    let mut error_responses = Vec::new();

    for (status_code, response_ref) in &operation.responses.responses {
        match status_code {
            openapiv3::StatusCode::Code(code) if (200..300).contains(code) => {
                success_responses.push((*code, response_ref));
            }
            openapiv3::StatusCode::Code(code) if *code >= 400 => {
                error_responses.push((*code, response_ref));
            }
            _ => {}
        }
    }

    success_responses.sort_by_key(|(code, _)| *code);
    error_responses.sort_by_key(|(code, _)| *code);

    let return_type = if success_responses.is_empty() {
        quote! { () }
    } else if success_responses.len() == 1 {
        let (_, response_ref) = success_responses[0];
        get_response_type_for_single(operation_id, response_ref)?
    } else {
        let response_type_name = format!("{}Response", operation_id.to_upper_camel_case());
        let response_type = Ident::new(&response_type_name, Span::call_site());
        quote! { #response_type }
    };

    let error_generation = generate_error_handling(operation_id, &error_responses, spec)?;

    let response_handling = if success_responses.is_empty() {
        generate_no_success_response_handling(&error_generation)?
    } else if success_responses.len() == 1 {
        generate_single_response_handling(operation_id, &success_responses[0], &error_generation)?
    } else {
        generate_multi_response_handling(operation_id, &success_responses, &error_generation)?
    };

    Ok(OperationResponse {
        return_type,
        response_handling,
        error_type: error_generation.body_type,
        error_definition: error_generation.body_definition,
    })
}

/// Determines the concrete return type for a single successful response variant.
fn get_response_type_for_single(
    operation_id: &str,
    response_ref: &openapiv3::ReferenceOr<openapiv3::Response>,
) -> Result<TokenStream, String> {
    use heck::ToUpperCamelCase;

    match response_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            // Referenced response - extract schema name
            let schema_name = reference
                .strip_prefix("#/components/responses/")
                .or_else(|| reference.strip_prefix("#/components/schemas/"))
                .ok_or_else(|| format!("Invalid response reference: {}", reference))?;
            let type_ident = Ident::new(schema_name, Span::call_site());
            Ok(quote! { #type_ident })
        }
        openapiv3::ReferenceOr::Item(response) => {
            // Check if response has content with a schema
            if let Some(media_type) = response
                .content
                .get("application/json")
                .or_else(|| response.content.values().next())
            {
                if let Some(schema_ref) = &media_type.schema {
                    match schema_ref {
                        openapiv3::ReferenceOr::Reference { reference } => {
                            // Schema reference - use that
                            let schema_name =
                                reference.strip_prefix("#/components/schemas/").ok_or_else(
                                    || format!("Invalid schema reference: {}", reference),
                                )?;
                            let type_ident = Ident::new(schema_name, Span::call_site());
                            Ok(quote! { #type_ident })
                        }
                        openapiv3::ReferenceOr::Item(_) => {
                            // Inline schema - use generated Response type
                            let response_type_name =
                                format!("{}Response", operation_id.to_upper_camel_case());
                            let response_type = Ident::new(&response_type_name, Span::call_site());
                            Ok(quote! { #response_type })
                        }
                    }
                } else {
                    // No schema - return unit
                    Ok(quote! { () })
                }
            } else {
                // No content - return unit
                Ok(quote! { () })
            }
        }
    }
}

/// Converts a numeric status code to an equivalent `reqwest::StatusCode` token when available.
fn status_code_to_constant(status: u16) -> TokenStream {
    match status {
        200 => quote! { reqwest::StatusCode::OK },
        201 => quote! { reqwest::StatusCode::CREATED },
        202 => quote! { reqwest::StatusCode::ACCEPTED },
        203 => quote! { reqwest::StatusCode::NON_AUTHORITATIVE_INFORMATION },
        204 => quote! { reqwest::StatusCode::NO_CONTENT },
        205 => quote! { reqwest::StatusCode::RESET_CONTENT },
        206 => quote! { reqwest::StatusCode::PARTIAL_CONTENT },
        207 => quote! { reqwest::StatusCode::MULTI_STATUS },
        208 => quote! { reqwest::StatusCode::ALREADY_REPORTED },
        226 => quote! { reqwest::StatusCode::IM_USED },
        400 => quote! { reqwest::StatusCode::BAD_REQUEST },
        401 => quote! { reqwest::StatusCode::UNAUTHORIZED },
        402 => quote! { reqwest::StatusCode::PAYMENT_REQUIRED },
        403 => quote! { reqwest::StatusCode::FORBIDDEN },
        404 => quote! { reqwest::StatusCode::NOT_FOUND },
        405 => quote! { reqwest::StatusCode::METHOD_NOT_ALLOWED },
        406 => quote! { reqwest::StatusCode::NOT_ACCEPTABLE },
        407 => quote! { reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED },
        408 => quote! { reqwest::StatusCode::REQUEST_TIMEOUT },
        409 => quote! { reqwest::StatusCode::CONFLICT },
        410 => quote! { reqwest::StatusCode::GONE },
        411 => quote! { reqwest::StatusCode::LENGTH_REQUIRED },
        412 => quote! { reqwest::StatusCode::PRECONDITION_FAILED },
        413 => quote! { reqwest::StatusCode::PAYLOAD_TOO_LARGE },
        414 => quote! { reqwest::StatusCode::URI_TOO_LONG },
        415 => quote! { reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE },
        416 => quote! { reqwest::StatusCode::RANGE_NOT_SATISFIABLE },
        417 => quote! { reqwest::StatusCode::EXPECTATION_FAILED },
        418 => quote! { reqwest::StatusCode::IM_A_TEAPOT },
        421 => quote! { reqwest::StatusCode::MISDIRECTED_REQUEST },
        422 => quote! { reqwest::StatusCode::UNPROCESSABLE_ENTITY },
        423 => quote! { reqwest::StatusCode::LOCKED },
        424 => quote! { reqwest::StatusCode::FAILED_DEPENDENCY },
        426 => quote! { reqwest::StatusCode::UPGRADE_REQUIRED },
        428 => quote! { reqwest::StatusCode::PRECONDITION_REQUIRED },
        429 => quote! { reqwest::StatusCode::TOO_MANY_REQUESTS },
        431 => quote! { reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE },
        451 => quote! { reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS },
        500 => quote! { reqwest::StatusCode::INTERNAL_SERVER_ERROR },
        501 => quote! { reqwest::StatusCode::NOT_IMPLEMENTED },
        502 => quote! { reqwest::StatusCode::BAD_GATEWAY },
        503 => quote! { reqwest::StatusCode::SERVICE_UNAVAILABLE },
        504 => quote! { reqwest::StatusCode::GATEWAY_TIMEOUT },
        505 => quote! { reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED },
        506 => quote! { reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES },
        507 => quote! { reqwest::StatusCode::INSUFFICIENT_STORAGE },
        508 => quote! { reqwest::StatusCode::LOOP_DETECTED },
        510 => quote! { reqwest::StatusCode::NOT_EXTENDED },
        511 => quote! { reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED },
        _ => {
            // Fall back to numeric for uncommon status codes
            quote! { #status }
        }
    }
}

/// Builds error handling match arms and endpoint-specific error metadata.
fn generate_error_handling(
    operation_id: &str,
    error_responses: &[(u16, &openapiv3::ReferenceOr<openapiv3::Response>)],
    spec: &openapiv3::OpenAPI,
) -> Result<ErrorGeneration, String> {
    if error_responses.is_empty() {
        return Ok(ErrorGeneration {
            match_arms: Vec::new(),
            body_type: quote! { String },
            body_definition: None,
        });
    }

    use heck::ToUpperCamelCase;

    let mut entries = Vec::new();

    for (status_code, response_ref) in error_responses {
        let status_const = status_code_to_constant(*status_code);
        let body_kind = match extract_error_schema_ident(response_ref, spec) {
            Some(ident) => BodyKind::Schema(ident),
            None => BodyKind::Text,
        };
        entries.push(ErrorEntry {
            status_code: *status_code,
            status_const,
            body_kind,
        });
    }

    let mut unique_keys = Vec::new();
    let mut unique_kinds = Vec::new();
    for entry in &entries {
        let key = entry.body_kind.key();
        if !unique_keys.contains(&key) {
            unique_keys.push(key.clone());
            unique_kinds.push(entry.body_kind.clone());
        }
    }

    let body_definition;
    let body_type;
    let mut match_arms = Vec::new();

    if unique_kinds.len() == 1 {
        let kind = &unique_kinds[0];
        body_type = match kind {
            BodyKind::Schema(ident) => quote! { #ident },
            BodyKind::Text => quote! { String },
        };

        for entry in entries {
            let status_const = entry.status_const;
            match entry.body_kind {
                BodyKind::Schema(ident) => {
                    match_arms.push(quote! {
                        #status_const => {
                            let body: #ident = response.json().await?;
                            Err(crate::error::SdkError::api_parsed(#status_const, body))
                        }
                    });
                }
                BodyKind::Text => {
                    match_arms.push(quote! {
                        #status_const => {
                            let body = response.text().await?;
                            Err(crate::error::SdkError::api_parsed(#status_const, body))
                        }
                    });
                }
            }
        }

        body_definition = None;
    } else {
        let enum_name = format!("{}ErrorBody", operation_id.to_upper_camel_case());
        let enum_ident = Ident::new(&enum_name, Span::call_site());

        let mut variant_defs = Vec::new();

        for entry in entries {
            let status_const = entry.status_const;
            let variant_name = format!("Status{}", entry.status_code);
            let variant_ident = Ident::new(&variant_name, Span::call_site());

            match entry.body_kind {
                BodyKind::Schema(ident) => {
                    variant_defs.push(quote! { #variant_ident(#ident), });
                    match_arms.push(quote! {
                        #status_const => {
                            let body: #ident = response.json().await?;
                            Err(crate::error::SdkError::api_parsed(
                                #status_const,
                                #enum_ident::#variant_ident(body),
                            ))
                        }
                    });
                }
                BodyKind::Text => {
                    variant_defs.push(quote! { #variant_ident(String), });
                    match_arms.push(quote! {
                        #status_const => {
                            let body = response.text().await?;
                            Err(crate::error::SdkError::api_parsed(
                                #status_const,
                                #enum_ident::#variant_ident(body),
                            ))
                        }
                    });
                }
            }
        }

        body_definition = Some(quote! {
            #[derive(Debug)]
            pub enum #enum_ident {
                #(#variant_defs)*
            }
        });
        body_type = quote! { #enum_ident };
    }

    Ok(ErrorGeneration {
        match_arms,
        body_type,
        body_definition,
    })
}

fn extract_error_schema_ident(
    response_ref: &openapiv3::ReferenceOr<openapiv3::Response>,
    spec: &openapiv3::OpenAPI,
) -> Option<Ident> {
    match response_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                Some(Ident::new(schema_name, Span::call_site()))
            } else if let Some(response_name) = reference.strip_prefix("#/components/responses/") {
                let components = spec.components.as_ref()?;
                let response_entry = components.responses.get(response_name)?;
                match response_entry {
                    openapiv3::ReferenceOr::Item(response) => {
                        extract_schema_from_response(response)
                    }
                    openapiv3::ReferenceOr::Reference { reference } => {
                        let nested_ref = openapiv3::ReferenceOr::Reference {
                            reference: reference.clone(),
                        };
                        extract_error_schema_ident(&nested_ref, spec)
                    }
                }
            } else {
                None
            }
        }
        openapiv3::ReferenceOr::Item(response) => extract_schema_from_response(response),
    }
}

fn extract_schema_from_response(response: &openapiv3::Response) -> Option<Ident> {
    let media_type = response
        .content
        .get("application/json")
        .or_else(|| response.content.values().next())?;
    let schema_ref = media_type.schema.as_ref()?;
    match schema_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            let schema_name = reference.strip_prefix("#/components/schemas/")?;
            Some(Ident::new(schema_name, Span::call_site()))
        }
        openapiv3::ReferenceOr::Item(_) => None,
    }
}

/// Handles operations lacking explicit success responses by only validating the status.
fn generate_no_success_response_handling(
    error_generation: &ErrorGeneration,
) -> Result<TokenStream, String> {
    let error_arms = &error_generation.match_arms;

    Ok(quote! {
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            match status {
                #(#error_arms)*
                _ => {
                    let body = response.text().await?;
                    Err(crate::error::SdkError::api_raw(status, body))
                }
            }
        }
    })
}

/// Produces response handling logic for operations with one successful status code.
fn generate_single_response_handling(
    operation_id: &str,
    (status, response_ref): &(u16, &openapiv3::ReferenceOr<openapiv3::Response>),
    error_generation: &ErrorGeneration,
) -> Result<TokenStream, String> {
    let status_code = *status;
    let status_const = status_code_to_constant(status_code);

    let has_content = match response_ref {
        openapiv3::ReferenceOr::Reference { .. } => true,
        openapiv3::ReferenceOr::Item(response) => response
            .content
            .get("application/json")
            .and_then(|mt| mt.schema.as_ref())
            .is_some(),
    };

    let error_arms = &error_generation.match_arms;

    if has_content {
        let response_type = get_response_type_for_single(operation_id, response_ref)?;
        Ok(quote! {
            let status = response.status();
            match status {
                #status_const => {
                    let data: #response_type = response.json().await?;
                    Ok(data)
                }
                #(#error_arms)*
                _ => {
                    let body = response.text().await?;
                    Err(crate::error::SdkError::api_raw(status, body))
                }
            }
        })
    } else {
        Ok(quote! {
            let status = response.status();
            match status {
                #status_const => Ok(()),
                #(#error_arms)*
                _ => {
                    let body = response.text().await?;
                    Err(crate::error::SdkError::api_raw(status, body))
                }
            }
        })
    }
}

/// Emits response handling that deserializes into an enum for multi-status operations.
fn generate_multi_response_handling(
    operation_id: &str,
    success_responses: &[(u16, &openapiv3::ReferenceOr<openapiv3::Response>)],
    error_generation: &ErrorGeneration,
) -> Result<TokenStream, String> {
    use heck::ToUpperCamelCase;

    let response_type_name = format!("{}Response", operation_id.to_upper_camel_case());
    let response_type = Ident::new(&response_type_name, Span::call_site());

    let mut match_arms = Vec::new();
    for (status_code, response_ref) in success_responses {
        let status_const = status_code_to_constant(*status_code);
        let variant_name = format!("Status{}", status_code);
        let variant = Ident::new(&variant_name, Span::call_site());

        // Determine the actual type for this response
        let (has_content, inner_type) = match response_ref {
            openapiv3::ReferenceOr::Reference { reference } => {
                // Referenced response - extract schema name
                let schema_name = reference
                    .strip_prefix("#/components/responses/")
                    .or_else(|| reference.strip_prefix("#/components/schemas/"))
                    .ok_or_else(|| format!("Invalid response reference: {}", reference))?;
                let type_ident = Ident::new(schema_name, Span::call_site());
                (true, quote! { #type_ident })
            }
            openapiv3::ReferenceOr::Item(resp) => {
                // Check if response has content with a schema
                if let Some(media_type) = resp
                    .content
                    .get("application/json")
                    .or_else(|| resp.content.values().next())
                {
                    if let Some(schema_ref) = &media_type.schema {
                        match schema_ref {
                            openapiv3::ReferenceOr::Reference { reference } => {
                                // Schema reference - use that
                                let schema_name =
                                    reference.strip_prefix("#/components/schemas/").ok_or_else(
                                        || format!("Invalid schema reference: {}", reference),
                                    )?;
                                let type_ident = Ident::new(schema_name, Span::call_site());
                                (true, quote! { #type_ident })
                            }
                            openapiv3::ReferenceOr::Item(_) => {
                                // Inline schema - use generated Response{status} type
                                let inner_type_name = format!(
                                    "{}Response{}",
                                    operation_id.to_upper_camel_case(),
                                    status_code
                                );
                                let type_ident = Ident::new(&inner_type_name, Span::call_site());
                                (true, quote! { #type_ident })
                            }
                        }
                    } else {
                        (false, quote! {})
                    }
                } else {
                    (false, quote! {})
                }
            }
        };

        if has_content {
            match_arms.push(quote! {
                #status_const => {
                    let data: #inner_type = response.json().await?;
                    Ok(#response_type::#variant(data))
                }
            });
        } else {
            match_arms.push(quote! {
                #status_const => Ok(#response_type::#variant)
            });
        }
    }

    let error_arms = &error_generation.match_arms;

    Ok(quote! {
        let status = response.status();
        match status {
            #(#match_arms)*
            #(#error_arms)*
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    })
}
