// The contents of this file are generated; do not modify them.

use super::common::*;
/// Details of the payment card.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CardResponse {
    /// Last 4 digits of the payment card number.
    ///
    /// Constraints:
    /// - read-only
    /// - min length: 4
    /// - max length: 4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CardType>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Device {
    /// Device name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Device OS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_name: Option<String>,
    /// Device model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Device OS version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_version: Option<String>,
    /// Device UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ElvCardAccount {
    /// ELV card sort code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    /// ELV card account number last 4 digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    /// ELV card sequence number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_no: Option<i64>,
    /// ELV IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
}
/// Entry mode value accepted by the `entry_modes[]` filter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EntryModeFilter {
    #[serde(rename = "BOLETO")]
    Boleto,
    #[serde(rename = "SOFORT")]
    Sofort,
    #[serde(rename = "IDEAL")]
    Ideal,
    #[serde(rename = "BANCONTACT")]
    Bancontact,
    #[serde(rename = "EPS")]
    Eps,
    #[serde(rename = "MYBANK")]
    Mybank,
    #[serde(rename = "SATISPAY")]
    Satispay,
    #[serde(rename = "BLIK")]
    Blik,
    P24,
    #[serde(rename = "GIROPAY")]
    Giropay,
    #[serde(rename = "PIX")]
    Pix,
    #[serde(rename = "QR_CODE_PIX")]
    QrCodePix,
    #[serde(rename = "APPLE_PAY")]
    ApplePay,
    #[serde(rename = "GOOGLE_PAY")]
    GooglePay,
    #[serde(rename = "PAYPAL")]
    Paypal,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "CHIP")]
    Chip,
    #[serde(rename = "MANUAL_ENTRY")]
    ManualEntry,
    #[serde(rename = "CUSTOMER_ENTRY")]
    CustomerEntry,
    #[serde(rename = "MAGSTRIPE_FALLBACK")]
    MagstripeFallback,
    #[serde(rename = "MAGSTRIPE")]
    Magstripe,
    #[serde(rename = "DIRECT_DEBIT")]
    DirectDebit,
    #[serde(rename = "CONTACTLESS")]
    Contactless,
    #[serde(rename = "MOTO")]
    Moto,
    #[serde(rename = "CONTACTLESS_MAGSTRIPE")]
    ContactlessMagstripe,
    #[serde(rename = "N/A")]
    NA,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>,
    /// Amount of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Date and time of the transaction event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Amount of the fee related to the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f32>,
    /// Consecutive number of the installment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_number: Option<i64>,
    /// Amount deducted for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deducted_amount: Option<f32>,
    /// Amount of the fee deducted for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deducted_fee_amount: Option<f32>,
}
pub type HorizontalAccuracy = f32;
pub type Lat = f32;
/// Details of a link to a related resource.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// Specifies the relation to the current resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    /// URL for accessing the related resource.
    ///
    /// Constraints:
    /// - format: `uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Specifies the media type of the related resource.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
