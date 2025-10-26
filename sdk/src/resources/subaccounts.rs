// The contents of this file are generated; do not modify them.

/// Error object for compat API calls.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompatError {
    pub error_code: String,
    pub message: String,
}
impl std::fmt::Display for CompatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for CompatError {}
/// Operator account for a merchant.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Operator {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub disabled: bool,
    /// The timestamp of when the operator was created.
    pub created_at: crate::datetime::DateTime,
    /// The timestamp of when the operator was last updated.
    pub updated_at: crate::datetime::DateTime,
    pub permissions: Permissions,
    pub account_type: String,
}
/// Permissions assigned to an operator or user.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Permissions {
    pub create_moto_payments: bool,
    pub create_referral: bool,
    pub full_transaction_history_view: bool,
    pub refund_transactions: bool,
    pub admin: bool,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreateSubAccountBodyPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_moto_payments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_referral: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_transaction_history_view: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_transactions: Option<bool>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateSubAccountBodyPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_moto_payments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_referral: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_transaction_history_view: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_transactions: Option<bool>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListSubAccountsParams {
    /// Search query used to filter users that match given query term.
    ///
    /// Current implementation allow querying only over the email address.
    /// All operators whos email address contains the query string are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// If true the list of operators will include also the primary user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_primary: Option<bool>,
}
/// List of operators.
pub type ListSubAccountsResponse = Vec<Operator>;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSubAccountBody {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<CreateSubAccountBodyPermissions>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateSubAccountBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<UpdateSubAccountBodyPermissions>,
}
use crate::client::Client;
///Client for the Subaccounts API endpoints.
#[derive(Debug)]
pub struct SubaccountsClient<'a> {
    client: &'a Client,
}
impl<'a> SubaccountsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// List operators
    ///
    /// Returns list of operators for currently authorized user's merchant.
    pub async fn list_sub_accounts(
        &self,
        params: ListSubAccountsParams,
    ) -> Result<ListSubAccountsResponse, Box<dyn std::error::Error>> {
        let path = "/v0.1/me/accounts";
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(ref value) = params.query {
            request = request.query(&[("query", value)]);
        }
        if let Some(ref value) = params.include_primary {
            request = request.query(&[("include_primary", value)]);
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: ListSubAccountsResponse = response.json().await?;
                Ok(data)
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Create an operator
    ///
    /// Creates new operator for currently authorized users' merchant.
    pub async fn create_sub_account(
        &self,
        body: CreateSubAccountBody,
    ) -> Result<Operator, Box<dyn std::error::Error>> {
        let path = "/v0.1/me/accounts";
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .post(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout())
            .json(&body);
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Operator = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error: CompatError = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Disable an operator.
    ///
    /// Disable the specified operator for the merchant account.
    pub async fn deactivate_sub_account(
        &self,
        operator_id: impl Into<String>,
    ) -> Result<Operator, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/me/accounts/{}", operator_id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .delete(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Operator = response.json().await?;
                Ok(data)
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Retrieve an operator
    ///
    /// Returns specific operator.
    pub async fn compat_get_operator(
        &self,
        operator_id: impl Into<String>,
    ) -> Result<Operator, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/me/accounts/{}", operator_id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Operator = response.json().await?;
                Ok(data)
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Update an operator
    ///
    /// Updates operator. If the operator was disabled and their password is updated they will be unblocked.
    pub async fn update_sub_account(
        &self,
        operator_id: impl Into<String>,
        body: UpdateSubAccountBody,
    ) -> Result<Operator, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/me/accounts/{}", operator_id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .put(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout())
            .json(&body);
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Operator = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let error: CompatError = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
}
