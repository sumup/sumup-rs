// The contents of this file are generated; do not modify them.

use crate::auth::Authorization;
/// The main SumUp API client.
///
/// Use this client to access different API endpoints organized by tags.
#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    authorization: Option<Authorization>,
    timeout: std::time::Duration,
    runtime_info: Vec<(&'static str, String)>,
}
impl Client {
    fn build_http_client() -> reqwest::Client {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/problem+json, application/json"),
        );
        reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .expect("failed to build reqwest client with default headers")
    }
    /// Creates a new SumUp API client with the default base URL.
    /// Tries to read the authorization token from the SUMUP_API_KEY environment variable.
    /// Default timeout is 10 seconds.
    pub fn new() -> Self {
        let authorization = std::env::var("SUMUP_API_KEY")
            .ok()
            .map(Authorization::APIKey);
        Self {
            http_client: Self::build_http_client(),
            base_url: "https://api.sumup.com".to_string(),
            authorization,
            timeout: std::time::Duration::from_secs(10),
            runtime_info: crate::version::runtime_info(),
        }
    }
    /// Overrides the underlying HTTP client used for requests.
    /// Returns a new client with the provided `reqwest::Client`.
    pub fn with_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }
    /// Sets the base URL for API requests.
    /// Returns a new client with the updated base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
    /// Sets the authorization token for API requests.
    /// Returns a new client with the updated token.
    pub fn with_authorization(mut self, auth: Authorization) -> Self {
        self.authorization = Some(auth);
        self
    }
    /// Sets the request timeout for API requests.
    /// Returns a new client with the updated timeout.
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Returns a reference to the HTTP client.
    pub(crate) fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }
    /// Returns the base URL for the API.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    /// Returns the authorization token if set.
    pub fn authorization(&self) -> Option<&str> {
        self.authorization.as_ref().map(|auth| auth.get_header())
    }
    /// Returns the request timeout.
    pub fn timeout(&self) -> std::time::Duration {
        self.timeout
    }
    /// Returns the runtime headers sent with each request.
    pub(crate) fn runtime_headers(&self) -> &[(&'static str, String)] {
        &self.runtime_info
    }
    #[cfg(feature = "webhooks")]
    /// Performs a GET request against an arbitrary absolute or relative URL and
    /// deserializes the successful JSON response into `T`.
    pub(crate) async fn get<T>(&self, url: &str) -> crate::error::SdkResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = if reqwest::Url::parse(url).is_ok() {
            url.to_owned()
        } else {
            format!("{}{}", self.base_url(), url)
        };
        let mut request = self
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.timeout());
        if let Some(authorization) = self.authorization() {
            request = request.header("Authorization", format!("Bearer {}", authorization));
        }
        for (header_name, header_value) in self.runtime_headers() {
            request = request.header(*header_name, header_value);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => response.json().await.map_err(Into::into),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    #[cfg(feature = "webhooks")]
    /// Creates a webhook helper bound to this client and signing secret.
    pub fn webhook_handler(&self, secret: impl AsRef<[u8]>) -> crate::webhooks::WebhookHandler<'_> {
        crate::webhooks::WebhookHandler::new(self, secret)
    }
    /// Returns a client for the Checkouts API endpoints.
    pub fn checkouts(&self) -> crate::resources::checkouts::CheckoutsClient<'_> {
        crate::resources::checkouts::CheckoutsClient::new(self)
    }
    /// Returns a client for the Customers API endpoints.
    pub fn customers(&self) -> crate::resources::customers::CustomersClient<'_> {
        crate::resources::customers::CustomersClient::new(self)
    }
    /// Returns a client for the Members API endpoints.
    pub fn members(&self) -> crate::resources::members::MembersClient<'_> {
        crate::resources::members::MembersClient::new(self)
    }
    /// Returns a client for the Memberships API endpoints.
    pub fn memberships(&self) -> crate::resources::memberships::MembershipsClient<'_> {
        crate::resources::memberships::MembershipsClient::new(self)
    }
    /// Returns a client for the Merchants API endpoints.
    pub fn merchants(&self) -> crate::resources::merchants::MerchantsClient<'_> {
        crate::resources::merchants::MerchantsClient::new(self)
    }
    /// Returns a client for the Payouts API endpoints.
    pub fn payouts(&self) -> crate::resources::payouts::PayoutsClient<'_> {
        crate::resources::payouts::PayoutsClient::new(self)
    }
    /// Returns a client for the Readers API endpoints.
    pub fn readers(&self) -> crate::resources::readers::ReadersClient<'_> {
        crate::resources::readers::ReadersClient::new(self)
    }
    /// Returns a client for the Receipts API endpoints.
    pub fn receipts(&self) -> crate::resources::receipts::ReceiptsClient<'_> {
        crate::resources::receipts::ReceiptsClient::new(self)
    }
    /// Returns a client for the Roles API endpoints.
    pub fn roles(&self) -> crate::resources::roles::RolesClient<'_> {
        crate::resources::roles::RolesClient::new(self)
    }
    /// Returns a client for the Transactions API endpoints.
    pub fn transactions(&self) -> crate::resources::transactions::TransactionsClient<'_> {
        crate::resources::transactions::TransactionsClient::new(self)
    }
}
impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
