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
    pub created_at: crate::datetime::DateTime,
    /// The timestamp of when the role was last updated.
    pub updated_at: crate::datetime::DateTime,
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
#[derive(Debug)]
pub enum ListMerchantRolesErrorBody {
    NotFound(String),
}
#[derive(Debug)]
pub enum CreateMerchantRoleErrorBody {
    BadRequest(String),
    NotFound(String),
}
#[derive(Debug)]
pub enum DeleteMerchantRoleErrorBody {
    BadRequest(String),
    NotFound(String),
}
#[derive(Debug)]
pub enum GetMerchantRoleErrorBody {
    NotFound(String),
}
#[derive(Debug)]
pub enum UpdateMerchantRoleErrorBody {
    BadRequest(String),
    NotFound(String),
}
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
    ) -> crate::error::SdkResult<ListMerchantRolesResponse, ListMerchantRolesErrorBody> {
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
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListMerchantRolesResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    ListMerchantRolesErrorBody::NotFound(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
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
    ) -> crate::error::SdkResult<Role, CreateMerchantRoleErrorBody> {
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
        let status = response.status();
        match status {
            reqwest::StatusCode::CREATED => {
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    CreateMerchantRoleErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    CreateMerchantRoleErrorBody::NotFound(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
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
    ) -> crate::error::SdkResult<(), DeleteMerchantRoleErrorBody> {
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
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    DeleteMerchantRoleErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    DeleteMerchantRoleErrorBody::NotFound(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
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
    ) -> crate::error::SdkResult<Role, GetMerchantRoleErrorBody> {
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
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    GetMerchantRoleErrorBody::NotFound(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
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
    ) -> crate::error::SdkResult<Role, UpdateMerchantRoleErrorBody> {
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
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    UpdateMerchantRoleErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api(
                    UpdateMerchantRoleErrorBody::NotFound(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
}
