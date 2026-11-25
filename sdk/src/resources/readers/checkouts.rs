// The contents of this file are generated; do not modify them.

//! Create, retrieve, and terminate payments on a card reader.
pub use crate::resources::common::*;
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
    /// Amount to charge in minor units. Use the decimal places for the currency.
    /// CLP, COP, and HUF use zero decimal places.
    ///
    /// For example, send `1234` for EUR 12.34 or `1000` for CLP 1000.
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
/// Amount to charge in minor units. Use the decimal places for the currency.
/// CLP, COP, and HUF use zero decimal places.
///
/// For example, send `1234` for EUR 12.34 or `1000` for CLP 1000.
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
/// The checkout amount. The value uses two decimal places, even for currencies
/// such as CLP that use zero decimal places in the create request.
///
/// For example, EUR 10.00 is returned as `1000` and CLP 1000 as `100000`.
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
    /// The checkout amount. The value uses two decimal places, even for currencies
    /// such as CLP that use zero decimal places in the create request.
    ///
    /// For example, EUR 10.00 is returned as `1000` and CLP 1000 as `100000`.
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
/// Optional object containing data for transactions from ERP integrators in Greece that comply with the AADE 1155 protocol.
/// When such regulatory/business requirements apply, this object must be provided and contains the data needed to validate the transaction with the AADE signature provider.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequestAade {
    /// The identifier of the AADE signature provider.
    ///
    /// Example: `123`
    #[builder(setter(into, doc = "Sets [`provider_id`](CreateRequestAade::provider_id)."))]
    pub provider_id: String,
    /// The base64 encoded signature of the transaction data.
    ///
    /// Example: `QjcxRDdBNTU1MDcyRTNFRTREMkZEM0Y0NTdBMjkxMTU4MzBFNkNCQTs7MjAyNTExMTIyMTQ3MTM7Nzk2OzEwNDs5MDA7OTAwOzU0ODg5MDM5`
    #[builder(setter(into, doc = "Sets [`signature`](CreateRequestAade::signature)."))]
    pub signature: String,
    /// The string containing the signed transaction data.
    ///
    /// Example: `B71D7A555072E3EE4D2FD3F457A29115830E6CBA;;20251112214713;796;104;900;900;54889039`
    #[builder(setter(
        into,
        doc = "Sets [`signature_data`](CreateRequestAade::signature_data)."
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
pub struct CreateRequestAffiliateTags {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    #[builder(
        default,
        setter(
            doc = "Sets [`additional_properties`](CreateRequestAffiliateTags::additional_properties)."
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
pub struct CreateRequestAffiliate {
    /// Integrator application identifier from the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    /// Required when the affiliate object is provided.
    ///
    /// Example: `com.example.app`
    #[builder(setter(into, doc = "Sets [`app_id`](CreateRequestAffiliate::app_id)."))]
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
            doc = "Sets [`foreign_transaction_id`](CreateRequestAffiliate::foreign_transaction_id)."
        )
    )]
    pub foreign_transaction_id: Option<String>,
    /// Affiliate key generated in the [Affiliate Keys](https://developer.sumup.com/affiliate-keys) page.
    /// Required when the affiliate object is provided.
    ///
    /// Example: `123e4567-e89b-12d3-a456-426614174000`
    #[builder(setter(into, doc = "Sets [`key`](CreateRequestAffiliate::key)."))]
    pub key: String,
    /// Optional key-value metadata associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`tags`](CreateRequestAffiliate::tags).")
    )]
    pub tags: Option<serde_json::Value>,
}
/// The card type of the card used for the transaction.
/// It is required only for some countries (e.g: Brazil).
///
/// Example: `credit`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateRequestCardType {
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
pub enum CreateRequestPaymentType {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "pix")]
    Pix,
    #[serde(untagged)]
    Other(String),
}
/// Amount to charge in minor units. Use the decimal places for the currency.
/// CLP, COP, and HUF use zero decimal places.
///
/// For example, send `1234` for EUR 12.34 or `1000` for CLP 1000.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequestTotalAmount {
    /// Currency ISO 4217 code
    ///
    /// Example: `EUR`
    #[builder(setter(into, doc = "Sets [`currency`](CreateRequestTotalAmount::currency)."))]
    pub currency: String,
    /// Integer value of the amount.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `1000`
    #[builder(setter(doc = "Sets [`value`](CreateRequestTotalAmount::value)."))]
    pub value: i64,
}
/// A checkout initial attributes
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequest {
    /// Optional object containing data for transactions from ERP integrators in Greece that comply with the AADE 1155 protocol.
    /// When such regulatory/business requirements apply, this object must be provided and contains the data needed to validate the transaction with the AADE signature provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`aade`](CreateRequest::aade).")
    )]
    pub aade: Option<CreateRequestAade>,
    /// Optional affiliate metadata associated with the checkout's payment.
    /// Provide both `app_id` and `key` when including this object.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`affiliate`](CreateRequest::affiliate).")
    )]
    pub affiliate: Option<crate::Nullable<CreateRequestAffiliate>>,
    /// The card type of the card used for the transaction.
    /// It is required only for some countries (e.g: Brazil).
    ///
    /// Example: `credit`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`card_type`](CreateRequest::card_type).")
    )]
    pub card_type: Option<CreateRequestCardType>,
    /// Description of the checkout to be shown in the Merchant Sales
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
            doc = "Sets [`installments`](CreateRequest::installments)."
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
            doc = "Sets [`payment_type`](CreateRequest::payment_type)."
        )
    )]
    pub payment_type: Option<CreateRequestPaymentType>,
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
            doc = "Sets [`return_url`](CreateRequest::return_url)."
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
        setter(strip_option, doc = "Sets [`tip_rates`](CreateRequest::tip_rates).")
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
            doc = "Sets [`tip_timeout`](CreateRequest::tip_timeout)."
        )
    )]
    pub tip_timeout: Option<i64>,
    /// Amount to charge in minor units. Use the decimal places for the currency.
    /// CLP, COP, and HUF use zero decimal places.
    ///
    /// For example, send `1234` for EUR 12.34 or `1000` for CLP 1000.
    #[builder(setter(doc = "Sets [`total_amount`](CreateRequest::total_amount)."))]
    pub total_amount: CreateRequestTotalAmount,
}
use crate::client::Client;
#[derive(Debug, PartialEq)]
pub enum CreateErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for CreateErrorBody {
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
pub enum GetErrorBody {
    Unauthorized(Problem),
    NotFound(Problem),
}
impl crate::error::ProblemDetails for GetErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum TerminateErrorBody {
    BadRequest(Problem),
    Unauthorized(Problem),
    NotFound(Problem),
    UnprocessableEntity(Problem),
}
impl crate::error::ProblemDetails for TerminateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(problem) => Some(problem),
            Self::Unauthorized(problem) => Some(problem),
            Self::NotFound(problem) => Some(problem),
            Self::UnprocessableEntity(problem) => Some(problem),
        }
    }
}
/// Client for the ReaderCheckouts API endpoints.
#[derive(Debug)]
pub struct ReaderCheckoutsClient<'a> {
    client: &'a Client,
}
impl<'a> ReaderCheckoutsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
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
    pub async fn create(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        body: CreateRequest,
    ) -> crate::error::SdkResult<CreateReaderCheckoutResponse, CreateErrorBody> {
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
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::UnprocessableEntity)
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
    pub async fn get(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
        checkout_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<GetReaderCheckoutResponse, GetErrorBody> {
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
    pub async fn terminate(
        &self,
        merchant_code: impl AsRef<str>,
        reader_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<(), TerminateErrorBody> {
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
                    serde_json::from_slice::<Problem>(raw).map(TerminateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(TerminateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(TerminateErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(TerminateErrorBody::UnprocessableEntity)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
