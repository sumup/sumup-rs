//! SDK error helpers.
//!
//! Provides a concrete error hierarchy for all generated clients.

use serde::{Deserialize, Serialize};

/// Generic SDK error type for SumUp API operations.
#[derive(Debug)]
pub enum SdkError<E = UnknownApiBody> {
    /// Errors originating from the underlying HTTP client (network, TLS, etc.).
    Network(reqwest::Error),
    /// The server returned an API response with an expected error payload.
    Api(E),
    /// The server returned an unexpected status or payload.
    Unexpected(reqwest::StatusCode, UnknownApiBody),
}

impl<E> SdkError<E> {
    /// Wraps a [`reqwest::Error`] as a network failure.
    pub fn from_reqwest(error: reqwest::Error) -> Self {
        Self::Network(error)
    }

    /// Creates a new API error using the supplied body payload.
    pub fn api(body: E) -> Self {
        Self::Api(body)
    }

    /// Creates an unexpected API error preserving the raw payload.
    pub fn unexpected(status: reqwest::StatusCode, body: UnknownApiBody) -> Self {
        Self::Unexpected(status, body)
    }

    /// Returns the HTTP status code associated with this error when available.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            Self::Network(err) => err.status(),
            Self::Unexpected(status, _) => Some(*status),
            Self::Api(_) => None,
        }
    }

    /// Returns the captured error body when the server responded with an expected error payload.
    pub fn body(&self) -> Option<&E> {
        match self {
            Self::Api(body) => Some(body),
            _ => None,
        }
    }

    /// Returns the captured unexpected body, when available.
    pub fn unexpected_body(&self) -> Option<&UnknownApiBody> {
        match self {
            Self::Unexpected(_, body) => Some(body),
            _ => None,
        }
    }

    /// Consumes the error, yielding the captured API body when available.
    pub fn into_body(self) -> Option<E> {
        match self {
            Self::Api(body) => Some(body),
            _ => None,
        }
    }
}

impl<E> From<reqwest::Error> for SdkError<E> {
    fn from(value: reqwest::Error) -> Self {
        Self::from_reqwest(value)
    }
}

impl<E> std::fmt::Display for SdkError<E>
where
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(err) => write!(f, "network error: {}", err),
            Self::Api(body) => write!(f, "API error: {:?}", body),
            Self::Unexpected(status, body) => {
                write!(f, "unexpected API error ({}): {}", status, body)
            }
        }
    }
}

impl<E> std::error::Error for SdkError<E>
where
    E: std::fmt::Debug + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network(err) => Some(err),
            _ => None,
        }
    }
}

/// Describes an unexpected SumUp API error payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnknownApiBody {
    /// JSON payload when the body was valid JSON but schema is unknown.
    Json(serde_json::Value),
    /// Plain text fallback.
    Text(String),
    /// Empty body.
    Empty,
}

impl UnknownApiBody {
    /// Converts a raw response body into an [`UnknownApiBody`].
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Self::Empty;
        }

        if let Ok(json) = serde_json::from_slice(bytes) {
            Self::Json(json)
        } else if let Ok(text) = std::str::from_utf8(bytes) {
            Self::Text(text.to_owned())
        } else {
            Self::Empty
        }
    }

    /// Converts a raw UTF-8 response body into an [`UnknownApiBody`].
    pub fn from_text(body: String) -> Self {
        Self::from_bytes(body.as_bytes())
    }
}

impl std::fmt::Display for UnknownApiBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(value) => write!(f, "{}", value),
            Self::Text(text) => write!(f, "{}", text),
            Self::Empty => write!(f, "<empty>"),
        }
    }
}

/// Result alias that uses [`SdkError`] as its error type.
pub type SdkResult<T, E = UnknownApiBody> = std::result::Result<T, SdkError<E>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_api_body_parses_json_payloads() {
        let payload = br#"{"error":"invalid"}"#;
        match UnknownApiBody::from_bytes(payload) {
            UnknownApiBody::Json(value) => assert_eq!(value["error"], "invalid"),
            other => panic!("expected Json variant, got {:?}", other),
        }
    }

    #[test]
    fn unknown_api_body_handles_plain_text() {
        let payload = b"plain text error";
        match UnknownApiBody::from_bytes(payload) {
            UnknownApiBody::Text(text) => assert_eq!(text, "plain text error"),
            other => panic!("expected Text variant, got {:?}", other),
        }
    }

    #[test]
    fn unknown_api_body_handles_non_utf8_bytes() {
        let payload = [0xff, 0xfe];
        match UnknownApiBody::from_bytes(&payload) {
            UnknownApiBody::Empty => {}
            other => panic!("expected Empty variant, got {:?}", other),
        }
    }
}
