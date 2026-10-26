//! SDK error helpers.
//!
//! Provides a concrete error hierarchy for all generated clients.

use serde::{Deserialize, Serialize};

/// Generic SDK error type for SumUp API operations.
#[derive(Debug)]
pub enum SdkError<E = UnknownApiBody> {
    /// The request was rejected locally before it could be sent.
    InvalidRequest(String),
    /// Errors originating from the underlying HTTP client (network, TLS, etc.).
    Network(reqwest::Error),
    /// An HTTP response arrived, but its body could not be read.
    Response(Box<ResponseError>),
    /// The server returned an API response with an expected error payload.
    Api(Box<ApiErrorResponse<E>>),
    /// The server returned an unexpected status or payload.
    Unexpected(Box<ApiErrorResponse<UnknownApiBody>>),
}

impl<E> SdkError<E> {
    /// Wraps a [`reqwest::Error`] as a network failure.
    pub fn from_reqwest(error: reqwest::Error) -> Self {
        Self::Network(error)
    }

    /// Reads an API error while retaining its HTTP response context.
    #[doc(hidden)]
    pub async fn from_response(
        response: reqwest::Response,
        decode: impl FnOnce(&[u8]) -> Result<E, serde_json::Error>,
    ) -> Self {
        let status = response.status();
        let headers = response.headers().clone();
        let raw_body = match response.bytes().await {
            Ok(bytes) => bytes.to_vec(),
            Err(source) => {
                return Self::Response(Box::new(ResponseError {
                    status,
                    headers,
                    source,
                }));
            }
        };
        match decode(&raw_body) {
            Ok(body) => Self::Api(Box::new(ApiErrorResponse {
                status,
                headers,
                body,
                raw_body,
            })),
            Err(_) => Self::Unexpected(Box::new(ApiErrorResponse {
                status,
                headers,
                body: UnknownApiBody::from_bytes(&raw_body),
                raw_body,
            })),
        }
    }

    /// Reads an unrecognized API response without losing its status or payload.
    #[doc(hidden)]
    pub async fn unexpected_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let headers = response.headers().clone();
        match response.bytes().await {
            Ok(bytes) => Self::Unexpected(Box::new(ApiErrorResponse {
                status,
                headers,
                body: UnknownApiBody::from_bytes(&bytes),
                raw_body: bytes.to_vec(),
            })),
            Err(source) => Self::Response(Box::new(ResponseError {
                status,
                headers,
                source,
            })),
        }
    }

    /// Returns the HTTP status code associated with this error when available.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            Self::Network(err) => err.status(),
            Self::Response(error) => Some(error.status),
            Self::Unexpected(response) => Some(response.status),
            Self::Api(response) => Some(response.status),
            Self::InvalidRequest(_) => None,
        }
    }

    /// Returns the captured error body when the server responded with an expected error payload.
    pub fn body(&self) -> Option<&E> {
        match self {
            Self::Api(response) => Some(&response.body),
            _ => None,
        }
    }

    /// Returns the captured unexpected body, when available.
    pub fn unexpected_body(&self) -> Option<&UnknownApiBody> {
        match self {
            Self::Unexpected(response) => Some(&response.body),
            _ => None,
        }
    }

    /// Consumes the error, yielding the captured API body when available.
    pub fn into_body(self) -> Option<E> {
        match self {
            Self::Api(response) => Some(response.body),
            _ => None,
        }
    }
    /// Returns the response headers for an API failure.
    pub fn headers(&self) -> Option<&reqwest::header::HeaderMap> {
        match self {
            Self::Response(error) => Some(&error.headers),
            Self::Api(response) => Some(&response.headers),
            Self::Unexpected(response) => Some(&response.headers),
            _ => None,
        }
    }

    /// Returns the original response bytes for an API failure.
    pub fn raw_body(&self) -> Option<&[u8]> {
        match self {
            Self::Api(response) => Some(&response.raw_body),
            Self::Unexpected(response) => Some(&response.raw_body),
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
            Self::InvalidRequest(reason) => write!(f, "invalid request: {reason}"),
            Self::Network(err) => write!(f, "network error: {}", err),
            Self::Response(error) => {
                write!(f, "response error ({}): {}", error.status, error.source)
            }
            Self::Api(response) => {
                write!(f, "API error ({}): {:?}", response.status, response.body)
            }
            Self::Unexpected(response) => {
                write!(
                    f,
                    "unexpected API error ({}): {}",
                    response.status, response.body
                )
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
            Self::Response(error) => Some(&error.source),
            _ => None,
        }
    }
}

/// An HTTP response whose body could not be read.
#[derive(Debug)]
pub struct ResponseError {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub source: reqwest::Error,
}

/// An API response with typed error details and its original HTTP context.
#[derive(Debug)]
pub struct ApiErrorResponse<E> {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: E,
    pub raw_body: Vec<u8>,
}

/// Provides typed Problem details for generated error bodies.
pub trait ProblemDetails {
    fn problem(&self) -> Option<&crate::resources::common::Problem>;
}

impl<E: ProblemDetails> SdkError<E> {
    /// Returns typed Problem details when this API failure contains them.
    pub fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Api(response) => response.body.problem(),
            Self::Unexpected(response) => response.body.problem(),
            _ => None,
        }
    }
}

/// Describes an unexpected SumUp API error payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnknownApiBody {
    /// A Problem response received outside the documented error cases.
    Problem(crate::resources::common::Problem),
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

        if let Ok(problem) = serde_json::from_slice::<crate::resources::common::Problem>(bytes) {
            return Self::Problem(problem);
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

impl ProblemDetails for UnknownApiBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Problem(problem) => Some(problem),
            _ => None,
        }
    }
}

impl std::fmt::Display for UnknownApiBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Problem(problem) => write!(f, "{}", problem),
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
    fn invalid_request_has_no_response_or_source() {
        let error = SdkError::<UnknownApiBody>::InvalidRequest("invalid event object URL".into());

        assert_eq!(
            error.to_string(),
            "invalid request: invalid event object URL"
        );
        assert_eq!(error.status(), None);
        assert_eq!(error.body(), None);
        assert_eq!(error.unexpected_body(), None);
        assert!(std::error::Error::source(&error).is_none());
        assert_eq!(error.into_body(), None);
    }

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
