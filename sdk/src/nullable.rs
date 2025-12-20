//! Support for nullable fields that distinguish between null and present values.
//!
//! When combined with `Option<Nullable<T>>`, this enables three distinct states for API fields:
//! - `None` - Field not included in the request/response (absent)
//! - `Some(Nullable::Null)` - Field explicitly set to `null`
//! - `Some(Nullable::Value(x))` - Field has an actual value
//!
//! This is particularly useful for PATCH/UPDATE operations where you need to distinguish
//! between "don't change this field" (absent), "clear this field" (null), and "set to value" (present).

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper type that distinguishes between `null` and an actual value.
///
/// When combined with `Option<Nullable<T>>`, this enables three distinct states:
/// - `None` - Field not included in the request/response (absent)
/// - `Some(Nullable::Null)` - Field explicitly set to `null`
/// - `Some(Nullable::Value(x))` - Field has an actual value
///
/// # Examples
///
/// ```
/// use sumup::Nullable;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct UpdateRequest {
///     // Use Option::is_none for the common case
///     #[serde(skip_serializing_if = "Option::is_none")]
///     name: Option<String>,
///     
///     // For nullable fields, use Option<Nullable<T>> with custom deserializer
///     #[serde(
///         default,
///         skip_serializing_if = "Option::is_none",
///         deserialize_with = "sumup::nullable::deserialize"
///     )]
///     description: Option<Nullable<String>>,
/// }
///
/// // Absent - field won't be serialized
/// let req = UpdateRequest { name: None, description: None };
/// assert_eq!(serde_json::to_string(&req).unwrap(), "{}");
///
/// // Null - field will be serialized as null
/// let req = UpdateRequest {
///     name: Some("Name".to_string()),
///     description: Some(Nullable::Null),
/// };
/// assert_eq!(
///     serde_json::to_string(&req).unwrap(),
///     r#"{"name":"Name","description":null}"#
/// );
///
/// // Present - field will be serialized with value
/// let req = UpdateRequest {
///     name: Some("Name".to_string()),
///     description: Some(Nullable::Value("Desc".to_string())),
/// };
/// assert_eq!(
///     serde_json::to_string(&req).unwrap(),
///     r#"{"name":"Name","description":"Desc"}"#
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nullable<T> {
    /// Field is explicitly `null` in the JSON
    Null,
    /// Field has an actual value
    Value(T),
}

impl<T> Nullable<T> {
    /// Returns `true` if the value is `Null`.
    pub fn is_null(&self) -> bool {
        matches!(self, Nullable::Null)
    }

    /// Returns `true` if the value is `Value`.
    pub fn is_value(&self) -> bool {
        matches!(self, Nullable::Value(_))
    }

    /// Converts from `&Nullable<T>` to `Nullable<&T>`.
    pub fn as_ref(&self) -> Nullable<&T> {
        match self {
            Nullable::Null => Nullable::Null,
            Nullable::Value(x) => Nullable::Value(x),
        }
    }

    /// Converts from `&mut Nullable<T>` to `Nullable<&mut T>`.
    pub fn as_mut(&mut self) -> Nullable<&mut T> {
        match self {
            Nullable::Null => Nullable::Null,
            Nullable::Value(x) => Nullable::Value(x),
        }
    }

    /// Returns the contained `Value`, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is `Null`.
    pub fn unwrap(self) -> T {
        match self {
            Nullable::Value(val) => val,
            Nullable::Null => panic!("called `Nullable::unwrap()` on a `Null` value"),
        }
    }

    /// Returns the contained `Value` or a provided default.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Nullable::Value(val) => val,
            Nullable::Null => default,
        }
    }

    /// Returns the contained `Value` or computes it from a closure.
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Nullable::Value(val) => val,
            Nullable::Null => f(),
        }
    }

    /// Maps a `Nullable<T>` to `Nullable<U>` by applying a function to the contained value.
    pub fn map<U, F>(self, f: F) -> Nullable<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Nullable::Value(val) => Nullable::Value(f(val)),
            Nullable::Null => Nullable::Null,
        }
    }

    /// Converts from `Nullable<T>` to `Option<T>`.
    ///
    /// - `Null` becomes `None`
    /// - `Value(x)` becomes `Some(x)`
    pub fn into_option(self) -> Option<T> {
        match self {
            Nullable::Null => None,
            Nullable::Value(val) => Some(val),
        }
    }

    /// Returns the contained value as an `Option`.
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Nullable::Null => None,
            Nullable::Value(val) => Some(val),
        }
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Nullable::Null,
            Some(val) => Nullable::Value(val),
        }
    }
}

impl<T> From<T> for Nullable<T> {
    fn from(val: T) -> Self {
        Nullable::Value(val)
    }
}

/// Helper function for deserializing `Option<Nullable<T>>` fields.
///
/// This should be used with `#[serde(default, deserialize_with = "nullable::deserialize")]`.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<Nullable<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct NullableVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for NullableVisitor<T> {
        type Value = Option<Nullable<T>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any value or null")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(Nullable::Null))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(Nullable::Null))
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(|val| Some(Nullable::Value(val)))
        }
    }

    deserializer.deserialize_option(NullableVisitor(std::marker::PhantomData))
}

impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Nullable::Null => serializer.serialize_none(),
            Nullable::Value(val) => val.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(|opt| match opt {
            None => Nullable::Null,
            Some(val) => Nullable::Value(val),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::nullable::deserialize"
        )]
        field: Option<Nullable<String>>,
    }

    #[test]
    fn test_serialize_absent() {
        let s = TestStruct { field: None };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_serialize_null() {
        let s = TestStruct {
            field: Some(Nullable::Null),
        };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, r#"{"field":null}"#);
    }

    #[test]
    fn test_serialize_value() {
        let s = TestStruct {
            field: Some(Nullable::Value("value".to_string())),
        };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, r#"{"field":"value"}"#);
    }

    #[test]
    fn test_deserialize_absent() {
        let json = "{}";
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.field, None);
    }

    #[test]
    fn test_deserialize_null() {
        let json = r#"{"field":null}"#;
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.field, Some(Nullable::Null));
    }

    #[test]
    fn test_deserialize_value() {
        let json = r#"{"field":"value"}"#;
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.field, Some(Nullable::Value("value".to_string())));
    }

    #[test]
    fn test_is_methods() {
        assert!(Nullable::<i32>::Null.is_null());
        assert!(!Nullable::<i32>::Null.is_value());

        assert!(!Nullable::Value(42).is_null());
        assert!(Nullable::Value(42).is_value());
    }

    #[test]
    fn test_map() {
        let x: Nullable<i32> = Nullable::Value(5);
        assert_eq!(x.map(|v| v * 2), Nullable::Value(10));

        let x: Nullable<i32> = Nullable::Null;
        assert_eq!(x.map(|v| v * 2), Nullable::Null);
    }

    #[test]
    fn test_into_option() {
        assert_eq!(Nullable::<i32>::Null.into_option(), None);
        assert_eq!(Nullable::Value(42).into_option(), Some(42));
    }

    #[test]
    fn test_as_option() {
        assert_eq!(Nullable::<i32>::Null.as_option(), None);
        assert_eq!(Nullable::Value(42).as_option(), Some(&42));
    }
}
