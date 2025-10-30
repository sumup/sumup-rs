//! SDK error helpers.
//!
//! Provides a concrete error hierarchy for all generated clients.

/// Generic SDK error type for SumUp API operations.
#[derive(Debug)]
pub enum SdkError<E> {
    /// Errors originating from the underlying HTTP client (network, TLS, etc.).
    Network(reqwest::Error),
    /// The server returned an API response with an error status code.
    Api(ApiError<E>),
}

impl<E> SdkError<E> {
    /// Wraps a [`reqwest::Error`] as a network failure.
    pub fn from_reqwest(error: reqwest::Error) -> Self {
        Self::Network(error)
    }

    /// Creates a new API error using the supplied body descriptor.
    pub fn api(status: reqwest::StatusCode, body: ApiErrorBody<E>) -> Self {
        Self::Api(ApiError::new(status, body))
    }

    /// Creates an API error with a successfully parsed body.
    pub fn api_parsed(status: reqwest::StatusCode, body: E) -> Self {
        Self::Api(ApiError::parsed(status, body))
    }

    /// Creates an API error that preserves the raw response body.
    pub fn api_raw(status: reqwest::StatusCode, body: impl Into<String>) -> Self {
        Self::Api(ApiError::raw(status, body))
    }

    /// Returns the HTTP status code associated with this error when available.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            Self::Network(err) => err.status(),
            Self::Api(api_err) => Some(api_err.status()),
        }
    }

    /// Returns the captured error body when the server responded with an error status.
    pub fn body(&self) -> Option<&ApiErrorBody<E>> {
        match self {
            Self::Api(api_err) => Some(api_err.body()),
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
            Self::Api(err) => write!(f, "API error: {}", err),
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

/// Detailed information about an API error response.
#[derive(Debug, Clone)]
pub struct ApiError<E> {
    status: reqwest::StatusCode,
    body: ApiErrorBody<E>,
}

impl<E> ApiError<E> {
    /// Constructs a new API error descriptor.
    pub fn new(status: reqwest::StatusCode, body: ApiErrorBody<E>) -> Self {
        Self { status, body }
    }

    /// Constructs an API error with a successfully parsed body.
    pub fn parsed(status: reqwest::StatusCode, body: E) -> Self {
        Self::new(status, ApiErrorBody::Parsed(body))
    }

    /// Constructs an API error capturing the raw response body.
    pub fn raw(status: reqwest::StatusCode, body: impl Into<String>) -> Self {
        Self::new(status, ApiErrorBody::Raw(body.into()))
    }

    /// Returns the HTTP status code that triggered this error.
    pub fn status(&self) -> reqwest::StatusCode {
        self.status
    }

    /// Returns a reference to the captured error body.
    pub fn body(&self) -> &ApiErrorBody<E> {
        &self.body
    }

    /// Consumes the error and returns the captured body.
    pub fn into_body(self) -> ApiErrorBody<E> {
        self.body
    }
}

impl<E> std::fmt::Display for ApiError<E>
where
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.body {
            ApiErrorBody::Parsed(body) => write!(f, "{}: {:?}", self.status, body),
            ApiErrorBody::Raw(body) => write!(f, "{}: {}", self.status, body),
        }
    }
}

impl<E> std::error::Error for ApiError<E> where E: std::fmt::Debug + 'static {}

/// Describes whether an API error body was parsed or captured raw.
#[derive(Debug, Clone)]
pub enum ApiErrorBody<E> {
    /// Body successfully parsed into the documented schema.
    Parsed(E),
    /// Raw response body captured as plain text.
    Raw(String),
}

impl<E> ApiErrorBody<E> {
    /// Returns the parsed body if available.
    pub fn parsed(&self) -> Option<&E> {
        match self {
            Self::Parsed(value) => Some(value),
            Self::Raw(_) => None,
        }
    }

    /// Returns the raw body when parsing was not possible.
    pub fn raw(&self) -> Option<&str> {
        match self {
            Self::Parsed(_) => None,
            Self::Raw(body) => Some(body),
        }
    }
}

/// Result alias that uses [`SdkError`] as its error type.
pub type SdkResult<T, E> = std::result::Result<T, SdkError<E>>;
