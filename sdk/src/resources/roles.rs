// The contents of this file are generated; do not modify them.

//! Endpoints to manage custom roles. Custom roles allow you to tailor roles from individual permissions to match your needs. Once created, you can assign your custom roles to your merchant account members using the memberships.
use super::common::*;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum RoleCreated {}
impl crate::event::private::Sealed for RoleCreated {}
impl crate::events::EventSpec for RoleCreated {
    const EVENT_TYPE: &'static str = "roles.created";
    type FetchedObject = Role;
}
/// Event notification for this event type.
pub type RoleCreatedEvent = crate::events::Event<RoleCreated>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum RoleDeleted {}
impl crate::event::private::Sealed for RoleDeleted {}
impl crate::events::EventSpec for RoleDeleted {
    const EVENT_TYPE: &'static str = "roles.deleted";
    type FetchedObject = Role;
}
/// Event notification for this event type.
pub type RoleDeletedEvent = crate::events::Event<RoleDeleted>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum RoleUpdated {}
impl crate::event::private::Sealed for RoleUpdated {}
impl crate::events::EventSpec for RoleUpdated {
    const EVENT_TYPE: &'static str = "roles.updated";
    type FetchedObject = Role;
}
/// Event notification for this event type.
pub type RoleUpdatedEvent = crate::events::Event<RoleUpdated>;
/// A custom role that can be used to assign set of permissions to members.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Role {
    /// The timestamp of when the role was created.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    #[builder(setter(doc = "Sets [`created_at`](Role::created_at)."))]
    pub created_at: crate::datetime::DateTime,
    /// User-defined description of the role.
    ///
    /// Example: `Manges the shop and the employees.`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`description`](Role::description).")
    )]
    pub description: Option<String>,
    /// Unique identifier of the role.
    ///
    /// Example: `role_WZsm7QTPhVrompscmPhoGTXXcrd58fr9MOhP`
    #[builder(setter(into, doc = "Sets [`id`](Role::id)."))]
    pub id: String,
    /// True if the role is provided by SumUp.
    ///
    /// Example: `true`
    #[builder(setter(doc = "Sets [`is_predefined`](Role::is_predefined)."))]
    pub is_predefined: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`metadata`](Role::metadata).")
    )]
    pub metadata: Option<Metadata>,
    /// User-defined name of the role.
    ///
    /// Example: `Senior Shop Manager II`
    #[builder(setter(into, doc = "Sets [`name`](Role::name)."))]
    pub name: String,
    /// List of permission granted by this role.
    ///
    /// Constraints:
    /// - max items: 100
    #[builder(setter(doc = "Sets [`permissions`](Role::permissions)."))]
    pub permissions: Vec<String>,
    /// The timestamp of when the role was last updated.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    #[builder(setter(doc = "Sets [`updated_at`](Role::updated_at)."))]
    pub updated_at: crate::datetime::DateTime,
}
/// Returns a list of Role objects.
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
    pub items: Vec<Role>,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequest {
    /// User-defined description of the role.
    ///
    /// Example: `Manges the shop and the employees.`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`description`](CreateRequest::description)."
        )
    )]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`metadata`](CreateRequest::metadata).")
    )]
    pub metadata: Option<Metadata>,
    /// User-defined name of the role.
    ///
    /// Example: `Senior Shop Manager II`
    #[builder(setter(into, doc = "Sets [`name`](CreateRequest::name)."))]
    pub name: String,
    /// User's permissions.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 100
    #[builder(setter(doc = "Sets [`permissions`](CreateRequest::permissions)."))]
    pub permissions: Vec<String>,
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
    /// User-defined description of the role.
    ///
    /// Example: `Manges the shop and the employees.`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`description`](UpdateRequest::description)."
        )
    )]
    pub description: Option<String>,
    /// User-defined name of the role.
    ///
    /// Example: `Senior Shop Manager II`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`name`](UpdateRequest::name).")
    )]
    pub name: Option<String>,
    /// User's permissions.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`permissions`](UpdateRequest::permissions)."
        )
    )]
    pub permissions: Option<Vec<String>>,
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
}
impl crate::error::ProblemDetails for CreateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum DeleteErrorBody {
    BadRequest(Problem),
    NotFound(Problem),
}
impl crate::error::ProblemDetails for DeleteErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
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
    NotFound(Problem),
}
impl crate::error::ProblemDetails for UpdateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
        }
    }
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
        merchant_code: impl AsRef<str>,
    ) -> crate::error::SdkResult<ListResponse, ListErrorBody> {
        let path = format!("/v0.1/merchants/{}/roles", merchant_code.as_ref());
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(ListErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
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
        merchant_code: impl AsRef<str>,
        body: CreateRequest,
    ) -> crate::error::SdkResult<Role, CreateErrorBody> {
        let path = format!("/v0.1/merchants/{}/roles", merchant_code.as_ref());
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
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
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
        merchant_code: impl AsRef<str>,
        role_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<(), DeleteErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/roles/{}",
            merchant_code.as_ref(),
            role_id.as_ref()
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(DeleteErrorBody::BadRequest)
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
    /// Retrieve a role
    ///
    /// Retrieve a custom role by ID.
    ///
    /// Responses:
    /// - 200: Returns the Role object for a valid identifier.
    /// - 404: Merchant or role not found.
    pub async fn get(
        &self,
        merchant_code: impl AsRef<str>,
        role_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<Role, GetErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/roles/{}",
            merchant_code.as_ref(),
            role_id.as_ref()
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Update a role
    ///
    /// Updates a custom role's name, description, or permissions and returns the updated role.
    ///
    /// Providing `permissions` replaces the role's permission list and changes the access granted to members assigned to that role. Omitted fields remain unchanged.
    ///
    /// Responses:
    /// - 200: Returns the updated Role object if the update succeeded.
    /// - 400: Invalid request.
    /// - 404: Merchant not found.
    pub async fn update(
        &self,
        merchant_code: impl AsRef<str>,
        role_id: impl AsRef<str>,
        body: UpdateRequest,
    ) -> crate::error::SdkResult<Role, UpdateErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/roles/{}",
            merchant_code.as_ref(),
            role_id.as_ref()
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
