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
