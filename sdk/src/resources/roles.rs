// The contents of this file are generated; do not modify them.

use super::common::*;
/// A custom role that can be used to assign set of permissions to members.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Role {
    /// Unique identifier of the role.
    pub id: String,
    /// User-defined name of the role.
    pub name: String,
    /// User-defined description of the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of permission granted by this role.
    pub permissions: Vec<String>,
    /// True if the role is provided by SumUp.
    pub is_predefined: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The timestamp of when the role was created.
    pub created_at: String,
    /// The timestamp of when the role was last updated.
    pub updated_at: String,
}
/// Returns a list of Role objects.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListMerchantRolesResponse {
    pub items: Vec<Role>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMerchantRoleBody {
    /// User-defined name of the role.
    pub name: String,
    /// User's permissions.
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// User-defined description of the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateMerchantRoleBody {
    /// User-defined name of the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User's permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// User-defined description of the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
use crate::client::Client;
///Client for the Roles API endpoints.
#[derive(Debug)]
pub struct RolesClient<'a> {
    client: &'a Client,
}
impl<'a> RolesClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// List roles
    ///
    /// List merchant's custom roles.
    pub async fn list(
        &self,
        merchant_code: impl Into<String>,
    ) -> Result<ListMerchantRolesResponse, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/merchants/{}/roles", merchant_code.into());
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
                let data: ListMerchantRolesResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Merchant not found.", body).into())
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Create a role
    ///
    /// Create a custom role for the merchant. Roles are defined by the set of permissions that they grant to the members that they are assigned to.
    pub async fn create(
        &self,
        merchant_code: impl Into<String>,
        body: CreateMerchantRoleBody,
    ) -> Result<Role, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/merchants/{}/roles", merchant_code.into());
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
            reqwest::StatusCode::CREATED => {
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Invalid request.", body).into())
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Merchant not found.", body).into())
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Delete a role
    ///
    /// Delete a custom role.
    pub async fn delete(
        &self,
        merchant_code: impl Into<String>,
        role_id: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!(
            "/v0.1/merchants/{}/roles/{}",
            merchant_code.into(),
            role_id.into()
        );
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
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Invalid request.", body).into())
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Merchant not found.", body).into())
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Retrieve a role
    ///
    /// Retrieve a custom role by ID.
    pub async fn get(
        &self,
        merchant_code: impl Into<String>,
        role_id: impl Into<String>,
    ) -> Result<Role, Box<dyn std::error::Error>> {
        let path = format!(
            "/v0.1/merchants/{}/roles/{}",
            merchant_code.into(),
            role_id.into()
        );
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
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Merchant or role not found.", body).into())
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
    /// Update a role
    ///
    /// Update a custom role.
    pub async fn update(
        &self,
        merchant_code: impl Into<String>,
        role_id: impl Into<String>,
        body: UpdateMerchantRoleBody,
    ) -> Result<Role, Box<dyn std::error::Error>> {
        let path = format!(
            "/v0.1/merchants/{}/roles/{}",
            merchant_code.into(),
            role_id.into()
        );
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .patch(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout())
            .json(&body);
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Invalid request.", body).into())
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(format!("{}: {}", "Merchant not found.", body).into())
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
}
