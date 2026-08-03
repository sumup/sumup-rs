use heck::ToSnakeCase;
use openapiv3::{
    Example, MediaType, OpenAPI, Operation, Parameter, ReferenceOr, RequestBody, Schema,
    SchemaKind, Type, VariantOrUnknownOrEmpty,
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

    for (path, path_item_ref) in &spec.paths.paths {
        let path_item = match path_item_ref {
            ReferenceOr::Item(path_item) => path_item,
            ReferenceOr::Reference { .. } => continue,
        };

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

    if !media_type.examples.is_empty() {
        let mut examples = media_type
            .examples
            .iter()
            .filter_map(|(name, example)| {
                resolve_example(spec, example).map(|example| RequestExample {
                    name: Some(name.clone()),
                    summary: example.summary.clone(),
                    description: example.description.clone(),
                    value: example.value.clone(),
                })
            })
            .collect::<Vec<_>>();
        examples.sort_by(|left, right| left.name.cmp(&right.name));
        if !examples.is_empty() {
            return examples;
        }
    }

    let value = media_type.example.clone().or_else(|| {
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
        ReferenceOr::Item(body) => Some(body),
        ReferenceOr::Reference { reference } => reference
            .strip_prefix("#/components/requestBodies/")
            .and_then(|name| spec.components.as_ref()?.request_bodies.get(name))
            .and_then(|body| resolve_request_body(spec, body)),
    }
}

fn resolve_example<'a>(
    spec: &'a OpenAPI,
    example: &'a ReferenceOr<Example>,
) -> Option<&'a Example> {
    match example {
        ReferenceOr::Item(example) => Some(example),
        ReferenceOr::Reference { reference } => reference
            .strip_prefix("#/components/examples/")
            .and_then(|name| spec.components.as_ref()?.examples.get(name))
            .and_then(|example| resolve_example(spec, example)),
    }
}

