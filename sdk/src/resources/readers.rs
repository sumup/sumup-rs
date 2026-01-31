// The contents of this file are generated; do not modify them.

use super::common::*;
/// 502 Bad Gateway
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BadGateway {
    pub errors: BadGatewayErrors,
}
impl std::fmt::Display for BadGateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for BadGateway {}
/// 400 Bad Request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BadRequest {
    pub errors: BadRequestErrors,
}
impl std::fmt::Display for BadRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for BadRequest {}
/// Error description
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderCheckoutError {
    pub errors: CreateReaderCheckoutErrorErrors,
}
impl std::fmt::Display for CreateReaderCheckoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for CreateReaderCheckoutError {}
/// Reader Checkout
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderCheckoutRequest {
    /// Affiliate metadata for the transaction.
    /// It is a field that allow for integrators to track the source of the transaction.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub affiliate: Option<crate::Nullable<Affiliate>>,
    /// The card type of the card used for the transaction.
    /// Is is required only for some countries (e.g: Brazil).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// Description of the checkout to be shown in the Merchant Sales
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Number of installments for the transaction.
    /// It may vary according to the merchant country.
    /// For example, in Brazil, the maximum number of installments is 12.
    ///
    /// Omit if the merchant country does support installments.
    /// Otherwise, the checkout will be rejected.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub installments: Option<crate::Nullable<i64>>,
    /// Webhook URL to which the payment result will be sent.
    /// It must be a HTTPS url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// List of tipping rates to be displayed to the cardholder.
    /// The rates are in percentage and should be between 0.01 and 0.99.
    /// The list should be sorted in ascending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_rates: Option<Vec<f32>>,
    /// Time in seconds the cardholder has to select a tip rate.
    /// If not provided, the default value is 30 seconds.
    ///
    /// It can only be set if `tip_rates` is provided.
    ///
    /// **Note**: If the target device is a Solo, it must be in version 3.3.38.0 or higher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_timeout: Option<i64>,
    /// Amount structure.
    ///
    /// The amount is represented as an integer value altogether with the currency and the minor unit.
    ///
    /// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
    pub total_amount: Money,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderCheckoutResponse {
    pub data: CreateReaderCheckoutResponseData,
}
/// Unprocessable entity
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderCheckoutUnprocessableEntity {
    pub errors: CreateReaderCheckoutUnprocessableEntityErrors,
}
impl std::fmt::Display for CreateReaderCheckoutUnprocessableEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for CreateReaderCheckoutUnprocessableEntity {}
/// Error description
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderTerminateError {
    pub errors: CreateReaderTerminateErrorErrors,
}
impl std::fmt::Display for CreateReaderTerminateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for CreateReaderTerminateError {}
/// Unprocessable entity
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderTerminateUnprocessableEntity {
    pub errors: CreateReaderTerminateUnprocessableEntityErrors,
}
impl std::fmt::Display for CreateReaderTerminateUnprocessableEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for CreateReaderTerminateUnprocessableEntity {}
/// 504 Gateway Timeout
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GatewayTimeout {
    pub errors: GatewayTimeoutErrors,
}
impl std::fmt::Display for GatewayTimeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for GatewayTimeout {}
/// 500 Internal Server Error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InternalServerError {
    pub errors: InternalServerErrorErrors,
}
impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for InternalServerError {}
/// 404 Not Found
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotFound {
    pub errors: NotFoundErrors,
}
impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for NotFound {}
/// A physical card reader device that can accept in-person payments.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Reader {
    pub id: ReaderId,
    pub name: ReaderName,
    pub status: ReaderStatus,
    pub device: ReaderDevice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The timestamp of when the reader was created.
    pub created_at: crate::datetime::DateTime,
    /// The timestamp of when the reader was last updated.
    pub updated_at: crate::datetime::DateTime,
}
/// Information about the underlying physical device.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReaderDevice {
    /// A unique identifier of the physical device (e.g. serial number).
    pub identifier: String,
    /// Identifier of the model of the device.
    pub model: String,
}
pub type ReaderId = String;
pub type ReaderName = String;
pub type ReaderPairingCode = String;
/// The status of the reader object gives information about the current state of the reader.
///
/// Possible values:
///
/// - `unknown` - The reader status is unknown.
/// - `processing` - The reader is created and waits for the physical device to confirm the pairing.
/// - `paired` - The reader is paired with a merchant account and can be used with SumUp APIs.
/// - `expired` - The pairing is expired and no longer usable with the account. The resource needs to get recreated.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReaderStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "paired")]
    Paired,
    #[serde(rename = "expired")]
    Expired,
}
/// Status of a device
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusResponse {
    pub data: StatusResponseData,
}
/// 401 Unauthorized
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Unauthorized {
    pub errors: UnauthorizedErrors,
}
impl std::fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Unauthorized {}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BadGatewayErrors {
    /// Fuller message giving context to error
    pub detail: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BadRequestErrors {
    /// Fuller message giving context to error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Key indicating type of error
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderCheckoutErrorErrors {
    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
/// Additional metadata for the transaction.
/// It is key-value object that can be associated with the transaction.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AffiliateTags {}
/// Affiliate metadata for the transaction.
/// It is a field that allow for integrators to track the source of the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Affiliate {
    /// Application ID of the affiliate.
    /// It is a unique identifier for the application and should be set by the integrator in the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    pub app_id: String,
    /// Foreign transaction ID of the affiliate.
    /// It is a unique identifier for the transaction.
    /// It can be used later to fetch the transaction details via the [Transactions API](https://developer.sumup.com/api/transactions/get).
    pub foreign_transaction_id: String,
    /// Key of the affiliate.
    /// It is a unique identifier for the key  and should be generated by the integrator in the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    pub key: String,
    /// Additional metadata for the transaction.
    /// It is key-value object that can be associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<AffiliateTags>,
}
/// Amount structure.
///
/// The amount is represented as an integer value altogether with the currency and the minor unit.
///
/// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Money {
    /// Currency ISO 4217 code
    pub currency: String,
    /// The minor units of the currency.
    /// It represents the number of decimals of the currency. For the currencies CLP, COP and HUF, the minor unit is 0.
    pub minor_unit: i64,
    /// Integer value of the amount.
    pub value: i64,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderCheckoutResponseData {
    /// The client transaction ID is a unique identifier for the transaction that is generated for the client.
    ///
    /// It can be used later to fetch the transaction details via the [Transactions API](https://developer.sumup.com/api/transactions/get).
    pub client_transaction_id: String,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderCheckoutUnprocessableEntityErrors {}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderTerminateErrorErrors {
    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreateReaderTerminateUnprocessableEntityErrors {}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GatewayTimeoutErrors {
    /// Fuller message giving context to error
    pub detail: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InternalServerErrorErrors {
    /// Fuller message giving context to error
    pub detail: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotFoundErrors {
    /// Fuller message giving context to error
    pub detail: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusResponseData {
    /// Battery level percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_level: Option<f32>,
    /// Battery temperature in Celsius
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_temperature: Option<i64>,
    /// Type of connection used by the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// Firmware version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    /// Timestamp of the last activity from the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<crate::datetime::DateTime>,
    /// Latest state of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Status of a device
    pub status: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnauthorizedErrors {
    /// Fuller message giving context to error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Key indicating type of error
    #[serde(rename = "type")]
    pub type_: String,
}
/// Returns a list Reader objects.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    pub items: Vec<Reader>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateBody {
    pub pairing_code: ReaderPairingCode,
    pub name: ReaderName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ReaderName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum CreateErrorBody {
    BadRequest(Problem),
    NotFound(Problem),
    Conflict(Problem),
}
#[derive(Debug)]
pub enum DeleteErrorBody {
    NotFound(Problem),
}
#[derive(Debug)]
pub enum GetErrorBody {
    NotFound(Problem),
}
#[derive(Debug)]
pub enum UpdateErrorBody {
    Forbidden(Problem),
    NotFound(Problem),
}
#[derive(Debug)]
pub enum CreateCheckoutErrorBody {
    BadRequest(CreateReaderCheckoutError),
    Unauthorized(CreateReaderCheckoutError),
    UnprocessableEntity(CreateReaderCheckoutUnprocessableEntity),
    InternalServerError(CreateReaderCheckoutError),
    BadGateway(CreateReaderCheckoutError),
    GatewayTimeout(CreateReaderCheckoutError),
}
#[derive(Debug)]
pub enum GetStatusErrorBody {
    BadRequest(BadRequest),
    Unauthorized(Unauthorized),
    NotFound(NotFound),
    InternalServerError(InternalServerError),
    BadGateway(BadGateway),
    GatewayTimeout(GatewayTimeout),
}
#[derive(Debug)]
pub enum TerminateCheckoutErrorBody {
    BadRequest(CreateReaderTerminateError),
    Unauthorized(CreateReaderTerminateError),
    UnprocessableEntity(CreateReaderTerminateUnprocessableEntity),
    InternalServerError(CreateReaderTerminateError),
    BadGateway(CreateReaderTerminateError),
    GatewayTimeout(CreateReaderTerminateError),
}
///Client for the Readers API endpoints.
#[derive(Debug)]
pub struct ReadersClient<'a> {
    client: &'a Client,
}
impl<'a> ReadersClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// List Readers
    ///
    /// List all readers of the merchant.
    pub async fn list(
        &self,
        merchant_code: impl Into<String>,
    ) -> crate::error::SdkResult<ListResponse, crate::error::UnknownApiBody> {
        let path = format!("/v0.1/merchants/{}/readers", merchant_code.into());
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
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Create a Reader
    ///
    /// Create a new Reader for the merchant account.
    pub async fn create(
        &self,
        merchant_code: impl Into<String>,
        body: CreateBody,
    ) -> crate::error::SdkResult<Reader, CreateErrorBody> {
        let path = format!("/v0.1/merchants/{}/readers", merchant_code.into());
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
                let data: Reader = response.json().await?;
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
            reqwest::StatusCode::CONFLICT => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(CreateErrorBody::Conflict(body)))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Delete a reader
    ///
    /// Delete a reader.
    pub async fn delete(
        &self,
        merchant_code: impl Into<String>,
        id: impl Into<String>,
    ) -> crate::error::SdkResult<(), DeleteErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}",
            merchant_code.into(),
            id.into()
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
    /// Retrieve a Reader
    ///
    /// Retrieve a Reader.
    pub async fn get(
        &self,
        merchant_code: impl Into<String>,
        id: impl Into<String>,
    ) -> crate::error::SdkResult<Reader, GetErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}",
            merchant_code.into(),
            id.into()
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
                let data: Reader = response.json().await?;
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
    /// Update a Reader
    ///
    /// Update a Reader.
    pub async fn update(
        &self,
        merchant_code: impl Into<String>,
        id: impl Into<String>,
        body: UpdateBody,
    ) -> crate::error::SdkResult<Reader, UpdateErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}",
            merchant_code.into(),
            id.into()
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
                let data: Reader = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::FORBIDDEN => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(UpdateErrorBody::Forbidden(
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
    /// Create a Reader Checkout
    ///
    /// Creates a Checkout for a Reader.
    ///
    /// This process is asynchronous and the actual transaction may take some time to be stared on the device.
    ///
    ///
    /// There are some caveats when using this endpoint:
    /// * The target device must be online, otherwise checkout won't be accepted
    /// * After the checkout is accepted, the system has 60 seconds to start the payment on the target device. During this time, any other checkout for the same device will be rejected.
    ///
    ///
    /// **Note**: If the target device is a Solo, it must be in version 3.3.24.3 or higher.
    pub async fn create_checkout(
        &self,
        merchant_code: impl Into<String>,
        reader_id: impl Into<String>,
        body: CreateReaderCheckoutRequest,
    ) -> crate::error::SdkResult<CreateReaderCheckoutResponse, CreateCheckoutErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/checkout",
            merchant_code.into(),
            reader_id.into()
        );
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
                let data: CreateReaderCheckoutResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: CreateReaderCheckoutError = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: CreateReaderCheckoutError = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                let body: CreateReaderCheckoutUnprocessableEntity = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::UnprocessableEntity(body),
                ))
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                let body: CreateReaderCheckoutError = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::InternalServerError(body),
                ))
            }
            reqwest::StatusCode::BAD_GATEWAY => {
                let body: CreateReaderCheckoutError = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::BadGateway(body),
                ))
            }
            reqwest::StatusCode::GATEWAY_TIMEOUT => {
                let body: CreateReaderCheckoutError = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::GatewayTimeout(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Get a Reader Status
    ///
    /// Provides the last known status for a Reader.
    ///
    /// This endpoint allows you to retrieve updates from the connected card reader, including the current screen being displayed during the payment process and the device status (battery level, connectivity, and update state).
    ///
    /// Supported States
    ///
    /// * `IDLE` – Reader ready for next transaction
    /// * `SELECTING_TIP` – Waiting for tip input
    /// * `WAITING_FOR_CARD` – Awaiting card insert/tap
    /// * `WAITING_FOR_PIN` – Waiting for PIN entry
    /// * `WAITING_FOR_SIGNATURE` – Waiting for customer signature
    /// * `UPDATING_FIRMWARE` – Firmware update in progress
    ///
    /// Device Status
    ///
    /// * `ONLINE` – Device connected and operational
    /// * `OFFLINE` – Device disconnected (last state persisted)
    ///
    /// **Note**: If the target device is a Solo, it must be in version 3.3.39.0 or higher.
    pub async fn get_status(
        &self,
        merchant_code: impl Into<String>,
        reader_id: impl Into<String>,
    ) -> crate::error::SdkResult<StatusResponse, GetStatusErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/status",
            merchant_code.into(),
            reader_id.into()
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
                let data: StatusResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: BadRequest = response.json().await?;
                Err(crate::error::SdkError::api(GetStatusErrorBody::BadRequest(
                    body,
                )))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Unauthorized = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetStatusErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: NotFound = response.json().await?;
                Err(crate::error::SdkError::api(GetStatusErrorBody::NotFound(
                    body,
                )))
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                let body: InternalServerError = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetStatusErrorBody::InternalServerError(body),
                ))
            }
            reqwest::StatusCode::BAD_GATEWAY => {
                let body: BadGateway = response.json().await?;
                Err(crate::error::SdkError::api(GetStatusErrorBody::BadGateway(
                    body,
                )))
            }
            reqwest::StatusCode::GATEWAY_TIMEOUT => {
                let body: GatewayTimeout = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetStatusErrorBody::GatewayTimeout(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Terminate a Reader Checkout
    ///
    /// Terminate a Reader Checkout stops the current transaction on the target device.
    ///
    /// This process is asynchronous and the actual termination may take some time to be performed on the device.
    ///
    ///
    /// There are some caveats when using this endpoint:
    /// * The target device must be online, otherwise terminate won't be accepted
    /// * The action will succeed only if the device is waiting for cardholder action: e.g: waiting for card, waiting for PIN, etc.
    /// * There is no confirmation of the termination.
    ///
    /// If a transaction is successfully terminated and `return_url` was provided on Checkout, the transaction status will be sent as `failed` to the provided URL.
    ///
    ///
    /// **Note**: If the target device is a Solo, it must be in version 3.3.28.0 or higher.
    pub async fn terminate_checkout(
        &self,
        merchant_code: impl Into<String>,
        reader_id: impl Into<String>,
    ) -> crate::error::SdkResult<(), TerminateCheckoutErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/terminate",
            merchant_code.into(),
            reader_id.into()
        );
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .post(&url)
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
            reqwest::StatusCode::ACCEPTED => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                let body: CreateReaderTerminateError = response.json().await?;
                Err(crate::error::SdkError::api(
                    TerminateCheckoutErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: CreateReaderTerminateError = response.json().await?;
                Err(crate::error::SdkError::api(
                    TerminateCheckoutErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                let body: CreateReaderTerminateUnprocessableEntity = response.json().await?;
                Err(crate::error::SdkError::api(
                    TerminateCheckoutErrorBody::UnprocessableEntity(body),
                ))
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                let body: CreateReaderTerminateError = response.json().await?;
                Err(crate::error::SdkError::api(
                    TerminateCheckoutErrorBody::InternalServerError(body),
                ))
            }
            reqwest::StatusCode::BAD_GATEWAY => {
                let body: CreateReaderTerminateError = response.json().await?;
                Err(crate::error::SdkError::api(
                    TerminateCheckoutErrorBody::BadGateway(body),
                ))
            }
            reqwest::StatusCode::GATEWAY_TIMEOUT => {
                let body: CreateReaderTerminateError = response.json().await?;
                Err(crate::error::SdkError::api(
                    TerminateCheckoutErrorBody::GatewayTimeout(body),
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
