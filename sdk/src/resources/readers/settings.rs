// The contents of this file are generated; do not modify them.

//! Retrieve and update card reader settings.
pub use crate::resources::common::*;
/// Menu lock settings reported by the reader, or the last saved settings when offline.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct GetReaderSettingsResponse {
    #[builder(setter(doc = "Sets [`data`](GetReaderSettingsResponse::data)."))]
    pub data: GetReaderSettingsResponseData,
}
/// Menu lock settings to update for a Solo reader.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct UpdateReaderSettingsRequest {
    /// Menu lock configuration. Provide at least one of `enabled` or `passcode`.
    ///
    /// Constraints:
    /// - min properties: 1
    #[builder(setter(doc = "Sets [`menu_lock`](UpdateReaderSettingsRequest::menu_lock)."))]
    pub menu_lock: UpdateReaderSettingsRequestMenuLock,
}
/// Menu lock settings applied to the reader.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct UpdateReaderSettingsResponse {
    #[builder(setter(doc = "Sets [`data`](UpdateReaderSettingsResponse::data)."))]
    pub data: UpdateReaderSettingsResponseData,
}
/// Menu lock configuration. Fields may be absent if no value is available.
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
pub struct GetReaderSettingsResponseDataMenuLock {
    /// Whether menu lock is enabled.
    ///
    /// Example: `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`enabled`](GetReaderSettingsResponseDataMenuLock::enabled)."
        )
    )]
    pub enabled: Option<bool>,
    /// Four-digit PIN used to unlock the reader's menu.
    ///
    /// Constraints:
    /// - pattern: `^[0-9]{4}$`
    ///
    /// Example: `1234`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`passcode`](GetReaderSettingsResponseDataMenuLock::passcode)."
        )
    )]
    pub passcode: Option<crate::secret::Secret>,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct GetReaderSettingsResponseData {
    /// Menu lock configuration. Fields may be absent if no value is available.
    #[builder(setter(doc = "Sets [`menu_lock`](GetReaderSettingsResponseData::menu_lock)."))]
    pub menu_lock: GetReaderSettingsResponseDataMenuLock,
}
/// Menu lock configuration. Provide at least one of `enabled` or `passcode`.
///
/// Constraints:
/// - min properties: 1
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
pub struct UpdateReaderSettingsRequestMenuLock {
    /// Whether menu lock is enabled.
    ///
    /// Example: `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`enabled`](UpdateReaderSettingsRequestMenuLock::enabled)."
        )
    )]
    pub enabled: Option<bool>,
    /// Four-digit PIN used to unlock the reader's menu.
    ///
    /// Constraints:
    /// - pattern: `^[0-9]{4}$`
    ///
    /// Example: `1234`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`passcode`](UpdateReaderSettingsRequestMenuLock::passcode)."
        )
    )]
    pub passcode: Option<crate::secret::Secret>,
}
/// Settings included in the update. Omitted fields are not returned.
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
pub struct UpdateReaderSettingsResponseDataMenuLock {
    /// Whether menu lock is enabled.
    ///
    /// Example: `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`enabled`](UpdateReaderSettingsResponseDataMenuLock::enabled)."
        )
    )]
    pub enabled: Option<bool>,
    /// Four-digit PIN used to unlock the reader's menu.
    ///
    /// Constraints:
    /// - pattern: `^[0-9]{4}$`
    ///
    /// Example: `1234`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`passcode`](UpdateReaderSettingsResponseDataMenuLock::passcode)."
        )
    )]
    pub passcode: Option<crate::secret::Secret>,
}
/// Indicates that the reader applied the settings update.
///
/// Example: `applied`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateReaderSettingsResponseDataStatus {
    #[serde(rename = "applied")]
    Applied,
    #[serde(untagged)]
    Other(String),
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct UpdateReaderSettingsResponseData {
    /// Settings included in the update. Omitted fields are not returned.
    #[builder(setter(doc = "Sets [`menu_lock`](UpdateReaderSettingsResponseData::menu_lock)."))]
    pub menu_lock: UpdateReaderSettingsResponseDataMenuLock,
    /// Indicates that the reader applied the settings update.
    ///
    /// Example: `applied`
    #[builder(setter(doc = "Sets [`status`](UpdateReaderSettingsResponseData::status)."))]
    pub status: UpdateReaderSettingsResponseDataStatus,
}
/// Menu lock configuration. Provide at least one of `enabled` or `passcode`.
///
/// Constraints:
/// - min properties: 1
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
pub struct UpdateRequestMenuLock {
    /// Whether menu lock is enabled.
    ///
    /// Example: `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`enabled`](UpdateRequestMenuLock::enabled)."
        )
    )]
    pub enabled: Option<bool>,
    /// Four-digit PIN used to unlock the reader's menu.
    ///
    /// Constraints:
    /// - pattern: `^[0-9]{4}$`
    ///
    /// Example: `1234`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`passcode`](UpdateRequestMenuLock::passcode)."
        )
    )]
    pub passcode: Option<crate::secret::Secret>,
}
/// Menu lock settings to update.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct UpdateRequest {
    /// Menu lock configuration. Provide at least one of `enabled` or `passcode`.
    ///
    /// Constraints:
    /// - min properties: 1
    #[builder(setter(doc = "Sets [`menu_lock`](UpdateRequest::menu_lock)."))]
    pub menu_lock: UpdateRequestMenuLock,
}
use crate::client::Client;
#[derive(Debug, PartialEq)]
pub enum GetErrorBody {
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for GetErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum UpdateErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for UpdateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
/// Client for the ReaderSettings API endpoints.
#[derive(Debug)]
pub struct ReaderSettingsClient<'a> {
    client: &'a Client,
}
impl<'a> ReaderSettingsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Get Reader Settings
    ///
    /// Returns the menu lock settings for a Solo reader.
    ///
    /// For an online reader, returns the current settings. For an offline reader,
    /// returns the last saved settings, which may differ from its current configuration.
    /// Returns `404` if the reader is offline and no saved settings are available.
    ///
    /// The response may include the menu lock PIN.
    ///
    /// Responses:
    /// - 200: Reader settings retrieved successfully.
    /// - 401: Unauthorized
    /// - 404: The reader was not found, or it is offline and has no saved settings.
    /// - 422: The reader model is not supported, the firmware version is unsupported, or the
    ///   reader could not return its settings.
    pub async fn get(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<GetReaderSettingsResponse, GetErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/settings",
            merchant_code.as_ref(),
            reader_id.as_ref()
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
                let data: GetReaderSettingsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetErrorBody::UnprocessableEntity)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Update Reader Settings
    ///
    /// Updates the menu lock settings for a Solo reader. The reader must be online.
    /// A successful response confirms that the reader applied the update.
    ///
    /// Include at least one of `menu_lock.enabled` or `menu_lock.passcode`.
    /// Omitted settings keep their current values. The response contains only the
    /// settings included in the request.
    ///
    /// When enabling menu lock for a reader without a PIN, include a four-digit PIN
    /// in `menu_lock.passcode` in the same request.
    ///
    /// Responses:
    /// - 200: Reader settings updated successfully.
    /// - 400: The request body is malformed.
    /// - 401: Unauthorized
    /// - 404: The reader was not found.
    /// - 422: The settings are invalid, the reader is offline, the reader model is not
    ///   supported, the firmware version is unsupported, or the reader could not apply the
    ///   update.
    pub async fn update(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        body: UpdateRequest,
    ) -> crate::error::SdkResult<UpdateReaderSettingsResponse, UpdateErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/settings",
            merchant_code.as_ref(),
            reader_id.as_ref()
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
                let data: UpdateReaderSettingsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::UnprocessableEntity)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
