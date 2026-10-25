// The contents of this file are generated; do not modify them.

//! Endpoints to manage account members. Members are users that have membership within merchant accounts.
use super::common::*;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum MemberCreated {}
impl crate::event::private::Sealed for MemberCreated {}
impl crate::events::EventSpec for MemberCreated {
    const EVENT_TYPE: &'static str = "members.created";
    type FetchedObject = Member;
}
/// Event notification for this event type.
pub type MemberCreatedEvent = crate::events::Event<MemberCreated>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum MemberDeleted {}
impl crate::event::private::Sealed for MemberDeleted {}
impl crate::events::EventSpec for MemberDeleted {
    const EVENT_TYPE: &'static str = "members.deleted";
    type FetchedObject = Member;
}
/// Event notification for this event type.
pub type MemberDeletedEvent = crate::events::Event<MemberDeleted>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum MemberUpdated {}
impl crate::event::private::Sealed for MemberUpdated {}
impl crate::events::EventSpec for MemberUpdated {
    const EVENT_TYPE: &'static str = "members.updated";
    type FetchedObject = Member;
}
/// Event notification for this event type.
pub type MemberUpdatedEvent = crate::events::Event<MemberUpdated>;
/// A member is user within specific resource identified by resource id, resource type, and associated roles.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`attributes`](Member::attributes).")
    )]
    pub attributes: Option<Attributes>,
    /// The timestamp of when the member was created.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    #[builder(setter(doc = "Sets [`created_at`](Member::created_at)."))]
    pub created_at: crate::datetime::DateTime,
    /// ID of the member.
    ///
    /// Example: `mem_WZsm7QTPhVrompscmPhoGTXXcrd58fr9MOhP`
    #[builder(setter(into, doc = "Sets [`id`](Member::id)."))]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`invite`](Member::invite).")
    )]
    pub invite: Option<Invite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`metadata`](Member::metadata).")
    )]
    pub metadata: Option<Metadata>,
    /// User's permissions.
    #[deprecated(
        note = "Permissions include only legacy permissions, please use roles instead. Member access is based on roles within a given resource and the permissions these roles grant."
    )]
    #[builder(setter(doc = "Sets [`permissions`](Member::permissions)."))]
    pub permissions: Vec<String>,
    /// User's roles.
    #[builder(setter(doc = "Sets [`roles`](Member::roles)."))]
    pub roles: Vec<String>,
    #[builder(setter(doc = "Sets [`status`](Member::status)."))]
    pub status: MembershipStatus,
    /// The timestamp of when the member was last updated.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    #[builder(setter(doc = "Sets [`updated_at`](Member::updated_at)."))]
    pub updated_at: crate::datetime::DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, doc = "Sets [`user`](Member::user)."))]
    pub user: Option<MembershipUser>,
}
/// Information about the user associated with the membership.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct MembershipUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`classic`](MembershipUser::classic).")
    )]
    pub classic: Option<MembershipUserClassic>,
    /// Time when the user has been disabled. Applies only to virtual users (`virtual_user: true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`disabled_at`](MembershipUser::disabled_at)."
        )
    )]
    pub disabled_at: Option<crate::datetime::DateTime>,
    /// End-User's preferred e-mail address. Its value MUST conform to the RFC 5322 [RFC5322] addr-spec syntax. The RP MUST NOT rely upon this value being unique, for unique identification use ID instead.
    ///
    /// Example: `example@sumup.com`
    #[builder(setter(into, doc = "Sets [`email`](MembershipUser::email)."))]
    pub email: String,
    /// Identifier for the End-User (also called Subject).
    ///
    /// Example: `44ca0f5b-813b-46e1-aee7-e6242010662e`
    #[builder(setter(into, doc = "Sets [`id`](MembershipUser::id)."))]
    pub id: String,
    /// True if the user has enabled MFA on login.
    ///
    /// Example: `true`
    #[builder(setter(
        doc = "Sets [`mfa_on_login_enabled`](MembershipUser::mfa_on_login_enabled)."
    ))]
    pub mfa_on_login_enabled: bool,
    /// User's nickname. Used for display purposes only.
    ///
    /// Example: `Test User`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`nickname`](MembershipUser::nickname)."
        )
    )]
    pub nickname: Option<String>,
    /// URL of the End-User's profile picture. This URL refers to an image file (for example, a PNG, JPEG, or GIF image file), rather than to a Web page containing an image.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `https://usercontent.sumup.com/44ca0f5b-813b-46e1-aee7-e6242010662e.png`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`picture`](MembershipUser::picture).")
    )]
    pub picture: Option<String>,
    /// True if the user is a service account.
    ///
    /// Example: `false`
    #[deprecated(note = "Rely on `type` instead.")]
    #[builder(setter(
        doc = "Sets [`service_account_user`](MembershipUser::service_account_user)."
    ))]
    pub service_account_user: bool,
    #[serde(rename = "type")]
    #[builder(setter(doc = "Sets [`r#type`](MembershipUser::r#type)."))]
    pub r#type: UserType,
    /// True if the user is a virtual user (operator).
    ///
    /// Example: `false`
    #[deprecated(note = "Rely on `type` instead.")]
    #[builder(setter(doc = "Sets [`virtual_user`](MembershipUser::virtual_user)."))]
    pub virtual_user: bool,
}
/// Classic identifiers of the user.
#[deprecated]
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct MembershipUserClassic {
    #[builder(setter(doc = "Sets [`user_id`](MembershipUserClassic::user_id)."))]
    pub user_id: i64,
}
/// Type of the user account.
///
/// Example: `user`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UserType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "managed_user")]
    ManagedUser,
    #[serde(rename = "service_account")]
    ServiceAccount,
    #[serde(rename = "system_account")]
    SystemAccount,
    #[serde(untagged)]
    Other(String),
}
/// Allows you to update user data of managed users.
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
pub struct UpdateRequestUser {
    /// User's nickname. Used for display purposes only.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `Test User`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`nickname`](UpdateRequestUser::nickname)."
        )
    )]
    pub nickname: Option<String>,
    /// Password of the member to add. Only used if `is_managed_user` is true.
    ///
    /// Constraints:
    /// - min length: 8
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`password`](UpdateRequestUser::password).")
    )]
    pub password: Option<crate::secret::Secret>,
}
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
pub struct ListParams {
    /// Offset of the first member to return.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`offset`](ListParams::offset).")
    )]
    pub offset: Option<i64>,
    /// Maximum number of members to return.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 25
    ///
    /// Example: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`limit`](ListParams::limit).")
    )]
    pub limit: Option<i64>,
    /// Indicates to skip count query.
    ///
    /// Example: `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`scroll`](ListParams::scroll).")
    )]
    pub scroll: Option<bool>,
    /// Filter the returned members by email address prefix.
    ///
    /// Example: `user`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`email`](ListParams::email).")
    )]
    pub email: Option<String>,
    /// Search for a member by user id.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `245b2ead-85bf-45ff-856f-311a88a5d454`
    #[serde(rename = "user.id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`user_id`](ListParams::user_id).")
    )]
    pub user_id: Option<String>,
    /// Filter the returned members by the membership status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`status`](ListParams::status).")
    )]
    pub status: Option<MembershipStatus>,
    /// Filter the returned members by role.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`roles`](ListParams::roles).")
    )]
    pub roles: Option<Vec<String>>,
}
/// Returns a list of Member objects.
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
pub struct ListResponse {
    #[builder(setter(doc = "Sets [`items`](ListResponse::items)."))]
    pub items: Vec<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`total_count`](ListResponse::total_count).")
    )]
    pub total_count: Option<i64>,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`attributes`](CreateRequest::attributes).")
    )]
    pub attributes: Option<Attributes>,
    /// Email address of the member to add.
    ///
    /// Constraints:
    /// - format: `email`
    /// - max length: 256
    #[builder(setter(into, doc = "Sets [`email`](CreateRequest::email)."))]
    pub email: String,
    /// True if the user is managed by the merchant. In this case, we'll created a virtual user with the provided password and nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`is_managed_user`](CreateRequest::is_managed_user)."
        )
    )]
    pub is_managed_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`metadata`](CreateRequest::metadata).")
    )]
    pub metadata: Option<Metadata>,
    /// Nickname of the member to add. Only used if `is_managed_user` is true. Used for display purposes only.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `Test User`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`nickname`](CreateRequest::nickname)."
        )
    )]
    pub nickname: Option<String>,
    /// Password of the member to add. Only used if `is_managed_user` is true. In the case of service accounts, the password is not used and can not be defined by the caller.
    ///
    /// Constraints:
    /// - min length: 8
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`password`](CreateRequest::password).")
    )]
    pub password: Option<crate::secret::Secret>,
    /// List of roles to assign to the new member.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 124
    #[builder(setter(doc = "Sets [`roles`](CreateRequest::roles)."))]
    pub roles: Vec<String>,
}
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
pub struct UpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`attributes`](UpdateRequest::attributes).")
    )]
    pub attributes: Option<Attributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`metadata`](UpdateRequest::metadata).")
    )]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`roles`](UpdateRequest::roles).")
    )]
    pub roles: Option<Vec<String>>,
    /// Allows you to update user data of managed users.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`user`](UpdateRequest::user).")
    )]
    pub user: Option<UpdateRequestUser>,
}
use crate::client::Client;
#[derive(Debug, PartialEq)]
pub enum ListErrorBody {
    NotFound(Problem),
}
impl crate::error::ProblemDetails for ListErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum CreateErrorBody {
    BadRequest(Problem),
    NotFound(Problem),
    TooManyRequests(Problem),
}
impl crate::error::ProblemDetails for CreateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::TooManyRequests(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum DeleteErrorBody {
    Forbidden(Problem),
    NotFound(Problem),
}
impl crate::error::ProblemDetails for DeleteErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Forbidden(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum GetErrorBody {
    NotFound(Problem),
}
impl crate::error::ProblemDetails for GetErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum UpdateErrorBody {
    BadRequest(Problem),
    Forbidden(Problem),
    NotFound(Problem),
    Conflict(Problem),
}
impl crate::error::ProblemDetails for UpdateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Forbidden(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::Conflict(problem) => Some(problem),
        }
    }
}
/// Client for the Members API endpoints.
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
    ///
    /// Responses:
    /// - 200: Returns a list of Member objects.
    /// - 404: Merchant not found.
    pub async fn list(
        &self,
        merchant_code: impl AsRef<str>,
        params: ListParams,
    ) -> crate::error::SdkResult<ListResponse, ListErrorBody> {
        let path = format!("/v0.1/merchants/{}/members", merchant_code.as_ref());
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
                let data: ListResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(ListErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Create a member
    ///
    /// Adds a member to the merchant account with the specified roles.
    ///
    /// By default, sends an invitation email to the provided address. The recipient must accept the invitation to join the account.
    /// When `is_managed_user` is `true`, creates a managed user with the provided password and optional nickname and assigns the roles directly, without sending an invitation.
    ///
    /// Responses:
    /// - 201: Returns the Member object if the creation succeeded.
    /// - 400: Invalid request.
    /// - 404: Merchant not found.
    /// - 429: Too many invitations were sent to that user and the rate limit was exceeded. The
    ///   Retry-After header indicates when the client can retry.
    pub async fn create(
        &self,
        merchant_code: impl AsRef<str>,
        body: CreateRequest,
    ) -> crate::error::SdkResult<Member, CreateErrorBody> {
        let path = format!("/v0.1/merchants/{}/members", merchant_code.as_ref());
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
                let data: Member = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::TooManyRequests)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Delete a member
    ///
    /// Deletes a merchant member.
    ///
    /// Responses:
    /// - 200: Returns an empty response if the deletion succeeded.
    /// - 403: Member deletion was forbidden.
    /// - 404: Merchant or member not found.
    pub async fn delete(
        &self,
        merchant_code: impl AsRef<str>,
        member_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<(), DeleteErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code.as_ref(),
            member_id.as_ref()
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
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(DeleteErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(DeleteErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Retrieve a member
    ///
    /// Retrieve a merchant member.
    ///
    /// Responses:
    /// - 200: Returns the Member object for a valid identifier.
    /// - 404: Merchant or member not found.
    pub async fn get(
        &self,
        merchant_code: impl AsRef<str>,
        member_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<Member, GetErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code.as_ref(),
            member_id.as_ref()
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
                let data: Member = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Update a member
    ///
    /// Updates a merchant member and returns the updated member.
    ///
    /// Providing `roles` replaces the member's assigned roles and can grant or revoke access. Providing `metadata` replaces the entire metadata object.
    /// For managed users, `user.nickname` changes the display name and `user.password` replaces the password. Updating the password also enables the managed user account.
    ///
    /// Responses:
    /// - 200: Returns the updated Member object if the update succeeded.
    /// - 400: Cannot set password or nickname for an invited user.
    /// - 403: Cannot change password for managed user. Password was already used before.
    /// - 404: Merchant or member not found.
    /// - 409: Cannot update member as some data conflict with existing members.
    pub async fn update(
        &self,
        merchant_code: impl AsRef<str>,
        member_id: impl AsRef<str>,
        body: UpdateRequest,
    ) -> crate::error::SdkResult<Member, UpdateErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code.as_ref(),
            member_id.as_ref()
        );
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .put(&url)
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
                let data: Member = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::CONFLICT => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::Conflict)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