fn preferred_request_media_type(content: &openapiv3::Content) -> Option<&MediaType> {
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
        let ReferenceOr::Item(Parameter::Path { parameter_data, .. }) = parameter else {
            continue;
        };
        if parameter_data.required {
            arguments.push(format!("{:?}", path_parameter_value(&parameter_data.name)));
        }
    }

    let body = operation
        .request_body
        .as_ref()
        .and_then(|body| resolve_request_body(spec, body))
        .and_then(|body| {
            preferred_request_media_type(&body.content)
                .and_then(|media_type| media_type.schema.as_ref())
                .map(|schema| (body.required, schema))
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
        .any(|parameter| matches!(parameter, ReferenceOr::Item(Parameter::Query { .. })))
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

fn schema_value(spec: &OpenAPI, schema_ref: &ReferenceOr<Schema>, depth: usize) -> Option<Value> {
    if depth > 20 {
        return None;
    }
    match schema_ref {
        ReferenceOr::Reference { reference } => reference
            .strip_prefix("#/components/schemas/")
            .and_then(|name| spec.components.as_ref()?.schemas.get(name))
            .and_then(|schema| schema_value(spec, schema, depth + 1)),
        ReferenceOr::Item(schema) => {
            if let Some(example) = &schema.schema_data.example {
                return Some(example.clone());
            }
            if let Some(default) = &schema.schema_data.default {
                return Some(default.clone());
            }
            schema_kind_value(spec, &schema.schema_kind, depth + 1)
        }
    }
}

fn boxed_schema_value(
    spec: &OpenAPI,
    schema_ref: &ReferenceOr<Box<Schema>>,
    depth: usize,
) -> Option<Value> {
    match schema_ref {
        ReferenceOr::Reference { reference } => schema_value(
            spec,
            &ReferenceOr::Reference {
                reference: reference.clone(),
            },
            depth,
        ),
        ReferenceOr::Item(schema) => {
            schema_value(spec, &ReferenceOr::Item((**schema).clone()), depth)
        }
    }
}

fn schema_kind_value(spec: &OpenAPI, kind: &SchemaKind, depth: usize) -> Option<Value> {
    match kind {
        SchemaKind::Type(Type::String(string)) => string
            .enumeration
            .iter()
            .flatten()
            .next()
            .cloned()
            .map(Value::String)
            .or_else(|| Some(Value::String(string_example(&string.format).to_string()))),
        SchemaKind::Type(Type::Number(number)) => number
            .enumeration
            .iter()
            .flatten()
            .next()
            .or(number.minimum.as_ref())
            .and_then(|value| serde_json::Number::from_f64(*value))
            .map(Value::Number)
            .or_else(|| Some(Value::from(1.0))),
        SchemaKind::Type(Type::Integer(integer)) => Some(Value::from(
            integer
                .enumeration
                .iter()
                .flatten()
                .next()
                .copied()
                .or(integer.minimum)
                .unwrap_or(1),
        )),
        SchemaKind::Type(Type::Boolean(_)) => Some(Value::Bool(true)),
        SchemaKind::Type(Type::Array(array)) => {
            let item = array
                .items
                .as_ref()
                .and_then(|item| boxed_schema_value(spec, item, depth + 1));
            Some(item.into_iter().collect())
        }
        SchemaKind::Type(Type::Object(object)) => Some(object_value(
            spec,
            &object.properties,
            &object.required,
            depth + 1,
        )),
        SchemaKind::AllOf { all_of } => {
            let mut combined = Map::new();
            for schema in all_of {
                if let Some(Value::Object(values)) = schema_value(spec, schema, depth + 1) {
                    combined.extend(values);
                }
            }
            Some(Value::Object(combined))
        }
        SchemaKind::OneOf { one_of } => one_of
            .first()
            .and_then(|schema| schema_value(spec, schema, depth + 1)),
        SchemaKind::AnyOf { any_of } => any_of
            .first()
            .and_then(|schema| schema_value(spec, schema, depth + 1)),
        SchemaKind::Not { .. } => None,
        SchemaKind::Any(any) => any_schema_value(spec, any, depth + 1),
    }
}

fn any_schema_value(spec: &OpenAPI, any: &openapiv3::AnySchema, depth: usize) -> Option<Value> {
    if let Some(value) = any.enumeration.first() {
        return Some(value.clone());
    }
    if !any.properties.is_empty() {
        return Some(object_value(
            spec,
            &any.properties,
            &any.required,
            depth + 1,
        ));
    }
    if let Some(schema) = any
        .all_of
        .first()
        .or_else(|| any.one_of.first())
        .or_else(|| any.any_of.first())
    {
        return schema_value(spec, schema, depth + 1);
    }
    Some(Value::Object(Map::new()))
}

fn object_value(
    spec: &OpenAPI,
    properties: &indexmap::IndexMap<String, ReferenceOr<Box<Schema>>>,
    required: &[String],
    depth: usize,
) -> Value {
    let mut values = Map::new();
    for name in required {
        if let Some(value) = properties
            .get(name)
            .and_then(|schema| boxed_schema_value(spec, schema, depth + 1))
        {
            values.insert(name.clone(), value);
        }
    }
    Value::Object(values)
}

fn string_example(format: &VariantOrUnknownOrEmpty<openapiv3::StringFormat>) -> &'static str {
    match format {
        VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => "2024-01-01T00:00:00Z",
        VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => "2024-01-01",
        VariantOrUnknownOrEmpty::Unknown(format) if format == "uri" => "https://example.com",
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
        assert_eq!(generated.open_api_version, "1.0.0");
        assert_eq!(generated.samples.len(), 47);
        assert!(generated
            .samples
            .windows(2)
            .all(|samples| samples[0].id < samples[1].id));
        assert_eq!(generated, catalog());

        for sample in &generated.samples {
            syn::parse_file(&sample.sample)
                .unwrap_or_else(|error| panic!("invalid sample {}: {error}", sample.id));
        }

        let hosted_checkout = generated
            .samples
            .iter()
            .find(|sample| sample.id == "CreateCheckout.HostedCheckout")
            .expect("hosted checkout sample");
        assert_eq!(hosted_checkout.example.as_deref(), Some("HostedCheckout"));
        assert!(hosted_checkout
            .sample
            .contains("b50pr914-6k0e-3091-a592-890010285b3d"));
        let encoded = serde_json::to_value(hosted_checkout).expect("encode sample");
        assert!(encoded.get("sample").is_some());
        assert!(encoded.get("source").is_none());
    }
}