pub type Lon = f32;
/// Purchase product.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Product {
    /// Product name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_label: Option<String>,
    /// Product price.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// VAT percentage.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate: Option<f64>,
    /// VAT amount for a single product.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_vat_amount: Option<f64>,
    /// Product price incl. VAT.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_with_vat: Option<f64>,
    /// VAT amount.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f64>,
    /// Product quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Quantity x product price.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<f64>,
    /// Total price incl. VAT.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_with_vat: Option<f64>,
}
/// Details of a transaction event.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>,
    /// Amount of the event.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Date when the transaction event is due to occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<crate::datetime::Date>,
    /// Date when the transaction event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::Date>,
    /// Consecutive number of the installment that is paid. Applicable only payout events, i.e. `event_type = PAYOUT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_number: Option<i64>,
    /// Date and time of the transaction event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionFull {
    /// Unique ID of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction code returned by the acquirer/processing entity after processing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Total amount of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Date and time of the creation of the transaction. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Current status of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    /// Current number of the installment for deferred payments.
    ///
    /// Constraints:
    /// - value >= 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
    /// Unique code of the registered merchant to whom the payment is made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Amount of the applicable VAT (out of the total transaction amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f32>,
    /// Amount of the tip (out of the total transaction amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<EntryMode>,
    /// Authorization code for the transaction sent by the payment card issuer or bank. Applicable only to card payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// Internal unique ID of the transaction on the SumUp platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<i64>,
    /// Short description of the payment. The value is taken from the `description` property of the related checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_summary: Option<String>,
    /// Total number of payouts to the registered user specified in the `user` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_total: Option<i64>,
    /// Number of payouts that are made to the registered user specified in the `user` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_received: Option<i64>,
    /// Payout plan of the registered user at the time when the transaction was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_plan: Option<String>,
    /// External/foreign transaction id (passed by clients).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_transaction_id: Option<String>,
    /// Client transaction id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_transaction_id: Option<String>,
    /// Email address of the registered user (merchant) to whom the payment is made.
    ///
    /// Constraints:
    /// - format: `email`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Transaction SumUp total fee amount.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<Lat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<Lon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<HorizontalAccuracy>,
    /// SumUp merchant internal Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<Device>,
    /// Simple name of the payment type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_payment_type: Option<String>,
    /// Verification method used for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elv_account: Option<ElvCardAccount>,
    /// Local date and time of the creation of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<crate::datetime::DateTime>,
    /// The date of the payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_date: Option<crate::datetime::Date>,
    /// Payout type for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<String>,
    /// Debit/Credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_as: Option<String>,
    /// List of products from the merchant's catalogue for which the transaction serves as a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<Product>>,
    /// List of VAT rates applicable to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rates: Option<Vec<TransactionFullVatRatesItem>>,
    /// List of transaction events related to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_events: Option<Vec<TransactionEvent>>,
    /// Status generated from the processing status and the latest transaction state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_status: Option<String>,
    /// List of hyperlinks for accessing related resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// List of events related to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// Details of the payment location as received from the payment terminal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TransactionFullLocation>,
    /// Indicates whether tax deduction is enabled for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_enabled: Option<bool>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionHistory {
    /// Unique ID of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction code returned by the acquirer/processing entity after processing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Total amount of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Date and time of the creation of the transaction. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Current status of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    /// Current number of the installment for deferred payments.
    ///
    /// Constraints:
    /// - value >= 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
    /// Short description of the payment. The value is taken from the `description` property of the related checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_summary: Option<String>,
    /// Total number of payouts to the registered user specified in the `user` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_total: Option<i64>,
    /// Number of payouts that are made to the registered user specified in the `user` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_received: Option<i64>,
    /// Payout plan of the registered user at the time when the transaction was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
    /// Client-specific ID of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_transaction_id: Option<String>,
    /// Email address of the registered user (merchant) to whom the payment is made.
    ///
    /// Constraints:
    /// - format: `email`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Type of the transaction for the registered user specified in the `user` property.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<CardType>,
    /// Payout date (if paid out at once).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_date: Option<crate::datetime::Date>,
    /// Payout type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<String>,
    /// Total refunded amount.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<f64>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionsHistoryLink {
    /// Relation.
    pub rel: String,
    /// Location.
    pub href: String,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionFullVatRatesItem {
    /// VAT rate.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /// NET amount of products having this VAT rate applied.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<f64>,
    /// VAT amount of this rate applied.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<f64>,
    /// Gross amount of products having this VAT rate applied.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross: Option<f64>,
}
/// Details of the payment location as received from the payment terminal.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionFullLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<Lat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<Lon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<HorizontalAccuracy>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RefundBody {
    /// Amount to be refunded. Eligible amount can't exceed the amount of the transaction and varies based on country and currency. If you do not specify a value, the system performs a full refund of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetDeprecatedParams {
    /// Retrieves the transaction resource with the specified transaction ID (the `id` parameter in the transaction resource).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Retrieves the transaction resource with the specified internal transaction ID (the `internal_id` parameter in the transaction resource).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    /// Retrieves the transaction resource with the specified transaction code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListDeprecatedParams {
    /// Retrieves the transaction resource with the specified transaction code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Specifies the order in which the returned results are displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Specifies the maximum number of results per page. Value must be a positive integer and if not specified, will return 10 results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filters the returned results by user email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// Filters the returned results by the specified list of final statuses of the transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// Filters the returned results by the specified list of payment types used for the transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_types: Option<Vec<PaymentType>>,
    /// Filters the returned results by the specified list of transaction types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// Filters the results by the latest modification time of resources and returns only transactions that are modified *at or after* the specified timestamp (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes_since: Option<crate::datetime::DateTime>,
    /// Filters the results by the creation time of resources and returns only transactions that are created *before* the specified timestamp (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_time: Option<crate::datetime::DateTime>,
    /// Filters the results by the reference ID of transaction events and returns only transactions with events whose IDs are *smaller* than the specified value. This parameters supersedes the `newest_time` parameter (if both are provided in the request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_ref: Option<String>,
    /// Filters the results by the creation time of resources and returns only transactions that are created *at or after* the specified timestamp (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_time: Option<crate::datetime::DateTime>,
    /// Filters the results by the reference ID of transaction events and returns only transactions with events whose IDs are *greater* than the specified value. This parameters supersedes the `oldest_time` parameter (if both are provided in the request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_ref: Option<String>,
}
/// OK
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListDeprecatedResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TransactionHistory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TransactionsHistoryLink>>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetParams {
    /// Retrieves the transaction resource with the specified transaction ID (the `id` parameter in the transaction resource).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Retrieves the transaction resource with the specified internal transaction ID (the `internal_id` parameter in the transaction resource).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    /// Retrieves the transaction resource with the specified transaction code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// External/foreign transaction id (passed by clients).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_transaction_id: Option<String>,
    /// Client transaction id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_transaction_id: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListParams {
    /// Retrieves the transaction resource with the specified transaction code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Specifies the order in which the returned results are displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Specifies the maximum number of results per page. Value must be a positive integer and if not specified, will return 10 results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filters the returned results by user email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// Filters the returned results by the specified list of final statuses of the transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// Filters the returned results by the specified list of payment types used for the transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_types: Option<Vec<PaymentType>>,
    /// Filters the returned results by the specified list of entry modes.
    #[serde(rename = "entry_modes[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_modes: Option<Vec<EntryModeFilter>>,
    /// Filters the returned results by the specified list of transaction types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// Filters the results by the latest modification time of resources and returns only transactions that are modified *at or after* the specified timestamp (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes_since: Option<crate::datetime::DateTime>,
    /// Filters the results by the creation time of resources and returns only transactions that are created *before* the specified timestamp (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_time: Option<crate::datetime::DateTime>,
    /// Filters the results by the reference ID of transaction events and returns only transactions with events whose IDs are *smaller* than the specified value. This parameters supersedes the `newest_time` parameter (if both are provided in the request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_ref: Option<String>,
    /// Filters the results by the creation time of resources and returns only transactions that are created *at or after* the specified timestamp (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_time: Option<crate::datetime::DateTime>,
    /// Filters the results by the reference ID of transaction events and returns only transactions with events whose IDs are *greater* than the specified value. This parameters supersedes the `oldest_time` parameter (if both are provided in the request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_ref: Option<String>,
}
/// OK
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TransactionHistory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TransactionsHistoryLink>>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum RefundErrorBody {
    NotFound(Error),
    Conflict(Error),
}
#[derive(Debug)]
pub enum GetDeprecatedErrorBody {
    Unauthorized(Error),
    NotFound(Error),
}
#[derive(Debug)]
pub enum ListDeprecatedErrorBody {
    Unauthorized(Error),
}
#[derive(Debug)]
pub enum GetErrorBody {
    Unauthorized(Error),
    NotFound(Error),
}
#[derive(Debug)]
pub enum ListErrorBody {
    Unauthorized(Error),
}
///Client for the Transactions API endpoints.
#[derive(Debug)]
pub struct TransactionsClient<'a> {
    client: &'a Client,
}
impl<'a> TransactionsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Refund a transaction
    ///
    /// Refunds an identified transaction either in full or partially.
    pub async fn refund(
        &self,
        txn_id: impl Into<String>,
        body: Option<RefundBody>,
    ) -> crate::error::SdkResult<(), RefundErrorBody> {
        let path = format!("/v0.1/me/refund/{}", txn_id.into());
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
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(RefundErrorBody::NotFound(body)))
            }
            reqwest::StatusCode::CONFLICT => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(RefundErrorBody::Conflict(body)))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Retrieve a transaction
    ///
    /// Retrieves the full details of an identified transaction. The transaction resource is identified by a query parameter and *one* of following parameters is required:
    ///
    /// *  `id`
    /// *  `internal_id`
    /// *  `transaction_code`
    /// *  `foreign_transaction_id`
    /// *  `client_transaction_id`
    pub async fn get_deprecated(
        &self,
        params: GetDeprecatedParams,
    ) -> crate::error::SdkResult<TransactionFull, GetDeprecatedErrorBody> {
        let path = "/v0.1/me/transactions";
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
        if let Some(ref value) = params.id {
            request = request.query(&[("id", value)]);
        }
        if let Some(ref value) = params.internal_id {
            request = request.query(&[("internal_id", value)]);
        }
        if let Some(ref value) = params.transaction_code {
            request = request.query(&[("transaction_code", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: TransactionFull = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetDeprecatedErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetDeprecatedErrorBody::NotFound(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// List transactions
    ///
    /// Lists detailed history of all transactions associated with the merchant profile.
    pub async fn list_deprecated(
        &self,
        params: ListDeprecatedParams,
    ) -> crate::error::SdkResult<ListDeprecatedResponse, ListDeprecatedErrorBody> {
        let path = "/v0.1/me/transactions/history";
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
        if let Some(ref value) = params.transaction_code {
            request = request.query(&[("transaction_code", value)]);
        }
        if let Some(ref value) = params.order {
            request = request.query(&[("order", value)]);
        }
        if let Some(ref value) = params.limit {
            request = request.query(&[("limit", value)]);
        }
        if let Some(ref value) = params.users {
            request = request.query(&[("users", value)]);
        }
        if let Some(ref value) = params.statuses {
            request = request.query(&[("statuses", value)]);
        }
        if let Some(ref value) = params.payment_types {
            request = request.query(&[("payment_types", value)]);
        }
        if let Some(ref value) = params.types {
            request = request.query(&[("types", value)]);
        }
        if let Some(ref value) = params.changes_since {
            request = request.query(&[("changes_since", value)]);
        }
        if let Some(ref value) = params.newest_time {
            request = request.query(&[("newest_time", value)]);
        }
        if let Some(ref value) = params.newest_ref {
            request = request.query(&[("newest_ref", value)]);
        }
        if let Some(ref value) = params.oldest_time {
            request = request.query(&[("oldest_time", value)]);
        }
        if let Some(ref value) = params.oldest_ref {
            request = request.query(&[("oldest_ref", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListDeprecatedResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ListDeprecatedErrorBody::Unauthorized(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Retrieve a transaction
    ///
    /// Retrieves the full details of an identified transaction. The transaction resource is identified by a query parameter and *one* of following parameters is required:
    ///
    /// *  `id`
    /// *  `internal_id`
    /// *  `transaction_code`
    /// *  `foreign_transaction_id`
    /// *  `client_transaction_id`
    pub async fn get(
        &self,
        merchant_code: impl Into<String>,
        params: GetParams,
    ) -> crate::error::SdkResult<TransactionFull, GetErrorBody> {
        let path = format!("/v2.1/merchants/{}/transactions", merchant_code.into());
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
        if let Some(ref value) = params.id {
            request = request.query(&[("id", value)]);
        }
        if let Some(ref value) = params.internal_id {
            request = request.query(&[("internal_id", value)]);
        }
        if let Some(ref value) = params.transaction_code {
            request = request.query(&[("transaction_code", value)]);
        }
        if let Some(ref value) = params.foreign_transaction_id {
            request = request.query(&[("foreign_transaction_id", value)]);
        }
        if let Some(ref value) = params.client_transaction_id {
            request = request.query(&[("client_transaction_id", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: TransactionFull = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(GetErrorBody::Unauthorized(
                    body,
                )))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(GetErrorBody::NotFound(body)))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// List transactions
    ///
    /// Lists detailed history of all transactions associated with the merchant profile.
    pub async fn list(
        &self,
        merchant_code: impl Into<String>,
        params: ListParams,
    ) -> crate::error::SdkResult<ListResponse, ListErrorBody> {
        let path = format!(
            "/v2.1/merchants/{}/transactions/history",
            merchant_code.into()
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
        if let Some(ref value) = params.transaction_code {
            request = request.query(&[("transaction_code", value)]);
        }
        if let Some(ref value) = params.order {
            request = request.query(&[("order", value)]);
        }
        if let Some(ref value) = params.limit {
            request = request.query(&[("limit", value)]);
        }
        if let Some(ref value) = params.users {
            request = request.query(&[("users", value)]);
        }
        if let Some(ref value) = params.statuses {
            request = request.query(&[("statuses", value)]);
        }
        if let Some(ref value) = params.payment_types {
            request = request.query(&[("payment_types", value)]);
        }
        if let Some(ref value) = params.entry_modes {
            request = request.query(&[("entry_modes[]", value)]);
        }
        if let Some(ref value) = params.types {
            request = request.query(&[("types", value)]);
        }
        if let Some(ref value) = params.changes_since {
            request = request.query(&[("changes_since", value)]);
        }
        if let Some(ref value) = params.newest_time {
            request = request.query(&[("newest_time", value)]);
        }
        if let Some(ref value) = params.newest_ref {
            request = request.query(&[("newest_ref", value)]);
        }
        if let Some(ref value) = params.oldest_time {
            request = request.query(&[("oldest_time", value)]);
        }
        if let Some(ref value) = params.oldest_ref {
            request = request.query(&[("oldest_ref", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
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
