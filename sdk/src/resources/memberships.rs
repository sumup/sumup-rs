// The contents of this file are generated; do not modify them.

use super::common::*;
/// A membership associates a user with a resource, memberships is defined by user, resource, resource type, and associated roles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Membership {
    /// ID of the membership.
    pub id: String,
    /// ID of the resource the membership is in.
    pub resource_id: String,
    #[serde(rename = "type")]
    pub type_: ResourceType,
    /// User's roles.
    pub roles: Vec<String>,
    /// User's permissions.
    #[deprecated(
        note = "Permissions include only legacy permissions, please use roles instead. Member access is based on their roles within a given resource and the permissions these roles grant."
    )]
    pub permissions: Vec<String>,
    /// The timestamp of when the membership was created.
    pub created_at: String,
    /// The timestamp of when the membership was last updated.
    pub updated_at: String,
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
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ResourceType,
    /// Display name of the resource.
    pub name: String,
    /// Logo fo the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// The timestamp of when the membership resource was created.
    pub created_at: String,
    /// The timestamp of when the membership resource was last updated.
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}
pub type ResourceType = String;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListMembershipsParams {
    /// Offset of the first member to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Maximum number of members to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filter memberships by resource kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ResourceType>,
    /// Filter memberships by the sandbox status of the resource the membership is in.
    #[serde(rename = "resource.attributes.sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attributes_sandbox: Option<bool>,
    /// Filter memberships by the name of the resource the membership is in.
    #[serde(rename = "resource.name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
/// Returns a list of Membership objects.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListMembershipsResponse {
    pub items: Vec<Membership>,
    pub total_count: i64,
}
use crate::client::Client;
///Client for the Memberships API endpoints.
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
    pub async fn list(
        &self,
        params: ListMembershipsParams,
    ) -> Result<ListMembershipsResponse, Box<dyn std::error::Error>> {
        let path = "/v0.1/memberships";
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
        if let Some(ref value) = params.offset {
            request = request.query(&[("offset", value)]);
        }
        if let Some(ref value) = params.limit {
            request = request.query(&[("limit", value)]);
        }
        if let Some(ref value) = params.kind {
            request = request.query(&[("kind", value)]);
        }
        if let Some(ref value) = params.resource_attributes_sandbox {
            request = request.query(&[("resource.attributes.sandbox", value)]);
        }
        if let Some(ref value) = params.resource_name {
            request = request.query(&[("resource.name", value)]);
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: ListMembershipsResponse = response.json().await?;
                Ok(data)
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
}
