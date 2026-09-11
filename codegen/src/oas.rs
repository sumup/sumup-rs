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
        oas3::spec::Schema::Object(object) => Some(object),
        oas3::spec::Schema::Boolean(_) => None,
    }
}
