// The contents of this file are generated; do not modify them.

//! Endpoints to manage user's memberships. Memberships are used to connect the user to merchant accounts and to grant them access to the merchant's resources via roles.
use super::common::*;
/// A membership associates a user with a resource, memberships is defined by user, resource, resource type, and associated roles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Membership {
    /// ID of the membership.
    ///
    /// Example: `mem_WZsm7QTPhVrompscmPhoGTXXcrd58fr9MOhP`
    pub id: String,
    /// ID of the resource the membership is in.
    ///
    /// Example: `M2DDT39A`
    pub resource_id: String,
    #[serde(rename = "type")]
    pub r#type: ResourceType,
    /// User's roles.
    pub roles: Vec<String>,
    /// User's permissions.
    #[deprecated(
        note = "Permissions include only legacy permissions, please use roles instead. Member access is based on their roles within a given resource and the permissions these roles grant."
    )]
    pub permissions: Vec<String>,
    /// The timestamp of when the membership was created.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    pub created_at: crate::datetime::DateTime,
    /// The timestamp of when the membership was last updated.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    pub updated_at: crate::datetime::DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite: Option<Invite>,
    pub status: MembershipStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
    pub resource: MembershipResource,
}
/// Information about the resource the membership is in.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MembershipResource {
    /// ID of the resource the membership is in.
    ///
    /// Example: `M2DDT39A`
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ResourceType,
    /// Display name of the resource.
    ///
    /// Example: `Acme Corp`
    pub name: String,
    /// Logo fo the resource.
    ///
    /// Constraints:
    /// - format: `uri`
    /// - max length: 256
    ///
    /// Example: `https://images.sumup.com/img_2x4y6z8a0b1c2d3e4f5g6h7j8k.png`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// The timestamp of when the membership resource was created.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    pub created_at: crate::datetime::DateTime,
    /// The timestamp of when the membership resource was last updated.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    pub updated_at: crate::datetime::DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}
pub type ResourceType = String;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListParams {
    /// Offset of the first member to return.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Maximum number of members to return.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 25
    ///
    /// Example: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filter memberships by resource kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ResourceType>,
    /// Filter the returned memberships by the membership status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MembershipStatus>,
    /// Filter memberships by resource kind.
    #[serde(rename = "resource.type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceType>,
    /// Filter memberships by the sandbox status of the resource the membership is in.
    #[serde(rename = "resource.attributes.sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attributes_sandbox: Option<bool>,
    /// Filter memberships by the name of the resource the membership is in.
    #[serde(rename = "resource.name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// Filter memberships by the parent of the resource the membership is in.
    /// When filtering by parent both `resource.parent.id` and `resource.parent.type` must be present. Pass explicit null to filter for resources without a parent.
    #[serde(rename = "resource.parent.id")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub resource_parent_id: Option<crate::Nullable<String>>,
    /// Filter memberships by the parent of the resource the membership is in.
    /// When filtering by parent both `resource.parent.id` and `resource.parent.type` must be present. Pass explicit null to filter for resources without a parent.
    #[serde(rename = "resource.parent.type")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub resource_parent_type: Option<crate::Nullable<String>>,
    /// Filter the returned memberships by role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}
/// Returns a list of Membership objects.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    pub items: Vec<Membership>,
    pub total_count: i64,
}
use crate::client::Client;
#[derive(Debug)]
pub enum ListErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
}
/// Client for the Memberships API endpoints.
#[derive(Debug)]
pub struct MembershipsClient<'a> {
    client: &'a Client,
}
impl<'a> MembershipsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// List memberships
    ///
    /// List memberships of the current user.
    ///
    /// Responses:
    /// - 200: Returns a list of Membership objects.
    /// - 400: Invalid query parameter combination.
    /// - 401: Authentication failed or missing required scope.
    pub async fn list(
        &self,
        params: ListParams,
    ) -> crate::error::SdkResult<ListResponse, ListErrorBody> {
        let path = "/v0.1/memberships";
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
        if let Some(ref value) = params.offset {
            request = request.query(&[("offset", value)]);
        }
        if let Some(ref value) = params.limit {
            request = request.query(&[("limit", value)]);
        }
        if let Some(ref value) = params.kind {
            request = request.query(&[("kind", value)]);
        }
        if let Some(ref value) = params.status {
            request = request.query(&[("status", value)]);
        }
        if let Some(ref value) = params.resource_type {
            request = request.query(&[("resource.type", value)]);
        }
        if let Some(ref value) = params.resource_attributes_sandbox {
            request = request.query(&[("resource.attributes.sandbox", value)]);
        }
        if let Some(ref value) = params.resource_name {
            request = request.query(&[("resource.name", value)]);
        }
        if let Some(ref value) = params.resource_parent_id {
            match value {
                crate::Nullable::Null => {
                    request = request.query(&[("resource.parent.id", "null")]);
                }
                crate::Nullable::Value(ref v) => {
                    request = request.query(&[("resource.parent.id", v)]);
                }
            }
        }
        if let Some(ref value) = params.resource_parent_type {
            match value {
                crate::Nullable::Null => {
                    request = request.query(&[("resource.parent.type", "null")]);
                }
                crate::Nullable::Value(ref v) => {
                    request = request.query(&[("resource.parent.type", v)]);
                }
            }
        }
        if let Some(ref value) = params.roles {
            request = request.query(&[("roles", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(ListErrorBody::BadRequest(body)))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(ListErrorBody::Unauthorized(
                    body,
                )))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
}
