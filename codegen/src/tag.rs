use openapiv3::OpenAPI;
use std::collections::{HashMap, HashSet};

/// Holds the schemas associated with a single OpenAPI tag.
pub struct TagSchemas {
    pub all_schemas: HashSet<String>,
    pub error_schemas: HashSet<String>,
    pub deprecation_notice: Option<String>,
}

/// Groups schemas by tag while tracking shared schema usage.
pub struct SchemasByTag {
    pub tag_schemas: HashMap<String, TagSchemas>,
    pub common_schemas: HashSet<String>,
    pub common_error_schemas: HashSet<String>,
}

/// Collects schemas referenced by each tag and identifies shared/common schemas.
pub fn collect_schemas_by_tag(spec: &OpenAPI) -> Result<SchemasByTag, String> {
    let mut tag_schemas: HashMap<String, TagSchemas> = HashMap::new();

    // Extract deprecation notices from tag definitions
    let mut tag_deprecations: HashMap<String, String> = HashMap::new();
    for tag in &spec.tags {
        if let Some(serde_json::Value::String(notice)) = tag.extensions.get("x-deprecation-notice")
        {
            tag_deprecations.insert(tag.name.clone(), notice.clone());
        }
    }

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
            // Get tags for this operation
            let tags = if operation.tags.is_empty() {
                vec!["Untagged".to_string()]
            } else {
                operation.tags.clone()
            };

            for tag in tags {
                let tag_data = tag_schemas
                    .entry(tag.clone())
                    .or_insert_with(|| TagSchemas {
                        all_schemas: HashSet::new(),
                        error_schemas: HashSet::new(),
                        deprecation_notice: tag_deprecations.get(&tag).cloned(),
                    });

                // Collect schemas from request body
                if let Some(request_body_ref) = &operation.request_body {
                    let request_body = match request_body_ref {
                        openapiv3::ReferenceOr::Item(rb) => rb,
                        openapiv3::ReferenceOr::Reference { .. } => continue,
                    };

                    for (_content_type, media_type) in &request_body.content {
                        if let Some(schema_ref) = &media_type.schema {
                            collect_schema_references_unboxed(
                                schema_ref,
                                &mut tag_data.all_schemas,
                            );
                        }
                    }
                }

                // Collect schemas from responses
                for (status, response_ref) in &operation.responses.responses {
                    let response = match response_ref {
                        openapiv3::ReferenceOr::Item(r) => r,
                        openapiv3::ReferenceOr::Reference { .. } => continue,
                    };

                    // Determine if this is an error response (400+)
                    let is_error = match status {
                        openapiv3::StatusCode::Code(code) => *code >= 400,
                        _ => false,
                    };

                    for (_content_type, media_type) in &response.content {
                        if let Some(schema_ref) = &media_type.schema {
                            // Collect to all_schemas first
                            collect_schema_references_unboxed(
                                schema_ref,
                                &mut tag_data.all_schemas,
                            );

                            // If error response, also collect top-level schema to error_schemas
                            if is_error {
                                collect_top_level_schema(schema_ref, &mut tag_data.error_schemas);
                            }
                        }
                    }
                }

                // Collect schemas from parameters
                for param_ref in &operation.parameters {
                    let param = match param_ref {
                        openapiv3::ReferenceOr::Item(p) => p,
                        openapiv3::ReferenceOr::Reference { .. } => continue,
                    };

                    match &param {
                        openapiv3::Parameter::Query { parameter_data, .. }
                        | openapiv3::Parameter::Header { parameter_data, .. }
                        | openapiv3::Parameter::Path { parameter_data, .. }
                        | openapiv3::Parameter::Cookie { parameter_data, .. } => {
                            if let openapiv3::ParameterSchemaOrContent::Schema(schema_ref) =
                                &parameter_data.format
                            {
                                collect_schema_references_unboxed(
                                    schema_ref,
                                    &mut tag_data.all_schemas,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // Now expand each tag's schema set to include all transitively referenced schemas
    let all_schemas = match &spec.components {
        Some(components) => &components.schemas,
        None => {
            return Ok(SchemasByTag {
                tag_schemas,
                common_schemas: HashSet::new(),
                common_error_schemas: HashSet::new(),
            })
        }
    };

    for tag_data in tag_schemas.values_mut() {
        let mut to_process: Vec<String> = tag_data.all_schemas.iter().cloned().collect();
        let mut processed = HashSet::new();

        while let Some(schema_name) = to_process.pop() {
            if processed.contains(&schema_name) {
                continue;
            }
            processed.insert(schema_name.clone());

            if let Some(schema_ref) = all_schemas.get(&schema_name) {
                let mut referenced = HashSet::new();
                match schema_ref {
                    openapiv3::ReferenceOr::Item(schema) => {
                        collect_schema_references_from_schema(schema, &mut referenced);
                    }
                    openapiv3::ReferenceOr::Reference { .. } => {}
                }

                for ref_schema in referenced {
                    if !processed.contains(&ref_schema) {
                        tag_data.all_schemas.insert(ref_schema.clone());
                        to_process.push(ref_schema);
                    }
                }
            }
        }
    }

    // Identify schemas used by multiple tags (common schemas)
    let common_schemas = identify_common_schemas(&tag_schemas);

    // Identify which common schemas are error schemas
    let mut common_error_schemas = HashSet::new();
    for tag_data in tag_schemas.values() {
        for error_schema in &tag_data.error_schemas {
            if common_schemas.contains(error_schema) {
                common_error_schemas.insert(error_schema.clone());
            }
        }
    }

    // Remove common schemas from individual tags
    for tag_data in tag_schemas.values_mut() {
        tag_data.all_schemas = tag_data
            .all_schemas
            .difference(&common_schemas)
            .cloned()
            .collect();
        tag_data.error_schemas = tag_data
            .error_schemas
            .difference(&common_schemas)
            .cloned()
            .collect();
    }

    Ok(SchemasByTag {
        tag_schemas,
        common_schemas,
        common_error_schemas,
    })
}

/// Finds schemas that appear under more than one tag.
fn identify_common_schemas(tag_schemas: &HashMap<String, TagSchemas>) -> HashSet<String> {
    let mut schema_tag_count: HashMap<String, usize> = HashMap::new();

    // Count how many tags each schema appears in
    for tag_data in tag_schemas.values() {
        for schema in &tag_data.all_schemas {
            *schema_tag_count.entry(schema.clone()).or_insert(0) += 1;
        }
    }

    // Schemas that appear in more than one tag are common
    schema_tag_count
        .into_iter()
        .filter_map(|(schema, count)| if count > 1 { Some(schema) } else { None })
        .collect()
}

/// Records the top-level schema name when the reference points into components.
fn collect_top_level_schema(
    schema_ref: &openapiv3::ReferenceOr<openapiv3::Schema>,
    schemas: &mut HashSet<String>,
) {
    // Only collect if it's a reference (top-level schema)
    if let openapiv3::ReferenceOr::Reference { reference } = schema_ref {
        if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
            schemas.insert(schema_name.to_string());
        }
    }
}

/// Adds schema names referenced by boxed schema values to the accumulator.
fn collect_schema_references_boxed(
    schema_ref: &openapiv3::ReferenceOr<Box<openapiv3::Schema>>,
    schemas: &mut HashSet<String>,
) {
    match schema_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                schemas.insert(schema_name.to_string());
            }
        }
        openapiv3::ReferenceOr::Item(schema) => {
            collect_schema_references_from_schema(schema, schemas);
        }
    }
}

/// Adds schema names referenced by inline schema values to the accumulator.
fn collect_schema_references_unboxed(
    schema_ref: &openapiv3::ReferenceOr<openapiv3::Schema>,
    schemas: &mut HashSet<String>,
) {
    match schema_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                schemas.insert(schema_name.to_string());
            }
        }
        openapiv3::ReferenceOr::Item(schema) => {
            collect_schema_references_from_schema(schema, schemas);
        }
    }
}

