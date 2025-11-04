// The contents of this file are generated; do not modify them.

use super::common::*;
/// A member is user within specific resource identified by resource id, resource type, and associated roles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Member {
    /// ID of the member.
    pub id: String,
    /// User's roles.
    pub roles: Vec<String>,
    /// User's permissions.
    #[deprecated(
        note = "Permissions include only legacy permissions, please use roles instead. Member access is based on roles within a given resource and the permissions these roles grant."
    )]
    pub permissions: Vec<String>,
    /// The timestamp of when the member was created.
    pub created_at: crate::datetime::DateTime,
    /// The timestamp of when the member was last updated.
    pub updated_at: crate::datetime::DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<MembershipUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite: Option<Invite>,
    pub status: MembershipStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}
/// Information about the user associated with the membership.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MembershipUser {
    /// Identifier for the End-User (also called Subject).
    pub id: String,
    /// End-User's preferred e-mail address. Its value MUST conform to the RFC 5322 [RFC5322] addr-spec syntax. The RP MUST NOT rely upon this value being unique, for unique identification use ID instead.
    pub email: String,
    /// True if the user has enabled MFA on login.
    pub mfa_on_login_enabled: bool,
    /// True if the user is a virtual user (operator).
    pub virtual_user: bool,
    /// True if the user is a service account.
    pub service_account_user: bool,
    /// Time when the user has been disabled. Applies only to virtual users (`virtual_user: true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option<crate::datetime::DateTime>,
    /// User's preferred name. Used for display purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// URL of the End-User's profile picture. This URL refers to an image file (for example, a PNG, JPEG, or GIF image file), rather than to a Web page containing an image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic: Option<MembershipUserClassic>,
}
/// Classic identifiers of the user.
#[deprecated]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MembershipUserClassic {
    pub user_id: i32,
}
/// Allows you to update user data of managed users.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateMerchantMemberBodyUser {
    /// User's preferred name. Used for display purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Password of the member to add. Only used if `is_managed_user` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<crate::secret::Password>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListMerchantMembersParams {
    /// Offset of the first member to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Maximum number of members to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Indicates to skip count query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll: Option<bool>,
    /// Filter the returned members by email address prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Search for a member by user id.
    #[serde(rename = "user.id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Filter the returned members by the membership status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MembershipStatus>,
    /// Filter the returned members by role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}
/// Returns a list of Member objects.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListMerchantMembersResponse {
    pub items: Vec<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMerchantMemberBody {
    /// True if the user is managed by the merchant. In this case, we'll created a virtual user with the provided password and nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_managed_user: Option<bool>,
    /// Email address of the member to add.
    pub email: String,
    /// Password of the member to add. Only used if `is_managed_user` is true. In the case of service accounts, the password is not used and can not be defined by the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<crate::secret::Password>,
    /// Nickname of the member to add. Only used if `is_managed_user` is true. Used for display purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// List of roles to assign to the new member.
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateMerchantMemberBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
    /// Allows you to update user data of managed users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UpdateMerchantMemberBodyUser>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum ListMerchantMembersErrorBody {
    NotFound,
}
#[derive(Debug)]
pub enum CreateMerchantMemberErrorBody {
    BadRequest,
    NotFound,
    TooManyRequests,
}
#[derive(Debug)]
pub enum DeleteMerchantMemberErrorBody {
    NotFound,
}
#[derive(Debug)]
pub enum GetMerchantMemberErrorBody {
    NotFound,
}
#[derive(Debug)]
pub enum UpdateMerchantMemberErrorBody {
    BadRequest,
    Forbidden,
    NotFound,
    Conflict,
}
///Client for the Members API endpoints.
#[derive(Debug)]
pub struct MembersClient<'a> {
    client: &'a Client,
}
impl<'a> MembersClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// List members
    ///
    /// Lists merchant members.
    pub async fn list(
        &self,
        merchant_code: impl Into<String>,
        params: ListMerchantMembersParams,
    ) -> crate::error::SdkResult<ListMerchantMembersResponse, ListMerchantMembersErrorBody> {
        let path = format!("/v0.1/merchants/{}/members", merchant_code.into());
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
        if let Some(ref value) = params.scroll {
            request = request.query(&[("scroll", value)]);
        }
        if let Some(ref value) = params.email {
            request = request.query(&[("email", value)]);
        }
        if let Some(ref value) = params.user_id {
            request = request.query(&[("user.id", value)]);
        }
        if let Some(ref value) = params.status {
            request = request.query(&[("status", value)]);
        }
        if let Some(ref value) = params.roles {
            request = request.query(&[("roles", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListMerchantMembersResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => Err(crate::error::SdkError::api(
                ListMerchantMembersErrorBody::NotFound,
            )),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Create a member
    ///
    /// Create a merchant member.
    pub async fn create(
        &self,
        merchant_code: impl Into<String>,
        body: CreateMerchantMemberBody,
    ) -> crate::error::SdkResult<Member, CreateMerchantMemberErrorBody> {
        let path = format!("/v0.1/merchants/{}/members", merchant_code.into());
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
                let data: Member = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => Err(crate::error::SdkError::api(
                CreateMerchantMemberErrorBody::BadRequest,
            )),
            reqwest::StatusCode::NOT_FOUND => Err(crate::error::SdkError::api(
                CreateMerchantMemberErrorBody::NotFound,
            )),
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(crate::error::SdkError::api(
                CreateMerchantMemberErrorBody::TooManyRequests,
            )),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Delete a member
    ///
    /// Deletes a merchant member.
    pub async fn delete(
        &self,
        merchant_code: impl Into<String>,
        member_id: impl Into<String>,
    ) -> crate::error::SdkResult<(), DeleteMerchantMemberErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code.into(),
            member_id.into()
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
            reqwest::StatusCode::NOT_FOUND => Err(crate::error::SdkError::api(
                DeleteMerchantMemberErrorBody::NotFound,
            )),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Retrieve a member
    ///
    /// Retrieve a merchant member.
    pub async fn get(
        &self,
        merchant_code: impl Into<String>,
        member_id: impl Into<String>,
    ) -> crate::error::SdkResult<Member, GetMerchantMemberErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code.into(),
            member_id.into()
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
                let data: Member = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => Err(crate::error::SdkError::api(
                GetMerchantMemberErrorBody::NotFound,
            )),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Update a member
    ///
    /// Update the merchant member.
    pub async fn update(
        &self,
        merchant_code: impl Into<String>,
        member_id: impl Into<String>,
        body: UpdateMerchantMemberBody,
    ) -> crate::error::SdkResult<Member, UpdateMerchantMemberErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code.into(),
            member_id.into()
        );
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
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Member = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => Err(crate::error::SdkError::api(
                UpdateMerchantMemberErrorBody::BadRequest,
            )),
            reqwest::StatusCode::FORBIDDEN => Err(crate::error::SdkError::api(
                UpdateMerchantMemberErrorBody::Forbidden,
            )),
            reqwest::StatusCode::NOT_FOUND => Err(crate::error::SdkError::api(
                UpdateMerchantMemberErrorBody::NotFound,
            )),
            reqwest::StatusCode::CONFLICT => Err(crate::error::SdkError::api(
                UpdateMerchantMemberErrorBody::Conflict,
            )),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
}
