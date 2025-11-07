// The contents of this file are generated; do not modify them.

/// The main SumUp API client.
///
/// Use this client to access different API endpoints organized by tags.
#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    authorization_token: Option<String>,
    timeout: std::time::Duration,
}
impl Client {
    /// Creates a new SumUp API client with the default base URL.
    /// Tries to read the authorization token from the SUMUP_API_KEY environment variable.
    /// Default timeout is 10 seconds.
    pub fn new() -> Self {
        let authorization_token = std::env::var("SUMUP_API_KEY").ok();
        Self {
            http_client: reqwest::Client::new(),
            base_url: "https://api.sumup.com".to_string(),
            authorization_token,
            timeout: std::time::Duration::from_secs(10),
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
    pub fn with_authorization(mut self, token: impl Into<String>) -> Self {
        self.authorization_token = Some(token.into());
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
    pub fn authorization_token(&self) -> Option<&str> {
        self.authorization_token.as_deref()
    }
    /// Returns the request timeout.
    pub fn timeout(&self) -> std::time::Duration {
        self.timeout
    }
    pub fn checkouts(&self) -> crate::resources::checkouts::CheckoutsClient<'_> {
        crate::resources::checkouts::CheckoutsClient::new(self)
    }
    pub fn customers(&self) -> crate::resources::customers::CustomersClient<'_> {
        crate::resources::customers::CustomersClient::new(self)
    }
    pub fn members(&self) -> crate::resources::members::MembersClient<'_> {
        crate::resources::members::MembersClient::new(self)
    }
    pub fn memberships(&self) -> crate::resources::memberships::MembershipsClient<'_> {
        crate::resources::memberships::MembershipsClient::new(self)
    }
    pub fn merchant(&self) -> crate::resources::merchant::MerchantClient<'_> {
        crate::resources::merchant::MerchantClient::new(self)
    }
    pub fn merchants(&self) -> crate::resources::merchants::MerchantsClient<'_> {
        crate::resources::merchants::MerchantsClient::new(self)
    }
    pub fn payouts(&self) -> crate::resources::payouts::PayoutsClient<'_> {
        crate::resources::payouts::PayoutsClient::new(self)
    }
    pub fn readers(&self) -> crate::resources::readers::ReadersClient<'_> {
        crate::resources::readers::ReadersClient::new(self)
    }
    pub fn receipts(&self) -> crate::resources::receipts::ReceiptsClient<'_> {
        crate::resources::receipts::ReceiptsClient::new(self)
    }
    pub fn roles(&self) -> crate::resources::roles::RolesClient<'_> {
        crate::resources::roles::RolesClient::new(self)
    }
    #[cfg(feature = "deprecated-resources")]
    #[allow(deprecated)]
    #[deprecated(
        note = "Subaccounts API is deprecated, please use [Members](https://developer.sumup.com/api/members) API instead to manage your account members."
    )]
    pub fn subaccounts(&self) -> crate::resources::subaccounts::SubaccountsClient<'_> {
        crate::resources::subaccounts::SubaccountsClient::new(self)
    }
    pub fn transactions(&self) -> crate::resources::transactions::TransactionsClient<'_> {
        crate::resources::transactions::TransactionsClient::new(self)
    }
}
impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
