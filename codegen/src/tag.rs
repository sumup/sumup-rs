use std::collections::{HashMap, HashSet};

use oas3::{
    spec::{ObjectOrReference, ObjectSchema, ParameterIn, Schema},
    Spec,
};

/// Holds the schemas associated with a single OpenAPI tag.
pub struct TagSchemas {
    pub all_schemas: HashSet<String>,
    pub error_schemas: HashSet<String>,
}

/// Groups schemas by tag while tracking shared schema usage.
pub struct SchemasByTag {
    pub tag_schemas: HashMap<String, TagSchemas>,
    pub common_schemas: HashSet<String>,
    pub common_error_schemas: HashSet<String>,
}

/// Collects schemas referenced by each tag and identifies shared/common schemas.
pub fn collect_schemas_by_tag(spec: &Spec) -> Result<SchemasByTag, String> {
    let mut tag_schemas: HashMap<String, TagSchemas> = HashMap::new();

    for path_item in spec.paths.iter().flat_map(|paths| paths.values()) {
        for (_http_method, operation) in crate::operations_for_path_item(path_item) {
            let tags = if operation.tags.is_empty() {
                vec!["Untagged".to_string()]
            } else {
                operation.tags.clone()
            };

            for tag in tags {
                let tag_data = tag_schemas.entry(tag).or_insert_with(|| TagSchemas {
                    all_schemas: HashSet::new(),
                    error_schemas: HashSet::new(),
                });

                if let Some(request_body_ref) = &operation.request_body {
                    let request_body = crate::body::resolve_request_body(spec, request_body_ref)?;
                    for media_type in request_body.content.values() {
                        if let Some(schema_ref) = &media_type.schema {
                            collect_schema_reference(schema_ref, &mut tag_data.all_schemas);
                        }
                    }
                }

                for (status, response_ref) in operation.responses.iter().flatten() {
                    let ObjectOrReference::Object(response) = response_ref else {
                        continue;
                    };
                    let is_error = status.parse::<u16>().is_ok_and(|status| status >= 400);

                    if let Some(media_type) =
                        crate::preferred_response_media_type(&response.content)
                    {
                        if let Some(schema_ref) = &media_type.schema {
                            collect_schema_reference(schema_ref, &mut tag_data.all_schemas);
                            if is_error {
                                collect_top_level_schema(schema_ref, &mut tag_data.error_schemas);
                            }
                        }
                    }
                }

                for param_ref in &operation.parameters {
                    let ObjectOrReference::Object(parameter) = param_ref else {
                        continue;
                    };
                    if matches!(
                        parameter.location,
                        ParameterIn::Query
                            | ParameterIn::Header
                            | ParameterIn::Path
                            | ParameterIn::Cookie
                    ) {
                        if let Some(schema_ref) = &parameter.schema {
                            collect_schema_reference(schema_ref, &mut tag_data.all_schemas);
                        }
                    }
                }
            }
        }
    }

    let Some(components) = &spec.components else {
        return Ok(SchemasByTag {
            tag_schemas,
            common_schemas: HashSet::new(),
            common_error_schemas: HashSet::new(),
        });
    };

    for tag_data in tag_schemas.values_mut() {
        let mut to_process: Vec<String> = tag_data.all_schemas.iter().cloned().collect();
        let mut processed = HashSet::new();

        while let Some(schema_name) = to_process.pop() {
            if !processed.insert(schema_name.clone()) {
                continue;
            }

            if let Some(schema_ref) = components.schemas.get(&schema_name) {
                let mut referenced = HashSet::new();
                collect_schema_reference(schema_ref, &mut referenced);
                for referenced_schema in referenced {
                    if !processed.contains(&referenced_schema) {
                        tag_data.all_schemas.insert(referenced_schema.clone());
                        to_process.push(referenced_schema);
                    }
                }
            }
        }
    }

    let common_schemas = identify_common_schemas(&tag_schemas);
    let common_error_schemas = tag_schemas
        .values()
        .flat_map(|tag| &tag.error_schemas)
        .filter(|schema| common_schemas.contains(*schema))
        .cloned()
        .collect();

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

fn identify_common_schemas(tag_schemas: &HashMap<String, TagSchemas>) -> HashSet<String> {
    let mut schema_tag_count: HashMap<String, usize> = HashMap::new();
    for tag_data in tag_schemas.values() {
        for schema in &tag_data.all_schemas {
            *schema_tag_count.entry(schema.clone()).or_default() += 1;
        }
    }
    schema_tag_count
        .into_iter()
        .filter_map(|(schema, count)| (count > 1).then_some(schema))
        .collect()
}

fn collect_top_level_schema(
    schema_ref: &ObjectOrReference<ObjectSchema>,
    schemas: &mut HashSet<String>,
) {
    if let ObjectOrReference::Ref { ref_path, .. } = schema_ref {
        if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
            schemas.insert(schema_name.to_string());
        }
    }
}

fn collect_schema_reference(
    schema_ref: &ObjectOrReference<ObjectSchema>,
    schemas: &mut HashSet<String>,
) {
    match schema_ref {
        ObjectOrReference::Ref { ref_path, .. } => {
            if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
                schemas.insert(schema_name.to_string());
            }
        }
        ObjectOrReference::Object(schema) => collect_schema_references(schema, schemas),
    }
}

fn collect_schema_document(schema: &Schema, schemas: &mut HashSet<String>) {
    if let Schema::Object(schema_ref) = schema {
        collect_schema_reference(schema_ref, schemas);
    }
}

fn collect_schema_references(schema: &ObjectSchema, schemas: &mut HashSet<String>) {
    for property in schema.properties.values() {
        collect_schema_reference(property, schemas);
    }
    if let Some(additional_properties) = &schema.additional_properties {
        collect_schema_document(additional_properties, schemas);
    }
    if let Some(items) = &schema.items {
        collect_schema_document(items, schemas);
    }
    for nested in schema
        .one_of
        .iter()
        .chain(&schema.any_of)
        .chain(&schema.all_of)
        .chain(&schema.prefix_items)
    {
        collect_schema_reference(nested, schemas);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_common_and_untagged_schemas() {
        let spec: Spec = serde_json::from_value(serde_json::json!({
            "openapi": "3.1.0",
            "info": { "title": "test", "version": "1.0.0" },
            "paths": {
                "/a": { "get": { "operationId": "getA", "tags": ["TagA"], "responses": {
                    "200": { "description": "ok", "content": { "application/json": {
                        "schema": { "$ref": "#/components/schemas/Shared" }
                    } } }
                } } },
                "/b": { "get": { "operationId": "getB", "tags": ["TagB"], "responses": {
                    "200": { "description": "ok", "content": { "application/json": {
                        "schema": { "$ref": "#/components/schemas/Shared" }
                    } } }
                } } },
                "/c": { "get": { "operationId": "getC", "responses": {
                    "200": { "description": "ok", "content": { "application/json": {
                        "schema": { "$ref": "#/components/schemas/OnlyUntagged" }
                    } } }
                } } }
            },
            "components": { "schemas": {
                "Shared": { "type": "object" },
                "OnlyUntagged": { "type": "object" }
            } }
        }))
        .expect("parse fixture");

        let schemas = collect_schemas_by_tag(&spec).expect("collect schemas");
        assert!(schemas.common_schemas.contains("Shared"));
        assert!(schemas.tag_schemas["Untagged"]
            .all_schemas
            .contains("OnlyUntagged"));
    }
}