/// Traverses a schema and collects every referenced schema name.
fn collect_schema_references_from_schema(
    schema: &openapiv3::Schema,
    schemas: &mut HashSet<String>,
) {
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(t) => match t {
            openapiv3::Type::Object(obj) => {
                for (_name, prop_ref) in &obj.properties {
                    collect_schema_references_boxed(prop_ref, schemas);
                }
                if let Some(openapiv3::AdditionalProperties::Schema(schema_ref)) =
                    &obj.additional_properties
                {
                    match schema_ref.as_ref() {
                        openapiv3::ReferenceOr::Reference { reference } => {
                            if let Some(schema_name) =
                                reference.strip_prefix("#/components/schemas/")
                            {
                                schemas.insert(schema_name.to_string());
                            }
                        }
                        openapiv3::ReferenceOr::Item(s) => {
                            collect_schema_references_from_schema(s, schemas);
                        }
                    }
                }
            }
            openapiv3::Type::Array(arr) => {
                if let Some(items) = &arr.items {
                    collect_schema_references_boxed(items, schemas);
                }
            }
            _ => {}
        },
        openapiv3::SchemaKind::OneOf { one_of } => {
            for schema_ref in one_of {
                collect_schema_references_unboxed(schema_ref, schemas);
            }
        }
        openapiv3::SchemaKind::AnyOf { any_of } => {
            for schema_ref in any_of {
                collect_schema_references_unboxed(schema_ref, schemas);
            }
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            for schema_ref in all_of {
                collect_schema_references_unboxed(schema_ref, schemas);
            }
        }
        _ => {}
    }
}
