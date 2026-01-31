//! Authentication helpers for the SumUp API client.
//!
//! Use [`Authorization::api_key`] for API keys (from the developer portal) or
//! [`Authorization::access_token`] for OAuth access tokens.

#[derive(Debug, Clone)]
/// Supported authorization credentials for the SumUp API.
pub enum Authorization {
    /// OAuth access token.
    AccessToken(String),
    /// API key provisioned in the SumUp developer portal.
    APIKey(String),
}

impl Authorization {
    /// Builds an API-key authorization credential.
    pub fn api_key(key: impl Into<String>) -> Self {
        Self::APIKey(key.into())
    }

    /// Builds an OAuth access-token authorization credential.
    pub fn access_token(token: impl Into<String>) -> Self {
        Self::AccessToken(token.into())
    }

    /// Returns the raw value that should be sent in the Authorization header.
    pub fn get_header(&self) -> &str {
        match self {
            Authorization::AccessToken(token) => token,
            Authorization::APIKey(api_key) => api_key,
        }
    }
}
