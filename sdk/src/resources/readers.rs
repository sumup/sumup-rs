// The contents of this file are generated; do not modify them.

//! A reader represents a device that accepts payments. You can use the SumUp Solo to accept in-person payments.
use super::common::*;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum ReaderCreated {}
impl crate::event::private::Sealed for ReaderCreated {}
impl crate::events::EventSpec for ReaderCreated {
    const EVENT_TYPE: &'static str = "readers.created";
    type FetchedObject = Reader;
}
/// Event notification for this event type.
pub type ReaderCreatedEvent = crate::events::Event<ReaderCreated>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum ReaderDeleted {}
impl crate::event::private::Sealed for ReaderDeleted {}
impl crate::events::EventSpec for ReaderDeleted {
    const EVENT_TYPE: &'static str = "readers.deleted";
    type FetchedObject = Reader;
}
/// Event notification for this event type.
pub type ReaderDeletedEvent = crate::events::Event<ReaderDeleted>;
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Affiliate {
    #[builder(setter(into, doc = "Sets [`app_id`](Affiliate::app_id)."))]
    pub app_id: String,
    #[builder(setter(into, doc = "Sets [`key`](Affiliate::key)."))]
    pub key: String,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Amount {
    /// Currency ISO 4217 code
    ///
    /// Example: `MXN`
    #[builder(setter(into, doc = "Sets [`currency`](Amount::currency)."))]
    pub currency: String,
    /// Amount in minor units (e.g. cents).
    ///
    /// Example: `1000`
    #[builder(setter(doc = "Sets [`value`](Amount::value)."))]
    pub value: i64,
}
/// Payment details for a Reader Checkout.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateReaderCheckoutRequest {
    /// Optional object containing data for transactions from ERP integrators in Greece that comply with the AADE 1155 protocol.
    /// When such regulatory/business requirements apply, this object must be provided and contains the data needed to validate the transaction with the AADE signature provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`aade`](CreateReaderCheckoutRequest::aade)."
        )
    )]
    pub aade: Option<CreateReaderCheckoutRequestAade>,
    /// Optional affiliate metadata associated with the checkout's payment.
    /// Provide both `app_id` and `key` when including this object.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`affiliate`](CreateReaderCheckoutRequest::affiliate)."
        )
    )]
    pub affiliate: Option<crate::Nullable<CreateReaderCheckoutRequestAffiliate>>,
    /// The card type of the card used for the transaction.
    /// It is required only for some countries (e.g: Brazil).
    ///
    /// Example: `credit`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`card_type`](CreateReaderCheckoutRequest::card_type)."
        )
    )]
    pub card_type: Option<CreateReaderCheckoutRequestCardType>,
    /// Description of the checkout to be shown in the Merchant Sales
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`description`](CreateReaderCheckoutRequest::description)."
        )
    )]
    pub description: Option<String>,
    /// Number of installments for the transaction.
    /// It may vary according to the merchant country.
    /// For example, in Brazil, the maximum number of installments is 12.
    ///
    /// Omit if the merchant country does not support installments.
    /// Otherwise, the checkout will be rejected.
    ///
    /// Constraints:
    /// - value >= 1
    ///
    /// Example: `1`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`installments`](CreateReaderCheckoutRequest::installments)."
        )
    )]
    pub installments: Option<crate::Nullable<i64>>,
    /// The payment method type for the transaction. Defaults to `card`.
    ///
    /// Use `pix` to initiate a Pix QR-code payment flow on the device.
    /// Pix is only supported for Brazilian merchants; other merchants will receive a 422 error.
    ///
    /// Example: `card`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`payment_type`](CreateReaderCheckoutRequest::payment_type)."
        )
    )]
    pub payment_type: Option<CreateReaderCheckoutRequestPaymentType>,
    /// Webhook URL to which the payment result will be sent.
    /// It must be a HTTPS url.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `https://www.example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`return_url`](CreateReaderCheckoutRequest::return_url)."
        )
    )]
    pub return_url: Option<String>,
    /// Exactly three distinct tipping rates to be displayed to the cardholder.
    /// Each rate is a fraction between 0.01 and 1.00 in increments of 0.01.
    /// The rates are sorted in ascending order by the server.
    ///
    /// Constraints:
    /// - min items: 3
    /// - max items: 3
    /// - items must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tip_rates`](CreateReaderCheckoutRequest::tip_rates)."
        )
    )]
    pub tip_rates: Option<Vec<f32>>,
    /// Time in seconds the cardholder has to select a tip rate.
    /// If not provided, the default value is 30 seconds.
    ///
    /// It can only be set if `tip_rates` is provided.
    ///
    /// **Note**: If the target device is a Solo, it must be in version 3.3.38.0 or higher.
    ///
    /// Constraints:
    /// - value >= 30
    /// - value <= 120
    ///
    /// Example: `30`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tip_timeout`](CreateReaderCheckoutRequest::tip_timeout)."
        )
    )]
    pub tip_timeout: Option<i64>,
    /// Amount structure.
    ///
    /// The amount is represented as an integer value altogether with the currency and the minor unit.
    ///
    /// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
    #[builder(setter(doc = "Sets [`total_amount`](CreateReaderCheckoutRequest::total_amount)."))]
    pub total_amount: CreateReaderCheckoutRequestTotalAmount,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateReaderCheckoutResponse {
    #[builder(setter(doc = "Sets [`data`](CreateReaderCheckoutResponse::data)."))]
    pub data: CreateReaderCheckoutResponseData,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct GetReaderCheckoutResponse {
    #[builder(setter(doc = "Sets [`data`](GetReaderCheckoutResponse::data)."))]
    pub data: GetReaderCheckoutResponseData,
}
/// Menu lock settings reported by the reader, or the last saved settings when offline.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct GetReaderSettingsResponse {
    #[builder(setter(doc = "Sets [`data`](GetReaderSettingsResponse::data)."))]
    pub data: GetReaderSettingsResponseData,
}
/// A physical card reader device that can accept in-person payments.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Reader {
    /// The timestamp of when the reader was created.
    ///
    /// Example: `2023-01-18T15:16:17Z`
    #[builder(setter(doc = "Sets [`created_at`](Reader::created_at)."))]
    pub created_at: crate::datetime::DateTime,
    #[builder(setter(doc = "Sets [`device`](Reader::device)."))]
    pub device: ReaderDevice,
    #[builder(setter(into, doc = "Sets [`id`](Reader::id)."))]
    pub id: ReaderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`metadata`](Reader::metadata).")
    )]
    pub metadata: Option<Metadata>,
    #[builder(setter(into, doc = "Sets [`name`](Reader::name)."))]
    pub name: ReaderName,
    /// Identifier of the system-managed service account associated with this reader.
    /// Present only for readers that are already paired.
    /// This field is currently in beta and may change.
    ///
    /// Constraints:
    /// - format: `uuid`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`service_account_id`](Reader::service_account_id)."
        )
    )]
    pub service_account_id: Option<String>,
    #[builder(setter(doc = "Sets [`status`](Reader::status)."))]
    pub status: ReaderStatus,
    /// The timestamp of when the reader was last updated.
    ///
    /// Example: `2023-01-20T15:16:17Z`
    #[builder(setter(doc = "Sets [`updated_at`](Reader::updated_at)."))]
    pub updated_at: crate::datetime::DateTime,
}
/// Information about the underlying physical device.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct ReaderDevice {
    /// A unique identifier of the physical device (e.g. serial number).
    ///
    /// Example: `U1DT3NA00-CN`
    #[builder(setter(into, doc = "Sets [`identifier`](ReaderDevice::identifier)."))]
    pub identifier: String,
    /// Identifier of the model of the device.
    ///
    /// Example: `solo`
    #[builder(setter(doc = "Sets [`model`](ReaderDevice::model)."))]
    pub model: ReaderDeviceModel,
}
/// Unique identifier of the reader that the payment is initiated on.
///
/// Constraints:
/// - min length: 30
/// - max length: 30
///
/// Example: `rdr_3MSAFM23CK82VSTT4BN6RWSQ65`
pub type ReaderId = String;
/// Custom human-readable, user-defined name for easier identification of the reader.
///
/// Constraints:
/// - max length: 500
///
/// Example: `Frontdesk`
pub type ReaderName = String;
/// The pairing code is a 8 or 9 character alphanumeric string that is displayed on a SumUp Device after initiating the pairing. It is used to link the physical device to the created pairing.
///
/// Constraints:
/// - min length: 8
/// - max length: 9
///
/// Example: `4WLFDSBF`
pub type ReaderPairingCode = String;
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct ReaderPaymentRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`affiliate`](ReaderPaymentRequestParams::affiliate)."
        )
    )]
    pub affiliate: Option<ReaderPaymentRequestParamsAffiliate>,
    /// Caller-supplied correlation identifier, used as the idempotency key.
    ///
    /// Example: `19e12390-72cf-4f9f-80b5-b0c8a67fa43f`
    #[builder(setter(
        into,
        doc = "Sets [`client_transaction_id`](ReaderPaymentRequestParams::client_transaction_id)."
    ))]
    pub client_transaction_id: String,
    /// Portion of total_amount that is a tip, in minor units.
    ///
    /// Example: `200`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tip_amount`](ReaderPaymentRequestParams::tip_amount)."
        )
    )]
    pub tip_amount: Option<i64>,
    #[builder(setter(doc = "Sets [`total_amount`](ReaderPaymentRequestParams::total_amount)."))]
    pub total_amount: Amount,
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
pub struct ReaderPaymentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`data`](ReaderPaymentResponse::data).")
    )]
    pub data: Option<ReaderPaymentResponseData>,
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
pub struct ReaderPaymentResponseData {
    /// Caller-supplied correlation identifier that was provided in the request.
    ///
    /// Example: `3fa85f64-5717-4562-b3fc-2c963f66afa6`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`client_transaction_id`](ReaderPaymentResponseData::client_transaction_id)."
        )
    )]
    pub client_transaction_id: Option<String>,
    /// Transaction code returned by the acquirer/processing entity after processing the transaction.
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`transaction_code`](ReaderPaymentResponseData::transaction_code)."
        )
    )]
    pub transaction_code: Option<String>,
}
/// The status of the reader object gives information about the current state of the reader.
///
/// Possible values:
///
/// - `unknown` - The reader status is unknown.
/// - `processing` - The reader is created and waits for the physical device to confirm the pairing.
/// - `paired` - The reader is paired with a merchant account and can be used with SumUp APIs.
/// - `expired` - The pairing is expired and no longer usable with the account. The resource needs to get recreated.
///
/// Example: `paired`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReaderStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "paired")]
    Paired,
    #[serde(rename = "expired")]
    Expired,
    #[serde(untagged)]
    Other(String),
}
/// Status of a device
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct StatusResponse {
    #[builder(setter(doc = "Sets [`data`](StatusResponse::data)."))]
    pub data: StatusResponseData,
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
/// Optional object containing data for transactions from ERP integrators in Greece that comply with the AADE 1155 protocol.
/// When such regulatory/business requirements apply, this object must be provided and contains the data needed to validate the transaction with the AADE signature provider.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateReaderCheckoutRequestAade {
    /// The identifier of the AADE signature provider.
    ///
    /// Example: `123`
    #[builder(setter(
        into,
        doc = "Sets [`provider_id`](CreateReaderCheckoutRequestAade::provider_id)."
    ))]
    pub provider_id: String,
    /// The base64 encoded signature of the transaction data.
    ///
    /// Example: `QjcxRDdBNTU1MDcyRTNFRTREMkZEM0Y0NTdBMjkxMTU4MzBFNkNCQTs7MjAyNTExMTIyMTQ3MTM7Nzk2OzEwNDs5MDA7OTAwOzU0ODg5MDM5`
    #[builder(setter(
        into,
        doc = "Sets [`signature`](CreateReaderCheckoutRequestAade::signature)."
    ))]
    pub signature: String,
    /// The string containing the signed transaction data.
    ///
    /// Example: `B71D7A555072E3EE4D2FD3F457A29115830E6CBA;;20251112214713;796;104;900;900;54889039`
    #[builder(setter(
        into,
        doc = "Sets [`signature_data`](CreateReaderCheckoutRequestAade::signature_data)."
    ))]
    pub signature_data: String,
}
/// Optional key-value metadata associated with the transaction.
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
pub struct CreateReaderCheckoutRequestAffiliateTags {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    #[builder(
        default,
        setter(
            doc = "Sets [`additional_properties`](CreateReaderCheckoutRequestAffiliateTags::additional_properties)."
        )
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
/// Optional affiliate metadata associated with the checkout's payment.
/// Provide both `app_id` and `key` when including this object.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateReaderCheckoutRequestAffiliate {
    /// Integrator application identifier from the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    /// Required when the affiliate object is provided.
    ///
    /// Example: `com.example.app`
    #[builder(setter(
        into,
        doc = "Sets [`app_id`](CreateReaderCheckoutRequestAffiliate::app_id)."
    ))]
    pub app_id: String,
    /// Integrator-owned transaction reference. Optional.
    /// Can be used to retrieve the transaction via the [Transactions API](https://developer.sumup.com/api/transactions/get).
    ///
    /// Example: `19e12390-72cf-4f9f-80b5-b0c8a67fa43f`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`foreign_transaction_id`](CreateReaderCheckoutRequestAffiliate::foreign_transaction_id)."
        )
    )]
    pub foreign_transaction_id: Option<String>,
    /// Affiliate key generated in the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    /// Required when the affiliate object is provided.
    ///
    /// Example: `123e4567-e89b-12d3-a456-426614174000`
    #[builder(setter(into, doc = "Sets [`key`](CreateReaderCheckoutRequestAffiliate::key)."))]
    pub key: String,
    /// Optional key-value metadata associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tags`](CreateReaderCheckoutRequestAffiliate::tags)."
        )
    )]
    pub tags: Option<serde_json::Value>,
}
/// The card type of the card used for the transaction.
/// It is required only for some countries (e.g: Brazil).
///
/// Example: `credit`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateReaderCheckoutRequestCardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(untagged)]
    Other(String),
}
/// The payment method type for the transaction. Defaults to `card`.
///
/// Use `pix` to initiate a Pix QR-code payment flow on the device.
/// Pix is only supported for Brazilian merchants; other merchants will receive a 422 error.
///
/// Example: `card`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateReaderCheckoutRequestPaymentType {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "pix")]
    Pix,
    #[serde(untagged)]
    Other(String),
}
/// Amount structure.
///
/// The amount is represented as an integer value altogether with the currency and the minor unit.
///
/// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateReaderCheckoutRequestTotalAmount {
    /// Currency ISO 4217 code
    ///
    /// Example: `EUR`
    #[builder(setter(
        into,
        doc = "Sets [`currency`](CreateReaderCheckoutRequestTotalAmount::currency)."
    ))]
    pub currency: String,
    /// The minor units of the currency.
    /// It represents the number of decimals of the currency. For the currencies CLP, COP and HUF, the minor unit is 0.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `2`
    #[builder(setter(
        doc = "Sets [`minor_unit`](CreateReaderCheckoutRequestTotalAmount::minor_unit)."
    ))]
    pub minor_unit: i64,
    /// Integer value of the amount.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `1000`
    #[builder(setter(doc = "Sets [`value`](CreateReaderCheckoutRequestTotalAmount::value)."))]
    pub value: i64,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateReaderCheckoutResponseData {
    /// The checkout ID is a unique identifier for the checkout.
    ///
    /// Example: `3fa85f64-5717-4562-b3fc-2c963f66afa6`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`checkout_id`](CreateReaderCheckoutResponseData::checkout_id)."
        )
    )]
    pub checkout_id: Option<String>,
    /// Unique identifier for the transaction associated with this checkout.
    /// Use this value to match the checkout with payment events and transaction records.
    ///
    /// Example: `3fa85f64-5717-4562-b3fc-2c963f66afa6`
    #[builder(setter(
        into,
        doc = "Sets [`client_transaction_id`](CreateReaderCheckoutResponseData::client_transaction_id)."
    ))]
    pub client_transaction_id: String,
}
/// Type of the card, or null when no card type was specified.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetReaderCheckoutResponseDataCardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(untagged)]
    Other(String),
}
/// Type of the payment. Required for some countries
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetReaderCheckoutResponseDataPaymentType {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "pix")]
    Pix,
    #[serde(untagged)]
    Other(String),
}
/// Current status of the checkout
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GetReaderCheckoutResponseDataStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(untagged)]
    Other(String),
}
/// Amount structure.
///
/// The amount is represented as an integer value altogether with the currency and the minor unit.
///
/// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct GetReaderCheckoutResponseDataTotalAmount {
    /// Currency ISO 4217 code
    ///
    /// Example: `EUR`
    #[builder(setter(
        into,
        doc = "Sets [`currency`](GetReaderCheckoutResponseDataTotalAmount::currency)."
    ))]
    pub currency: String,
    /// The minor units of the currency.
    /// It represents the number of decimals of the currency. For the currencies CLP, COP and HUF, the minor unit is 0.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `2`
    #[builder(setter(
        doc = "Sets [`minor_unit`](GetReaderCheckoutResponseDataTotalAmount::minor_unit)."
    ))]
    pub minor_unit: i64,
    /// Integer value of the amount.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `1000`
    #[builder(setter(doc = "Sets [`value`](GetReaderCheckoutResponseDataTotalAmount::value)."))]
    pub value: i64,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct GetReaderCheckoutResponseData {
    /// Type of the card, or null when no card type was specified.
    #[builder(setter(doc = "Sets [`card_type`](GetReaderCheckoutResponseData::card_type)."))]
    pub card_type: GetReaderCheckoutResponseDataCardType,
    /// Unique identifier for the checkout
    ///
    /// Constraints:
    /// - format: `uuid`
    #[builder(setter(
        into,
        doc = "Sets [`checkout_id`](GetReaderCheckoutResponseData::checkout_id)."
    ))]
    pub checkout_id: String,
    /// Client transaction identifier associated with the checkout
    #[builder(setter(
        into,
        doc = "Sets [`client_transaction_id`](GetReaderCheckoutResponseData::client_transaction_id)."
    ))]
    pub client_transaction_id: String,
    /// Checkout creation timestamp
    #[builder(setter(doc = "Sets [`created_at`](GetReaderCheckoutResponseData::created_at)."))]
    pub created_at: crate::datetime::DateTime,
    /// Number of installments for the transaction. Required for some countries.
    #[builder(setter(
        doc = "Sets [`installments`](GetReaderCheckoutResponseData::installments)."
    ))]
    pub installments: i64,
    /// Reason for the payment failure, or null when no failure reason is available.
    #[builder(setter(
        into,
        doc = "Sets [`payment_failure_reason`](GetReaderCheckoutResponseData::payment_failure_reason)."
    ))]
    pub payment_failure_reason: String,
    /// Status of the payment, or null when no payment status is available.
    #[builder(setter(
        into,
        doc = "Sets [`payment_status`](GetReaderCheckoutResponseData::payment_status)."
    ))]
    pub payment_status: String,
    /// Type of the payment. Required for some countries
    #[builder(setter(
        doc = "Sets [`payment_type`](GetReaderCheckoutResponseData::payment_type)."
    ))]
    pub payment_type: GetReaderCheckoutResponseDataPaymentType,
    /// Reader firmware version
    #[builder(setter(
        into,
        doc = "Sets [`reader_firmware_version`](GetReaderCheckoutResponseData::reader_firmware_version)."
    ))]
    pub reader_firmware_version: String,
    /// Device serial number
    #[builder(setter(
        into,
        doc = "Sets [`reader_serial_number`](GetReaderCheckoutResponseData::reader_serial_number)."
    ))]
    pub reader_serial_number: String,
    /// Current status of the checkout
    #[builder(setter(doc = "Sets [`status`](GetReaderCheckoutResponseData::status)."))]
    pub status: GetReaderCheckoutResponseDataStatus,
    /// Amount structure.
    ///
    /// The amount is represented as an integer value altogether with the currency and the minor unit.
    ///
    /// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
    #[builder(setter(
        doc = "Sets [`total_amount`](GetReaderCheckoutResponseData::total_amount)."
    ))]
    pub total_amount: GetReaderCheckoutResponseDataTotalAmount,
    /// Checkout last update timestamp
    #[builder(setter(doc = "Sets [`updated_at`](GetReaderCheckoutResponseData::updated_at)."))]
    pub updated_at: crate::datetime::DateTime,
    /// Checkout expiration timestamp. After this time, the checkout will be automatically cancelled.
    #[builder(setter(doc = "Sets [`valid_until`](GetReaderCheckoutResponseData::valid_until)."))]
    pub valid_until: crate::datetime::DateTime,
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
/// Identifier of the model of the device.
///
/// Example: `solo`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReaderDeviceModel {
    #[serde(rename = "solo")]
    Solo,
    #[serde(rename = "virtual-solo")]
    VirtualSolo,
    #[serde(untagged)]
    Other(String),
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct ReaderPaymentRequestParamsAffiliate {
    #[builder(setter(
        into,
        doc = "Sets [`app_id`](ReaderPaymentRequestParamsAffiliate::app_id)."
    ))]
    pub app_id: String,
    #[builder(setter(into, doc = "Sets [`key`](ReaderPaymentRequestParamsAffiliate::key)."))]
    pub key: String,
}
/// Type of connection used by the device
///
/// Example: `Wi-Fi`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StatusResponseDataConnectionType {
    #[serde(rename = "btle")]
    Btle,
    #[serde(rename = "edge")]
    Edge,
    #[serde(rename = "gprs")]
    Gprs,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "umts")]
    Umts,
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "Wi-Fi")]
    WiFi,
    #[serde(untagged)]
    Other(String),
}
/// Latest state of the device
///
/// Example: `IDLE`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StatusResponseDataState {
    #[serde(rename = "IDLE")]
    Idle,
    #[serde(rename = "SELECTING_TIP")]
    SelectingTip,
    #[serde(rename = "WAITING_FOR_CARD")]
    WaitingForCard,
    #[serde(rename = "WAITING_FOR_PIN")]
    WaitingForPin,
    #[serde(rename = "WAITING_FOR_SIGNATURE")]
    WaitingForSignature,
    #[serde(rename = "UPDATING_FIRMWARE")]
    UpdatingFirmware,
    #[serde(untagged)]
    Other(String),
}
/// Status of a device
///
/// Example: `ONLINE`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StatusResponseDataStatus {
    #[serde(rename = "ONLINE")]
    Online,
    #[serde(rename = "OFFLINE")]
    Offline,
    #[serde(untagged)]
    Other(String),
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct StatusResponseData {
    /// Battery level percentage
    ///
    /// Constraints:
    /// - value >= 0
    /// - value <= 100
    ///
    /// Example: `10.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`battery_level`](StatusResponseData::battery_level)."
        )
    )]
    pub battery_level: Option<f32>,
    /// Battery temperature in Celsius
    ///
    /// Example: `35`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`battery_temperature`](StatusResponseData::battery_temperature)."
        )
    )]
    pub battery_temperature: Option<i64>,
    /// Type of connection used by the device
    ///
    /// Example: `Wi-Fi`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`connection_type`](StatusResponseData::connection_type)."
        )
    )]
    pub connection_type: Option<StatusResponseDataConnectionType>,
    /// Firmware version of the device
    ///
    /// Example: `3.3.3.21`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`firmware_version`](StatusResponseData::firmware_version)."
        )
    )]
    pub firmware_version: Option<String>,
    /// Timestamp of the last activity from the device
    ///
    /// Example: `2025-09-25T15:20:00Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`last_activity`](StatusResponseData::last_activity)."
        )
    )]
    pub last_activity: Option<crate::datetime::DateTime>,
    /// Latest state of the device
    ///
    /// Example: `IDLE`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`state`](StatusResponseData::state).")
    )]
    pub state: Option<StatusResponseDataState>,
    /// Status of a device
    ///
    /// Example: `ONLINE`
    #[builder(setter(doc = "Sets [`status`](StatusResponseData::status)."))]
    pub status: StatusResponseDataStatus,
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
/// Optional object containing data for transactions from ERP integrators in Greece that comply with the AADE 1155 protocol.
/// When such regulatory/business requirements apply, this object must be provided and contains the data needed to validate the transaction with the AADE signature provider.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateCheckoutRequestAade {
    /// The identifier of the AADE signature provider.
    ///
    /// Example: `123`
    #[builder(setter(
        into,
        doc = "Sets [`provider_id`](CreateCheckoutRequestAade::provider_id)."
    ))]
    pub provider_id: String,
    /// The base64 encoded signature of the transaction data.
    ///
    /// Example: `QjcxRDdBNTU1MDcyRTNFRTREMkZEM0Y0NTdBMjkxMTU4MzBFNkNCQTs7MjAyNTExMTIyMTQ3MTM7Nzk2OzEwNDs5MDA7OTAwOzU0ODg5MDM5`
    #[builder(setter(
        into,
        doc = "Sets [`signature`](CreateCheckoutRequestAade::signature)."
    ))]
    pub signature: String,
    /// The string containing the signed transaction data.
    ///
    /// Example: `B71D7A555072E3EE4D2FD3F457A29115830E6CBA;;20251112214713;796;104;900;900;54889039`
    #[builder(setter(
        into,
        doc = "Sets [`signature_data`](CreateCheckoutRequestAade::signature_data)."
    ))]
    pub signature_data: String,
}
/// Optional key-value metadata associated with the transaction.
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
pub struct CreateCheckoutRequestAffiliateTags {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    #[builder(
        default,
        setter(
            doc = "Sets [`additional_properties`](CreateCheckoutRequestAffiliateTags::additional_properties)."
        )
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
/// Optional affiliate metadata associated with the checkout's payment.
/// Provide both `app_id` and `key` when including this object.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateCheckoutRequestAffiliate {
    /// Integrator application identifier from the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    /// Required when the affiliate object is provided.
    ///
    /// Example: `com.example.app`
    #[builder(setter(into, doc = "Sets [`app_id`](CreateCheckoutRequestAffiliate::app_id)."))]
    pub app_id: String,
    /// Integrator-owned transaction reference. Optional.
    /// Can be used to retrieve the transaction via the [Transactions API](https://developer.sumup.com/api/transactions/get).
    ///
    /// Example: `19e12390-72cf-4f9f-80b5-b0c8a67fa43f`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`foreign_transaction_id`](CreateCheckoutRequestAffiliate::foreign_transaction_id)."
        )
    )]
    pub foreign_transaction_id: Option<String>,
    /// Affiliate key generated in the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    /// Required when the affiliate object is provided.
    ///
    /// Example: `123e4567-e89b-12d3-a456-426614174000`
    #[builder(setter(into, doc = "Sets [`key`](CreateCheckoutRequestAffiliate::key)."))]
    pub key: String,
    /// Optional key-value metadata associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tags`](CreateCheckoutRequestAffiliate::tags)."
        )
    )]
    pub tags: Option<serde_json::Value>,
}
/// The card type of the card used for the transaction.
/// It is required only for some countries (e.g: Brazil).
///
/// Example: `credit`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateCheckoutRequestCardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(untagged)]
    Other(String),
}
/// The payment method type for the transaction. Defaults to `card`.
///
/// Use `pix` to initiate a Pix QR-code payment flow on the device.
/// Pix is only supported for Brazilian merchants; other merchants will receive a 422 error.
///
/// Example: `card`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateCheckoutRequestPaymentType {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "pix")]
    Pix,
    #[serde(untagged)]
    Other(String),
}
/// Amount structure.
///
/// The amount is represented as an integer value altogether with the currency and the minor unit.
///
/// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateCheckoutRequestTotalAmount {
    /// Currency ISO 4217 code
    ///
    /// Example: `EUR`
    #[builder(setter(
        into,
        doc = "Sets [`currency`](CreateCheckoutRequestTotalAmount::currency)."
    ))]
    pub currency: String,
    /// The minor units of the currency.
    /// It represents the number of decimals of the currency. For the currencies CLP, COP and HUF, the minor unit is 0.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `2`
    #[builder(setter(doc = "Sets [`minor_unit`](CreateCheckoutRequestTotalAmount::minor_unit)."))]
    pub minor_unit: i64,
    /// Integer value of the amount.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `1000`
    #[builder(setter(doc = "Sets [`value`](CreateCheckoutRequestTotalAmount::value)."))]
    pub value: i64,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateGoCheckoutRequestAffiliate {
    #[builder(setter(
        into,
        doc = "Sets [`app_id`](CreateGoCheckoutRequestAffiliate::app_id)."
    ))]
    pub app_id: String,
    #[builder(setter(into, doc = "Sets [`key`](CreateGoCheckoutRequestAffiliate::key)."))]
    pub key: String,
}
/// Returns a list Reader objects.
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
    pub items: Vec<Reader>,
}
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`metadata`](CreateRequest::metadata).")
    )]
    pub metadata: Option<Metadata>,
    #[builder(setter(into, doc = "Sets [`name`](CreateRequest::name)."))]
    pub name: ReaderName,
    #[builder(setter(into, doc = "Sets [`pairing_code`](CreateRequest::pairing_code)."))]
    pub pairing_code: ReaderPairingCode,
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
        setter(strip_option, doc = "Sets [`metadata`](UpdateRequest::metadata).")
    )]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`name`](UpdateRequest::name).")
    )]
    pub name: Option<ReaderName>,
}
/// A checkout initial attributes
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateCheckoutRequest {
    /// Optional object containing data for transactions from ERP integrators in Greece that comply with the AADE 1155 protocol.
    /// When such regulatory/business requirements apply, this object must be provided and contains the data needed to validate the transaction with the AADE signature provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`aade`](CreateCheckoutRequest::aade).")
    )]
    pub aade: Option<CreateCheckoutRequestAade>,
    /// Optional affiliate metadata associated with the checkout's payment.
    /// Provide both `app_id` and `key` when including this object.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`affiliate`](CreateCheckoutRequest::affiliate)."
        )
    )]
    pub affiliate: Option<crate::Nullable<CreateCheckoutRequestAffiliate>>,
    /// The card type of the card used for the transaction.
    /// It is required only for some countries (e.g: Brazil).
    ///
    /// Example: `credit`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`card_type`](CreateCheckoutRequest::card_type)."
        )
    )]
    pub card_type: Option<CreateCheckoutRequestCardType>,
    /// Description of the checkout to be shown in the Merchant Sales
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`description`](CreateCheckoutRequest::description)."
        )
    )]
    pub description: Option<String>,
    /// Number of installments for the transaction.
    /// It may vary according to the merchant country.
    /// For example, in Brazil, the maximum number of installments is 12.
    ///
    /// Omit if the merchant country does not support installments.
    /// Otherwise, the checkout will be rejected.
    ///
    /// Constraints:
    /// - value >= 1
    ///
    /// Example: `1`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`installments`](CreateCheckoutRequest::installments)."
        )
    )]
    pub installments: Option<crate::Nullable<i64>>,
    /// The payment method type for the transaction. Defaults to `card`.
    ///
    /// Use `pix` to initiate a Pix QR-code payment flow on the device.
    /// Pix is only supported for Brazilian merchants; other merchants will receive a 422 error.
    ///
    /// Example: `card`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`payment_type`](CreateCheckoutRequest::payment_type)."
        )
    )]
    pub payment_type: Option<CreateCheckoutRequestPaymentType>,
    /// Webhook URL to which the payment result will be sent.
    /// It must be a HTTPS url.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `https://www.example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`return_url`](CreateCheckoutRequest::return_url)."
        )
    )]
    pub return_url: Option<String>,
    /// Exactly three distinct tipping rates to be displayed to the cardholder.
    /// Each rate is a fraction between 0.01 and 1.00 in increments of 0.01.
    /// The rates are sorted in ascending order by the server.
    ///
    /// Constraints:
    /// - min items: 3
    /// - max items: 3
    /// - items must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tip_rates`](CreateCheckoutRequest::tip_rates)."
        )
    )]
    pub tip_rates: Option<Vec<f32>>,
    /// Time in seconds the cardholder has to select a tip rate.
    /// If not provided, the default value is 30 seconds.
    ///
    /// It can only be set if `tip_rates` is provided.
    ///
    /// **Note**: If the target device is a Solo, it must be in version 3.3.38.0 or higher.
    ///
    /// Constraints:
    /// - value >= 30
    /// - value <= 120
    ///
    /// Example: `30`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tip_timeout`](CreateCheckoutRequest::tip_timeout)."
        )
    )]
    pub tip_timeout: Option<i64>,
    /// Amount structure.
    ///
    /// The amount is represented as an integer value altogether with the currency and the minor unit.
    ///
    /// For example, EUR 1.00 is represented as value 100 with minor unit of 2.
    #[builder(setter(doc = "Sets [`total_amount`](CreateCheckoutRequest::total_amount)."))]
    pub total_amount: CreateCheckoutRequestTotalAmount,
}
/// Payment details to initiate on the reader.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateGoCheckoutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`affiliate`](CreateGoCheckoutRequest::affiliate)."
        )
    )]
    pub affiliate: Option<CreateGoCheckoutRequestAffiliate>,
    /// Caller-supplied correlation identifier, used as the idempotency key.
    ///
    /// Example: `19e12390-72cf-4f9f-80b5-b0c8a67fa43f`
    #[builder(setter(
        into,
        doc = "Sets [`client_transaction_id`](CreateGoCheckoutRequest::client_transaction_id)."
    ))]
    pub client_transaction_id: String,
    /// Portion of total_amount that is a tip, in minor units.
    ///
    /// Example: `200`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`tip_amount`](CreateGoCheckoutRequest::tip_amount)."
        )
    )]
    pub tip_amount: Option<i64>,
    #[builder(setter(doc = "Sets [`total_amount`](CreateGoCheckoutRequest::total_amount)."))]
    pub total_amount: Amount,
}
use crate::client::Client;
#[derive(Debug, PartialEq)]
pub enum ListErrorBody {
    Unauthorized(Problem),
}
impl crate::error::ProblemDetails for ListErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum CreateErrorBody {
    BadRequest(Problem),
    NotFound(Problem),
    Conflict(Problem),
}
impl crate::error::ProblemDetails for CreateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::Conflict(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum DeleteErrorBody {
    NotFound(Problem),
}
impl crate::error::ProblemDetails for DeleteErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
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
    Forbidden(Problem),
    NotFound(Problem),
}
impl crate::error::ProblemDetails for UpdateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Forbidden(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum CreateCheckoutErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for CreateCheckoutErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum GetCheckoutErrorBody {
    Unauthorized(Problem),
    NotFound(Problem),
}
impl crate::error::ProblemDetails for GetCheckoutErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum GetReaderSettingsErrorBody {
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for GetReaderSettingsErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum UpdateReaderSettingsErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for UpdateReaderSettingsErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum GetStatusErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
}
impl crate::error::ProblemDetails for GetStatusErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum TerminateCheckoutErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for TerminateCheckoutErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum CreateGoCheckoutErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for CreateGoCheckoutErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
/// Client for the Readers API endpoints.
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
    ///
    /// Responses:
    /// - 200: Returns a list Reader objects.
    /// - 401: Authentication failed or missing required scope.
    pub async fn list(
        &self,
        merchant_code: impl AsRef<str>,
    ) -> crate::error::SdkResult<ListResponse, ListErrorBody> {
        let path = format!("/v0.1/merchants/{}/readers", merchant_code.as_ref());
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
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(ListErrorBody::Unauthorized)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Create a Reader
    ///
    /// Create a new Reader for the merchant account.
    ///
    /// Responses:
    /// - 201: Returns the Reader object if the creation succeeded.
    /// - 400: The request is invalid.
    /// - 404: There's no pending reader for the submitted pairing code.
    /// - 409: The Reader is not in a pending state.
    pub async fn create(
        &self,
        merchant_code: impl AsRef<str>,
        body: CreateRequest,
    ) -> crate::error::SdkResult<Reader, CreateErrorBody> {
        let path = format!("/v0.1/merchants/{}/readers", merchant_code.as_ref());
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
            reqwest::StatusCode::CONFLICT => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::Conflict)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Delete a reader
    ///
    /// Delete a reader.
    ///
    /// Responses:
    /// - 200: Returns an empty response if the deletion succeeded.
    /// - 404: The requested Reader resource does not exist.
    pub async fn delete(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<(), DeleteErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}",
            merchant_code.as_ref(),
            reader_id.as_ref()
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(DeleteErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Retrieve a Reader
    ///
    /// Retrieve a Reader.
    ///
    /// Responses:
    /// - 200: Returns a Reader object for a valid identifier.
    /// - 404: The requested Reader resource does not exist.
    pub async fn get(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<Reader, GetErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}",
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
                let data: Reader = response.json().await?;
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
    /// Update a Reader
    ///
    /// Updates a reader's name or metadata and returns the updated reader.
    ///
    /// Providing `metadata` replaces the entire metadata object; include all entries that should be retained. Omitted fields remain unchanged.
    ///
    /// Responses:
    /// - 200: Returns the updated Reader object if the update succeeded.
    /// - 403: The request isn't sufficiently authorized to modify the reader.
    /// - 404: The requested Reader resource does not exist.
    pub async fn update(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        body: UpdateRequest,
    ) -> crate::error::SdkResult<Reader, UpdateErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}",
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
                let data: Reader = response.json().await?;
                Ok(data)
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
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Create a Reader Checkout
    ///
    /// Creates a Checkout for a Reader.
    ///
    /// This process is asynchronous and the actual transaction may take some time to be started on the device.
    ///
    ///
    /// There are some caveats when using this endpoint:
    /// * The target device must be online, otherwise checkout won't be accepted
    /// * After the checkout is accepted, the system has 60 seconds to start the payment on the target device. During this time, any other checkout for the same device will be rejected.
    ///
    ///
    /// **Note**: If the target device is a Solo, it must be in version 3.3.24.3 or higher.
    ///
    ///
    /// Optionally include an `affiliate` object to attach integrator metadata to the payment.
    /// Provide both `app_id` and `key` when including this object.
    ///
    /// Responses:
    /// - 201: The Checkout got successfully created for the given reader.
    /// - 400: Response when given params (or one of them) are invalid
    /// - 401: Unauthorized
    /// - 404: Response when given reader is not found
    /// - 422: Response when given params (or one of them) are invalid
    pub async fn create_checkout(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        body: CreateCheckoutRequest,
    ) -> crate::error::SdkResult<CreateReaderCheckoutResponse, CreateCheckoutErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/checkout",
            merchant_code.as_ref(),
            reader_id.as_ref()
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateCheckoutErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(CreateCheckoutErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateCheckoutErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(CreateCheckoutErrorBody::UnprocessableEntity)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Get a Reader Checkout
    ///
    /// Get a Checkout for a Reader.
    ///
    /// Responses:
    /// - 200: The Checkout got successfully retrieved for the given reader.
    /// - 401: Unauthorized
    /// - 404: Response when given reader or checkout is not found
    pub async fn get_checkout(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        checkout_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<GetReaderCheckoutResponse, GetCheckoutErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/checkout/{}",
            merchant_code.as_ref(),
            reader_id.as_ref(),
            checkout_id.as_ref()
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
                let data: GetReaderCheckoutResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetCheckoutErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetCheckoutErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
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
    /// - 422: The reader model is not supported, or the reader could not return its settings.
    pub async fn get_reader_settings(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<GetReaderSettingsResponse, GetReaderSettingsErrorBody> {
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
                    serde_json::from_slice::<Problem>(raw)
                        .map(GetReaderSettingsErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetReaderSettingsErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(GetReaderSettingsErrorBody::UnprocessableEntity)
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
    ///   supported, or the reader could not apply the update.
    pub async fn update_reader_settings(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        body: UpdateReaderSettingsRequest,
    ) -> crate::error::SdkResult<UpdateReaderSettingsResponse, UpdateReaderSettingsErrorBody> {
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
                    serde_json::from_slice::<Problem>(raw)
                        .map(UpdateReaderSettingsErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(UpdateReaderSettingsErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(UpdateReaderSettingsErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(UpdateReaderSettingsErrorBody::UnprocessableEntity)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
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
    ///
    /// Responses:
    /// - 200: Response with the device status.
    /// - 400: Response when given params (or one of them) are invalid
    /// - 401: Response when given merchant's token is invalid
    /// - 404: Response when given reader is not found
    pub async fn get_status(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<StatusResponse, GetStatusErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/status",
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
                let data: StatusResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetStatusErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetStatusErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetStatusErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
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
    ///
    /// Responses:
    /// - 202: The Terminate action was successfully dispatched for the given reader.
    /// - 400: Response when given params (or one of them) are invalid
    /// - 401: Unauthorized
    /// - 404: Response when given reader is not found
    /// - 422: Response when given params (or one of them) are invalid
    pub async fn terminate_checkout(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<(), TerminateCheckoutErrorBody> {
        let path = format!(
            "/v0.1/merchants/{}/readers/{}/terminate",
            merchant_code.as_ref(),
            reader_id.as_ref()
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(TerminateCheckoutErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(TerminateCheckoutErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(TerminateCheckoutErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(TerminateCheckoutErrorBody::UnprocessableEntity)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Create a Go Reader Payment
    ///
    /// Initiates a payment on the SumUp Go terminal identified by the reader ID.
    ///
    /// Use `client_transaction_id` as an idempotency key: retrying the request with the same value returns the result of the original payment instead of creating a duplicate.
    ///
    /// Responses:
    /// - 200: Returns the result of the payment initiated on the reader.
    /// - 400: The request is invalid.
    /// - 401: Authentication failed or missing required scope.
    /// - 404: The requested Reader resource does not exist.
    /// - 422: The request could not be processed as it violates a business rule.
    pub async fn create_go_checkout(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        body: CreateGoCheckoutRequest,
    ) -> crate::error::SdkResult<ReaderPaymentResponse, CreateGoCheckoutErrorBody> {
        let path = format!(
            "/v0/merchants/{}/readers/{}/go-checkout",
            merchant_code.as_ref(),
            reader_id.as_ref()
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
            reqwest::StatusCode::OK => {
                let data: ReaderPaymentResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(CreateGoCheckoutErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(CreateGoCheckoutErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateGoCheckoutErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(CreateGoCheckoutErrorBody::UnprocessableEntity)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
