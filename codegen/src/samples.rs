use heck::ToSnakeCase;
use oas3::{
    Spec as OpenAPI,
    spec::{
        self as openapiv3, MediaType, ObjectOrReference as ReferenceOr, ObjectSchema, Operation,
        ParameterIn, RequestBody, Schema, SchemaType,
    },
};
use serde::Serialize;
use serde_json::{Map, Value};

const CATALOG_SCHEMA_VERSION: u8 = 1;
const SDK_CRATE: &str = "sumup";

/// Versioned JSON catalog consumed by the developer portal.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSampleCatalog {
    pub schema_version: u8,
    pub language: &'static str,
    pub sdk: CodeSampleSdk,
    #[serde(rename = "openAPIVersion")]
    pub open_api_version: String,
    pub samples: Vec<CodeSample>,
}

/// Rust crate used by every generated sample.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CodeSampleSdk {
    pub module: &'static str,
    pub version: String,
}

/// Complete Rust program for one OpenAPI operation example.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSample {
    pub id: String,
    pub operation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub http_method: String,
    pub path: String,
    pub sample: String,
}

#[derive(Debug, Clone)]
struct RequestExample {
    name: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    value: Option<Value>,
}

/// Generates a deterministic catalog from the same OpenAPI model used for SDK generation.
pub fn generate_code_samples(
    spec: &OpenAPI,
    sdk_version: impl Into<String>,
) -> Result<CodeSampleCatalog, String> {
    let mut samples = Vec::new();

    for (path, path_item) in spec.paths.iter().flatten() {
        for (http_method, operation) in crate::operations_for_path_item(path_item) {
            let operation_id = operation
                .operation_id
                .as_deref()
                .ok_or_else(|| {
                    format!(
                        "operation is missing operationId: {} {}",
                        http_method.to_ascii_uppercase(),
                        path
                    )
                })?
                .to_string();
            let tag = operation
                .tags
                .first()
                .ok_or_else(|| format!("operation {operation_id} is missing a tag"))?;

            for example in request_examples(spec, operation) {
                let id = match &example.name {
                    Some(name) => format!("{operation_id}.{name}"),
                    None => operation_id.clone(),
                };
                let summary = preferred_text(example.summary, operation.summary.clone());
                let description =
                    preferred_text(example.description, operation.description.clone());
                let sample =
                    render_program(spec, path_item, operation, tag, example.value.as_ref())?;

                samples.push(CodeSample {
                    id,
                    operation_id: operation_id.clone(),
                    example: example.name,
                    summary,
                    description,
                    http_method: http_method.to_ascii_uppercase(),
                    path: path.clone(),
                    sample,
                });
            }
        }
    }

    samples.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(CodeSampleCatalog {
        schema_version: CATALOG_SCHEMA_VERSION,
        language: "rust",
        sdk: CodeSampleSdk {
            module: SDK_CRATE,
            version: sdk_version.into(),
        },
        open_api_version: spec.info.version.trim().to_string(),
        samples,
    })
}

fn preferred_text(preferred: Option<String>, fallback: Option<String>) -> Option<String> {
    preferred
        .filter(|value| !value.trim().is_empty())
        .or_else(|| fallback.filter(|value| !value.trim().is_empty()))
        .map(|value| value.trim().to_string())
}

fn request_examples(spec: &OpenAPI, operation: &Operation) -> Vec<RequestExample> {
    let Some(request_body) = operation
        .request_body
        .as_ref()
        .and_then(|body| resolve_request_body(spec, body))
    else {
        return vec![empty_example()];
    };
    let Some(media_type) = preferred_request_media_type(&request_body.content) else {
        return vec![empty_example()];
    };

    if media_type
        .examples
        .as_ref()
        .is_some_and(|examples| !examples.is_empty())
    {
        let mut examples = media_type
            .examples(spec)
            .into_iter()
            .map(|(name, example)| RequestExample {
                name: Some(name),
                summary: example.summary,
                description: example.description,
                value: example.value,
            })
            .collect::<Vec<_>>();
        examples.sort_by(|left, right| left.name.cmp(&right.name));
        if !examples.is_empty() {
            return examples;
        }
    }

    let value = media_type
        .examples(spec)
        .into_values()
        .next()
        .and_then(|example| example.value)
        .or_else(|| {
            media_type
                .schema
                .as_ref()
                .and_then(|schema| schema_value(spec, schema, 0))
        });
    vec![RequestExample {
        value,
        ..empty_example()
    }]
}

fn empty_example() -> RequestExample {
    RequestExample {
        name: None,
        summary: None,
        description: None,
        value: None,
    }
}

