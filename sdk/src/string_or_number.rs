use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T> {
    String(String),
    Number(T),
}

/// Deserializes a numeric value that can be encoded either as a JSON number
/// or as a JSON string containing a number.
#[allow(dead_code)]
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
{
    match StringOrNumber::<T>::deserialize(deserializer)? {
        StringOrNumber::String(value) => value.parse().map_err(serde::de::Error::custom),
        StringOrNumber::Number(value) => Ok(value),
    }
}

/// Deserializes an optional numeric value that can be encoded either as a JSON
/// number or as a JSON string containing a number.
pub fn deserialize_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
{
    match Option::<StringOrNumber<T>>::deserialize(deserializer)? {
        Some(StringOrNumber::String(value)) => {
            value.parse().map(Some).map_err(serde::de::Error::custom)
        }
        Some(StringOrNumber::Number(value)) => Ok(Some(value)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct RequiredFloat {
        #[serde(deserialize_with = "crate::string_or_number::deserialize")]
        amount: f64,
    }

    #[derive(Debug, Deserialize)]
    struct OptionalInt {
        #[serde(
            default,
            deserialize_with = "crate::string_or_number::deserialize_option"
        )]
        id: Option<i64>,
    }

    #[test]
    fn deserializes_required_number_from_json_number() {
        let value: RequiredFloat = serde_json::from_str(r#"{"amount": 12.5}"#).unwrap();
        assert_eq!(value.amount, 12.5);
    }

    #[test]
    fn deserializes_required_number_from_json_string() {
        let value: RequiredFloat = serde_json::from_str(r#"{"amount": "12.5"}"#).unwrap();
        assert_eq!(value.amount, 12.5);
    }

    #[test]
    fn deserializes_optional_integer_from_string() {
        let value: OptionalInt = serde_json::from_str(r#"{"id": "42"}"#).unwrap();
        assert_eq!(value.id, Some(42));
    }

    #[test]
    fn deserializes_missing_optional_integer_as_none() {
        let value: OptionalInt = serde_json::from_str("{}").unwrap();
        assert_eq!(value.id, None);
    }
}
