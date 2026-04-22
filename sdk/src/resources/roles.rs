// The contents of this file are generated; do not modify them.

//! Endpoints to manage custom roles. Custom roles allow you to tailor roles from individual permissions to match your needs. Once created, you can assign your custom roles to your merchant account members using the memberships.
use super::common::*;
/// A custom role that can be used to assign set of permissions to members.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Role {
    /// Unique identifier of the role.
    ///
    /// Example: `role_WZsm7QTPhVrompscmPhoGTXXcrd58fr9MOhP`
    pub id: String,
    /// User-defined name of the role.
    ///
    /// Example: `Senior Shop Manager II`
    pub name: String,
    /// User-defined description of the role.
    ///
    /// Example: `Manges the shop and the employees.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of permission granted by this role.
    ///
    /// Constraints:
    /// - max items: 100
    pub permissions: Vec<String>,
    /// True if the role is provided by SumUp.
    ///
    /// Example: `true`
    pub is_predefined: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The timestamp of when the role was created.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    pub created_at: crate::datetime::DateTime,
    /// The timestamp of when the role was last updated.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    pub updated_at: crate::datetime::DateTime,
}
/// Returns a list of Role objects.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    pub items: Vec<Role>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateBody {
    /// User-defined name of the role.
    ///
    /// Example: `Senior Shop Manager II`
    pub name: String,
    /// User's permissions.
    ///
    /// Constraints:
    /// - max items: 100
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// User-defined description of the role.
    ///
    /// Example: `Manges the shop and the employees.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateBody {
    /// User-defined name of the role.
    ///
    /// Example: `Senior Shop Manager II`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User's permissions.
    ///
    /// Constraints:
    /// - max items: 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// User-defined description of the role.
    ///
    /// Example: `Manges the shop and the employees.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum ListErrorBody {
    NotFound(Problem),
}
#[derive(Debug)]
pub enum CreateErrorBody {
    BadRequest(Problem),
    NotFound(Problem),
}
#[derive(Debug)]
pub enum DeleteErrorBody {
    BadRequest(Problem),
    NotFound(Problem),
}
#[derive(Debug)]
pub enum GetErrorBody {
    NotFound(Problem),
}
#[derive(Debug)]
pub enum UpdateErrorBody {
    BadRequest(Problem),
    NotFound(Problem),
}
/// Client for the Roles API endpoints.
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
    ///
    /// Responses:
    /// - 200: Returns a list of Role objects.
    /// - 404: Merchant not found.
    pub async fn list(
        &self,
        merchant_code: impl Into<String>,
    ) -> crate::error::SdkResult<ListResponse, ListErrorBody> {
        let path = format!("/v0.1/merchants/{}/roles", merchant_code.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
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
                let data: ListResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(ListErrorBody::NotFound(body)))
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
    ///
    /// Responses:
    /// - 201: Returns the Role object after successful custom role creation.
    /// - 400: Invalid request.
    /// - 404: Merchant not found.
    pub async fn create(
        &self,
        merchant_code: impl Into<String>,
        body: CreateBody,
    ) -> crate::error::SdkResult<Role, CreateErrorBody> {
        let path = format!("/v0.1/merchants/{}/roles", merchant_code.into());
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
            reqwest::StatusCode::CREATED => {
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(CreateErrorBody::BadRequest(
                    body,
                )))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(CreateErrorBody::NotFound(body)))
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
    ///
    /// Responses:
    /// - 200: Returns an empty response if the role deletion succeeded.
    /// - 400: Invalid request.
    /// - 404: Merchant not found.
    pub async fn delete(
        &self,
        merchant_code: impl Into<String>,
        role_id: impl Into<String>,
    ) -> crate::error::SdkResult<(), DeleteErrorBody> {
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
        if let Some(authorization) = self.client.authorization() {
            request = request.header("Authorization", format!("Bearer {}", authorization));
        }
        for (header_name, header_value) in self.client.runtime_headers() {
            request = request.header(*header_name, header_value);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(DeleteErrorBody::BadRequest(
                    body,
                )))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(DeleteErrorBody::NotFound(body)))
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
    ///
    /// Responses:
    /// - 200: Returns the Role object for a valid identifier.
    /// - 404: Merchant or role not found.
    pub async fn get(
        &self,
        merchant_code: impl Into<String>,
        role_id: impl Into<String>,
    ) -> crate::error::SdkResult<Role, GetErrorBody> {
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
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(GetErrorBody::NotFound(body)))
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
    ///
    /// Responses:
    /// - 200: Returns the updated Role object if the update succeeded.
    /// - 400: Invalid request.
    /// - 404: Merchant not found.
    pub async fn update(
        &self,
        merchant_code: impl Into<String>,
        role_id: impl Into<String>,
        body: UpdateBody,
    ) -> crate::error::SdkResult<Role, UpdateErrorBody> {
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
                let data: Role = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(UpdateErrorBody::BadRequest(
                    body,
                )))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(UpdateErrorBody::NotFound(body)))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
}