fn resolve_request_body<'a>(
    spec: &'a OpenAPI,
    request_body: &'a ReferenceOr<RequestBody>,
) -> Option<&'a RequestBody> {
    match request_body {
        ReferenceOr::Object(body) => Some(body),
        ReferenceOr::Ref {
            ref_path: reference,
            ..
        } => reference
            .strip_prefix("#/components/requestBodies/")
            .and_then(|name| spec.components.as_ref()?.request_bodies.get(name))
            .and_then(|body| resolve_request_body(spec, body)),
    }
}

fn preferred_request_media_type(
    content: &std::collections::BTreeMap<String, MediaType>,
) -> Option<&MediaType> {
    content
        .get("application/json")
        .or_else(|| content.values().next())
}

fn render_program(
    spec: &OpenAPI,
    path_item: &openapiv3::PathItem,
    operation: &Operation,
    tag: &str,
    example: Option<&Value>,
) -> Result<String, String> {
    let operation_name = crate::operation_name(operation);
    let method = operation_name.to_snake_case();
    let resource = tag.to_snake_case();
    let mut arguments = Vec::new();

    for parameter in path_item.parameters.iter().chain(&operation.parameters) {
        let ReferenceOr::Object(parameter) = parameter else {
            continue;
        };
        if parameter.location == ParameterIn::Path && parameter.required.unwrap_or(false) {
            arguments.push(format!("{:?}", path_parameter_value(&parameter.name)));
        }
    }

    let body = operation
        .request_body
        .as_ref()
        .and_then(|body| resolve_request_body(spec, body))
        .and_then(|body| {
            preferred_request_media_type(&body.content)
                .and_then(|media_type| media_type.schema.as_ref())
                .map(|schema| (body.required.unwrap_or(false), schema))
        });
    let mut body_declaration = String::new();
    if let Some((required, schema)) = body {
        let value = example
            .cloned()
            .or_else(|| schema_value(spec, schema, 0))
            .unwrap_or_else(|| Value::Object(Map::new()));
        let json = serde_json::to_string_pretty(&value)
            .map_err(|error| format!("serialize request body example: {error}"))?;
        body_declaration = format!(
            "    let body = serde_json::from_value(serde_json::json!({json}))\n        .expect(\"build request body\");\n"
        );
        arguments.push(if required { "body" } else { "Some(body)" }.to_string());
    }

    if operation
        .parameters
        .iter()
        .any(|parameter| matches!(parameter, ReferenceOr::Object(parameter) if parameter.location == ParameterIn::Query))
    {
        arguments.push("Default::default()".to_string());
    }

    let arguments = arguments.join(", ");
    let source = format!(
        "use sumup::{{Authorization, Client}};\n\n#[tokio::main]\nasync fn main() {{\n    let client = Client::default()\n        .with_authorization(Authorization::api_key(\"sup_sk_test_...\"));\n{body_declaration}\n    let response = client\n        .{resource}()\n        .{method}({arguments})\n        .await\n        .expect(\"{method} request failed\");\n\n    println!(\"{{response:#?}}\");\n}}\n"
    );
    let syntax = syn::parse_file(&source).map_err(|error| {
        format!(
            "format sample for {}: {error}\n{source}",
            operation.operation_id.as_deref().unwrap_or("unknown")
        )
    })?;
    Ok(prettyplease::unparse(&syntax))
}

fn path_parameter_value(name: &str) -> &str {
    match name {
        "checkout_id" => "CHECKOUT_ID",
        "customer_id" => "CUSTOMER_ID",
        "member_id" => "MEMBER_ID",
        "merchant_code" => "MERCHANT_CODE",
        "person_id" => "PERSON_ID",
        "reader_id" => "READER_ID",
        "role_id" => "ROLE_ID",
        "token" => "PAYMENT_INSTRUMENT_TOKEN",
        "transaction_id" => "TRANSACTION_ID",
        _ => "RESOURCE_ID",
    }
}

fn schema_value(
    spec: &OpenAPI,
    schema_ref: &ReferenceOr<ObjectSchema>,
    depth: usize,
) -> Option<Value> {
    if depth > 20 {
        return None;
    }
    match schema_ref {
        ReferenceOr::Ref {
            ref_path: reference,
            ..
        } => reference
            .strip_prefix("#/components/schemas/")
            .and_then(|name| spec.components.as_ref()?.schemas.get(name))
            .and_then(|schema| schema_value(spec, schema, depth + 1)),
        ReferenceOr::Object(schema) => {
            if let Some(example) = crate::oas::schema_example(schema) {
                return Some(example.clone());
            }
            if let Some(default) = &schema.default {
                return Some(default.clone());
            }
            schema_object_value(spec, schema, depth + 1)
        }
    }
}

