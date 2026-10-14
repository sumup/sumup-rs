use oas3::spec::{ObjectSchema, SchemaType as Type, SchemaTypeSet as TypeSet};

pub(crate) fn schema_type(schema: &ObjectSchema) -> Option<Type> {
    match schema.schema_type.as_ref()? {
        TypeSet::Single(schema_type) => Some(*schema_type),
        TypeSet::Multiple(types) => types.iter().copied().find(|kind| *kind != Type::Null),
    }
}

pub(crate) fn schema_example(schema: &ObjectSchema) -> Option<&serde_json::Value> {
    schema.example.as_ref().or_else(|| schema.examples.first())
}

/// Returns the object or reference part of a JSON Schema, if present.
pub(crate) fn schema_object(
    schema: &oas3::spec::Schema,
) -> Option<&oas3::spec::ObjectOrReference<ObjectSchema>> {
    match schema {
        oas3::spec::Schema::Object(object) => Some(schema_reference(object)),
        oas3::spec::Schema::Boolean(_) => None,
    }
}

/// Returns the effective reference or schema inside annotation-only wrappers.
pub(crate) fn schema_reference(
    object: &oas3::spec::ObjectOrReference<ObjectSchema>,
) -> &oas3::spec::ObjectOrReference<ObjectSchema> {
    if let oas3::spec::ObjectOrReference::Object(value) = object
        && !value.all_of.is_empty()
        && annotation_only(object, true)
    {
        let mut branches = value
            .all_of
            .iter()
            .filter(|branch| !annotation_only(*branch, false));
        if let Some(branch) = branches.next()
            && branches.next().is_none()
            && let Some(target) = schema_object(branch)
        {
            return target;
        }
    }
    object
}

// Annotation-only allOf branches do not change the underlying JSON value type.
// Inspect serialized keys so an unrecognized validation keyword is never erased.
fn annotation_only(schema: &impl serde::Serialize, allow_all_of: bool) -> bool {
    let Ok(serde_json::Value::Object(value)) = serde_json::to_value(schema) else {
        return false;
    };
    value.keys().all(|key| {
        matches!(
            key.as_str(),
            "title"
                | "description"
                | "$comment"
                | "default"
                | "example"
                | "examples"
                | "deprecated"
                | "readOnly"
                | "writeOnly"
                | "externalDocs"
        ) || (allow_all_of && key == "allOf")
    })
}
