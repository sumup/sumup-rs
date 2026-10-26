// The contents of this file are generated; do not modify them.

//! Merchant tokens let your application accept card-present payments with SumUp SDKs that support token-based authentication. Use them when you want users to sign in to your application with your own authentication, then accept payments for a SumUp merchant account.
//!
//! Create tokens on your backend using one of the [supported authentication methods](https://developer.sumup.com/api/authentication). Keep your authentication credentials on your backend. Return only the short-lived merchant token to your application and pass its `access_token` to the SDK.
//!
//! Before returning a token, authenticate the user in your application and check that they can accept payments for the requested merchant account. A merchant token is limited to that account and the requested actions, and does not grant permissions beyond those of the identity that created it.
//!
//! Tokens expire 15 minutes after creation. Use `expires_in` to determine the remaining lifetime and create a new token when needed. This endpoint does not return a refresh token. Treat merchant tokens as secrets: send them over HTTPS and keep them out of URLs, logs, and source code.
use super::common::*;
/// A short-lived credential for the requested merchant account and actions. Pass access_token to a SumUp SDK that supports merchant tokens.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Token {
    /// Secret token to pass to the SDK. Treat this value as an opaque string; do not parse it or depend on its format.
    ///
    /// Example: `MERCHANT_TOKEN`
    #[builder(setter(into, doc = "Sets [`access_token`](Token::access_token)."))]
    pub access_token: String,
    /// Remaining lifetime of the token in seconds when the response is created. Tokens expire 15 minutes after creation. Request a new token when needed.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `899`
    #[builder(setter(doc = "Sets [`expires_in`](Token::expires_in)."))]
    pub expires_in: i64,
    /// Authentication scheme used by the token. Always Bearer.
    ///
    /// Example: `Bearer`
    #[builder(setter(doc = "Sets [`token_type`](Token::token_type)."))]
    pub token_type: TokenTokenType,
}
/// Authentication scheme used by the token. Always Bearer.
///
/// Example: `Bearer`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TokenTokenType {
    Bearer,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateRequestActionsItem {
    #[serde(rename = "checkout")]
    Checkout,
    #[serde(untagged)]
    Other(String),
}
/// Actions the merchant token should allow.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequest {
    /// Actions the token allows for this merchant account.
    ///
    /// Constraints:
    /// - min items: 1
    /// - items must be unique
    #[builder(setter(doc = "Sets [`actions`](CreateRequest::actions)."))]
    pub actions: Vec<CreateRequestActionsItem>,
}
use crate::client::Client;
#[derive(Debug, PartialEq)]
pub enum CreateErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    Forbidden(Problem),
}
impl crate::error::ProblemDetails for CreateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::Forbidden(problem) => Some(problem),
        }
    }
}
/// Client for the Tokens API endpoints.
#[derive(Debug)]
pub struct TokensClient<'a> {
    client: &'a Client,
}
impl<'a> TokensClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Create a merchant token
    ///
    /// Creates a short-lived token for the merchant account identified by `merchant_code` and the requested actions.
    ///
    /// Call this endpoint from your backend using a [supported authentication method](https://developer.sumup.com/api/authentication). You must have access to the merchant account. Before returning the token, check that the user signed in to your application is allowed to accept payments for that account.
    ///
    /// Pass the returned `access_token` to the SDK. The token expires 15 minutes after creation; use `expires_in` for its remaining lifetime. To continue after it expires, create a new token through this endpoint.
    ///
    /// Responses:
    /// - 200: Merchant token created. Pass its access_token to the SDK.
    /// - 400: The request body is invalid or contains unsupported or duplicate actions.
    /// - 401: Authentication is missing, invalid, or expired.
    /// - 403: The authenticated identity does not have permission to create tokens for this
    ///   merchant account.
    pub async fn create(
        &self,
        merchant_code: impl AsRef<str>,
        body: CreateRequest,
    ) -> crate::error::SdkResult<Token, CreateErrorBody> {
        let path = format!("/v0.1/merchants/{}/tokens", merchant_code.as_ref());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .post(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout())
            .json(&body);
        if let Some(authorization) = self.client.authorization() {
            request = request.header("Authorization", format!("Bearer {}", authorization));
        }
        for (header_name, header_value) in self.client.runtime_headers() {
            request = request.header(*header_name, header_value);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Token = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::Forbidden)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