fn schema_object_value(spec: &OpenAPI, schema: &ObjectSchema, depth: usize) -> Option<Value> {
    if let Some(value) = schema.enum_values.iter().find(|value| !value.is_null()) {
        return Some(value.clone());
    }
    if !schema.all_of.is_empty() {
        let mut combined = Map::new();
        for part in &schema.all_of {
            if let Some(Value::Object(values)) = schema_value(spec, part, depth + 1) {
                combined.extend(values);
            }
        }
        return Some(Value::Object(combined));
    }
    if let Some(value) = schema
        .one_of
        .first()
        .and_then(|schema| schema_value(spec, schema, depth + 1))
    {
        return Some(value);
    }
    if let Some(value) = schema
        .any_of
        .first()
        .and_then(|schema| schema_value(spec, schema, depth + 1))
    {
        return Some(value);
    }
    match crate::oas::schema_type(schema) {
        Some(SchemaType::String) => Some(Value::String(
            string_example(schema.format.as_deref()).to_string(),
        )),
        Some(SchemaType::Number | SchemaType::Integer) => schema
            .minimum
            .clone()
            .map(Value::Number)
            .or_else(|| Some(Value::from(1))),
        Some(SchemaType::Boolean) => Some(Value::Bool(true)),
        Some(SchemaType::Array) => {
            let item = schema.items.as_deref().and_then(|item| match item {
                Schema::Object(item) => schema_value(spec, item, depth + 1),
                Schema::Boolean(_) => None,
            });
            Some(item.into_iter().collect())
        }
        Some(SchemaType::Object) | None if !schema.properties.is_empty() => Some(object_value(
            spec,
            &schema.properties,
            &schema.required,
            depth + 1,
        )),
        Some(SchemaType::Null) => Some(Value::Null),
        _ => Some(Value::Object(Map::new())),
    }
}

fn object_value(
    spec: &OpenAPI,
    properties: &std::collections::BTreeMap<String, ReferenceOr<ObjectSchema>>,
    required: &[String],
    depth: usize,
) -> Value {
    let mut values = Map::new();
    for name in required {
        if let Some(value) = properties
            .get(name)
            .and_then(|schema| schema_value(spec, schema, depth + 1))
        {
            values.insert(name.clone(), value);
        }
    }
    Value::Object(values)
}

fn string_example(format: Option<&str>) -> &'static str {
    match format {
        Some("date-time") => "2024-01-01T00:00:00Z",
        Some("date") => "2024-01-01",
        Some("uri") => "https://example.com",
        _ => "example",
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use super::*;

    fn catalog() -> CodeSampleCatalog {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root = manifest_dir.parent().expect("workspace root");
        let spec: OpenAPI = serde_json::from_reader(
            File::open(root.join("openapi.json")).expect("open OpenAPI document"),
        )
        .expect("parse OpenAPI document");
        generate_code_samples(&spec, "test").expect("generate samples")
    }

    #[test]
    fn generates_deterministic_portal_catalog() {
        let generated = catalog();
        assert_eq!(generated.schema_version, 1);
        assert_eq!(generated.language, "rust");
        assert_eq!(generated.sdk.module, "sumup");
        assert_eq!(generated.sdk.version, "test");
        assert!(!generated.open_api_version.is_empty());
        assert!(!generated.samples.is_empty());
        assert!(
            generated
                .samples
                .windows(2)
                .all(|samples| samples[0].id < samples[1].id)
        );
        assert_eq!(generated, catalog());

        for sample in &generated.samples {
            syn::parse_file(&sample.sample)
                .unwrap_or_else(|error| panic!("invalid sample {}: {error}", sample.id));
        }

        let encoded = serde_json::to_value(&generated.samples[0]).expect("encode sample");
        assert!(encoded.get("sample").is_some());
        assert!(encoded.get("source").is_none());
    }

    #[test]
    fn preserves_whole_request_example() {
        let spec: OpenAPI = serde_json::from_value(serde_json::json!({
            "openapi": "3.0.3",
            "info": { "title": "Samples", "version": "1.0.0" },
            "paths": {
                "/samples": {
                    "post": {
                        "operationId": "CreateSample",
                        "tags": ["Samples"],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "required": ["selected", "missing"],
                                        "properties": {
                                            "selected": { "type": "string", "example": "property-selected" },
                                            "missing": { "type": "string", "example": "property-missing" }
                                        }
                                    },
                                    "example": { "selected": "request-selected" }
                                }
                            }
                        },
                        "responses": { "204": { "description": "Created" } }
                    }
                }
            }
        }))
        .expect("parse OpenAPI document");

        let generated = generate_code_samples(&spec, "test").expect("generate samples");
        let sample = &generated.samples[0].sample;
        assert!(sample.contains("request-selected"));
        assert!(!sample.contains("property-"));
    }
}
