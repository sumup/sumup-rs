// The contents of this file are generated; do not modify them.

//! Transactions represent completed or attempted payment operations processed for a merchant account. A transaction contains the core payment result, such as the amount, currency, payment method, creation time, and current high-level status.
//!
//! In addition to the main payment outcome, a transaction can contain related events that describe what happened after the original payment attempt. These events provide visibility into the financial lifecycle of the transaction, for example:
//! - `PAYOUT`: the payment being prepared for payout or included in a payout to the merchant
//! - `REFUND`: money returned to the payer
//! - `CHARGE_BACK`: money reversed after the original payment
//! - `PAYOUT_DEDUCTION`: an amount deducted from a payout to cover a refund or chargeback
//!
//! From an integrator's perspective, transactions are the authoritative record of payment outcomes. Use this tag to:
//! - list transactions for reporting, reconciliation, and customer support workflows
//! - retrieve a single transaction when you need the latest payment details
//! - inspect `simple_status` for the current merchant-facing outcome of the payment
//! - inspect `events` or `transaction_events` when you need refund, payout, or chargeback history
//!
//! Typical workflow:
//! - create and process payments through the Checkouts endpoints
//! - use the Transactions endpoints to read the resulting payment records
//! - use the returned statuses and events to update your own order, accounting, or support systems
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
    ///
    /// Example: `3456`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CardType>,
}
/// Details of the device used to create the transaction.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Device {
    /// Device name.
    ///
    /// Example: `m0xx`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Device OS.
    ///
    /// Example: `Android`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_name: Option<String>,
    /// Device model.
    ///
    /// Example: `GT-I9300`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Device OS version.
    ///
    /// Example: `4.3`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_version: Option<String>,
    /// Device UUID.
    ///
    /// Example: `3ae2a6b7-fb0d-3b50-adbf-cb7e2db30cd2`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
/// Details of the ELV card account associated with the transaction.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ElvCardAccount {
    /// ELV card sort code.
    ///
    /// Example: `87096214`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    /// ELV card account number last 4 digits.
    ///
    /// Example: `5674`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    /// ELV card sequence number.
    ///
    /// Example: `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_no: Option<i64>,
    /// ELV IBAN.
    ///
    /// Example: `DE60870962140012345674`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
}
/// High-level transaction event details.
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
    /// Minimum allowed amount for the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<f32>,
    /// Maximum allowed amount for the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f32>,
}
pub type Lon = f32;
/// Purchase product.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Product {
    /// Product name.
    ///
    /// Example: `Purchase reader for merchant with code ME3FCAVF`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_label: Option<String>,
    /// Product price.
    ///
    /// Constraints:
    /// - format: `decimal`
    ///
    /// Example: `100`
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
    ///
    /// Example: `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Quantity x product price.
    ///
    /// Constraints:
    /// - format: `decimal`
    ///
    /// Example: `100`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<f64>,
    /// Total price incl. VAT.
    ///
    /// Constraints:
    /// - format: `decimal`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_with_vat: Option<f64>,
}
/// Detailed information about a transaction event.
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
    ///
    /// Example: `58.8`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Date when the transaction event is due to occur.
    ///
    /// Example: `2020-05-25`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<crate::datetime::Date>,
    /// Date when the transaction event occurred.
    ///
    /// Example: `2020-05-25`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::Date>,
    /// Consecutive number of the installment that is paid. Applicable only payout events, i.e. `event_type = PAYOUT`.
    ///
    /// Example: `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_number: Option<i64>,
    /// Date and time of the transaction event.
    ///
    /// Example: `2020-05-25T10:49:42.784Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
}
/// Full transaction resource with checkout, payout, and event details.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionFull {
    /// Unique ID of the transaction.
    ///
    /// Example: `6b425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction code returned by the acquirer/processing entity after processing the transaction.
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Total amount of the transaction.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Date and time of the creation of the transaction. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    ///
    /// Example: `2020-02-29T10:56:56.876Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Current status of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionFullStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    /// Current number of the installment for deferred payments.
    ///
    /// Constraints:
    /// - value >= 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
    /// Unique code of the registered merchant to whom the payment is made.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Amount of the applicable VAT (out of the total transaction amount).
    ///
    /// Example: `6`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f32>,
    /// Amount of the tip (out of the total transaction amount).
    ///
    /// Example: `3`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<EntryMode>,
    /// Authorization code for the transaction sent by the payment card issuer or bank. Applicable only to card payments.
    ///
    /// Example: `053201`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
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
    pub payout_plan: Option<TransactionFullPayoutPlan>,
    /// External/foreign transaction id (passed by clients).
    ///
    /// Example: `J13253253x1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_transaction_id: Option<String>,
    /// Client transaction id.
    ///
    /// Example: `urn:sumup:pos:sale:MNKKNGST:1D4E3B2D-111D-48D7-9AF0-832DAEF63DD7;2`
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
    ///
    /// Example: `8`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<Lat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<Lon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<HorizontalAccuracy>,
    /// SumUp merchant internal Id.
    ///
    /// Example: `136902`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<Device>,
    /// Simple name of the payment type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_payment_type: Option<TransactionFullSimplePaymentType>,
    /// Verification method used for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<TransactionFullVerificationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elv_account: Option<ElvCardAccount>,
    /// Local date and time of the creation of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<crate::datetime::DateTime>,
    /// The date of the payout.
    ///
    /// Example: `2019-08-28`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_date: Option<crate::datetime::Date>,
    /// Payout type for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<TransactionFullPayoutType>,
    /// Debit/Credit.
    ///
    /// Example: `CREDIT`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_as: Option<TransactionFullProcessAs>,
    /// List of products from the merchant's catalogue for which the transaction serves as a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<Product>>,
    /// List of VAT rates applicable to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rates: Option<Vec<TransactionFullVatRatesItem>>,
    /// Detailed list of events related to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_events: Option<Vec<TransactionEvent>>,
    /// High-level status of the transaction from the merchant's perspective.
    ///
    /// - `PENDING`: The payment has been initiated and is still being processed. A final outcome is not available yet.
    /// - `SUCCESSFUL`: The payment was completed successfully.
    /// - `PAID_OUT`: The payment was completed successfully and the funds have already been included in a payout to the merchant.
    /// - `FAILED`: The payment did not complete successfully.
    /// - `CANCELLED`: The payment was cancelled or reversed and is no longer payable or payable to the merchant.
    /// - `CANCEL_FAILED`: An attempt to cancel or reverse the payment was not completed successfully.
    /// - `REFUNDED`: The payment was refunded in full or in part.
    /// - `REFUND_FAILED`: An attempt to refund the payment was not completed successfully.
    /// - `CHARGEBACK`: The payment was subject to a chargeback.
    /// - `NON_COLLECTION`: The amount could not be collected from the merchant after a chargeback or related adjustment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_status: Option<TransactionFullSimpleStatus>,
    /// List of hyperlinks for accessing related resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// Compact list of events related to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// Details of the payment location as received from the payment terminal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TransactionFullLocation>,
    /// Indicates whether tax deduction is enabled for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_enabled: Option<bool>,
}
/// Transaction entry returned in history listing responses.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionHistory {
    /// Unique ID of the transaction.
    ///
    /// Example: `6b425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction code returned by the acquirer/processing entity after processing the transaction.
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Total amount of the transaction.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Date and time of the creation of the transaction. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    ///
    /// Example: `2020-02-29T10:56:56.876Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Current status of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionHistoryStatus>,
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
    pub payout_plan: Option<TransactionHistoryPayoutPlan>,
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
    pub r#type: Option<TransactionHistoryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<CardType>,
    /// Payout date (if paid out at once).
    ///
    /// Example: `2019-08-28`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_date: Option<crate::datetime::Date>,
    /// Payout type.
    ///
    /// Example: `BANK_ACCOUNT`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<TransactionHistoryPayoutType>,
    /// Total refunded amount.
    ///
    /// Constraints:
    /// - format: `decimal`
    ///
    /// Example: `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<f64>,
}
/// Hypermedia link used for transaction history pagination.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionsHistoryLink {
    /// Relation.
    ///
    /// Example: `next`
    pub rel: String,
    /// Location.
    ///
    /// Example: `limit=10&oldest_ref=090df9bf-93b7-40f1-8181-fbdb236568a1&order=ascending`
    pub href: String,
}
/// Current status of the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionFullStatus {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(untagged)]
    Other(String),
}
/// Payout plan of the registered user at the time when the transaction was made.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionFullPayoutPlan {
    #[serde(rename = "SINGLE_PAYMENT")]
    SinglePayment,
    #[serde(rename = "TRUE_INSTALLMENT")]
    TrueInstallment,
    #[serde(rename = "ACCELERATED_INSTALLMENT")]
    AcceleratedInstallment,
    #[serde(untagged)]
    Other(String),
}
/// Simple name of the payment type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionFullSimplePaymentType {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CC_SIGNATURE")]
    CcSignature,
    #[serde(rename = "ELV")]
    Elv,
    #[serde(rename = "ELV_WITHOUT_SIGNATURE")]
    ElvWithoutSignature,
    #[serde(rename = "CC_CUSTOMER_ENTERED")]
    CcCustomerEntered,
    #[serde(rename = "MANUAL_ENTRY")]
    ManualEntry,
    #[serde(rename = "EMV")]
    Emv,
    #[serde(rename = "RECURRING")]
    Recurring,
    #[serde(rename = "BALANCE")]
    Balance,
    #[serde(rename = "MOTO")]
    Moto,
    #[serde(rename = "BOLETO")]
    Boleto,
    #[serde(rename = "APM")]
    Apm,
    #[serde(rename = "BITCOIN")]
    Bitcoin,
    #[serde(rename = "CARD")]
    Card,
    #[serde(untagged)]
    Other(String),
}
/// Verification method used for the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionFullVerificationMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "signature")]
    Signature,
    #[serde(rename = "offline PIN")]
    OfflinePin,
    #[serde(rename = "online PIN")]
    OnlinePin,
    #[serde(rename = "offline PIN + signature")]
    OfflinePinSignature,
    #[serde(rename = "na")]
    Na,
    #[serde(untagged)]
    Other(String),
}
/// Payout type for the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionFullPayoutType {
    #[serde(rename = "BANK_ACCOUNT")]
    BankAccount,
    #[serde(rename = "PREPAID_CARD")]
    PrepaidCard,
    #[serde(untagged)]
    Other(String),
}
/// Debit/Credit.
///
/// Example: `CREDIT`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionFullProcessAs {
    #[serde(rename = "CREDIT")]
    Credit,
    #[serde(rename = "DEBIT")]
    Debit,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionFullVatRatesItem {
    /// VAT rate.
    ///
    /// Constraints:
    /// - format: `decimal`
    ///
    /// Example: `0.045`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /// NET amount of products having this VAT rate applied.
    ///
    /// Constraints:
    /// - format: `decimal`
    ///
    /// Example: `1.36`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<f64>,
    /// VAT amount of this rate applied.
    ///
    /// Constraints:
    /// - format: `decimal`
    ///
    /// Example: `0.06`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<f64>,
    /// Gross amount of products having this VAT rate applied.
    ///
    /// Constraints:
    /// - format: `decimal`
    ///
    /// Example: `1.42`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross: Option<f64>,
}
/// High-level status of the transaction from the merchant's perspective.
///
/// - `PENDING`: The payment has been initiated and is still being processed. A final outcome is not available yet.
/// - `SUCCESSFUL`: The payment was completed successfully.
/// - `PAID_OUT`: The payment was completed successfully and the funds have already been included in a payout to the merchant.
/// - `FAILED`: The payment did not complete successfully.
/// - `CANCELLED`: The payment was cancelled or reversed and is no longer payable or payable to the merchant.
/// - `CANCEL_FAILED`: An attempt to cancel or reverse the payment was not completed successfully.
/// - `REFUNDED`: The payment was refunded in full or in part.
/// - `REFUND_FAILED`: An attempt to refund the payment was not completed successfully.
/// - `CHARGEBACK`: The payment was subject to a chargeback.
/// - `NON_COLLECTION`: The amount could not be collected from the merchant after a chargeback or related adjustment.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionFullSimpleStatus {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "PAID_OUT")]
    PaidOut,
    #[serde(rename = "CANCEL_FAILED")]
    CancelFailed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "CHARGEBACK")]
    Chargeback,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "REFUND_FAILED")]
    RefundFailed,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "NON_COLLECTION")]
    NonCollection,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(untagged)]
    Other(String),
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
/// Current status of the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionHistoryStatus {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(untagged)]
    Other(String),
}
/// Payout plan of the registered user at the time when the transaction was made.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionHistoryPayoutPlan {
    #[serde(rename = "SINGLE_PAYMENT")]
    SinglePayment,
    #[serde(rename = "TRUE_INSTALLMENT")]
    TrueInstallment,
    #[serde(rename = "ACCELERATED_INSTALLMENT")]
    AcceleratedInstallment,
    #[serde(untagged)]
    Other(String),
}
/// Type of the transaction for the registered user specified in the `user` property.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionHistoryType {
    #[serde(rename = "PAYMENT")]
    Payment,
    #[serde(rename = "REFUND")]
    Refund,
    #[serde(rename = "CHARGE_BACK")]
    ChargeBack,
    #[serde(untagged)]
    Other(String),
}
/// Payout type.
///
/// Example: `BANK_ACCOUNT`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionHistoryPayoutType {
    #[serde(rename = "BANK_ACCOUNT")]
    BankAccount,
    #[serde(rename = "PREPAID_CARD")]
    PrepaidCard,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListDeprecatedParamsOrder {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListDeprecatedParamsStatusesItem {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "CHARGE_BACK")]
    ChargeBack,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListDeprecatedParamsTypesItem {
    #[serde(rename = "PAYMENT")]
    Payment,
    #[serde(rename = "REFUND")]
    Refund,
    #[serde(rename = "CHARGE_BACK")]
    ChargeBack,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListParamsOrder {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListParamsStatusesItem {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "CHARGE_BACK")]
    ChargeBack,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListParamsTypesItem {
    #[serde(rename = "PAYMENT")]
    Payment,
    #[serde(rename = "REFUND")]
    Refund,
    #[serde(rename = "CHARGE_BACK")]
    ChargeBack,
    #[serde(untagged)]
    Other(String),
}
/// Optional amount for partial refunds.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RefundBody {
    /// Amount to be refunded. Eligible amount can't exceed the amount of the transaction and varies based on country and currency. If you do not specify a value, the system performs a full refund of the transaction.
    ///
    /// Example: `5`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetDeprecatedParams {
    /// Retrieves the transaction resource with the specified transaction ID (the `id` parameter in the transaction resource).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    pub order: Option<ListDeprecatedParamsOrder>,
    /// Specifies the maximum number of results per page. Value must be a positive integer and if not specified, will return 10 results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filters the returned results by user email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// Filters the returned results by the specified list of final statuses of the transactions.
    #[serde(rename = "statuses[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<ListDeprecatedParamsStatusesItem>>,
    /// Filters the returned results by the specified list of payment types used for the transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_types: Option<Vec<PaymentType>>,
    /// Filters the returned results by the specified list of transaction types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<ListDeprecatedParamsTypesItem>>,
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
/// Returns a page of transaction history items.
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
    pub order: Option<ListParamsOrder>,
    /// Specifies the maximum number of results per page. Value must be a positive integer and if not specified, will return 10 results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filters the returned results by user email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// Filters the returned results by the specified list of final statuses of the transactions.
    #[serde(rename = "statuses[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<ListParamsStatusesItem>>,
    /// Filters the returned results by the specified list of payment types used for the transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_types: Option<Vec<PaymentType>>,
    /// Filters the returned results by the specified list of entry modes.
    #[serde(rename = "entry_modes[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_modes: Option<Vec<EntryMode>>,
    /// Filters the returned results by the specified list of transaction types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<ListParamsTypesItem>>,
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
/// Returns a page of transaction history items.
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
    Unauthorized(Problem),
    NotFound(Error),
}
#[derive(Debug)]
pub enum ListDeprecatedErrorBody {
    BadRequest(Error),
    Unauthorized(Problem),
}
#[derive(Debug)]
pub enum GetErrorBody {
    Unauthorized(Problem),
    NotFound(Error),
}
#[derive(Debug)]
pub enum ListErrorBody {
    BadRequest(Error),
    Unauthorized(Problem),
}
/// Client for the Transactions API endpoints.
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
    ///
    /// Responses:
    /// - 204: Returns an empty response body when the operation succeeds.
    /// - 404: The requested resource does not exist.
    /// - 409: The transaction cannot be refunded due to business constraints.
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
    /// - `id`
    /// - `transaction_code`
    /// - `foreign_transaction_id`
    /// - `client_transaction_id`
    ///
    /// Responses:
    /// - 200: Returns the requested transaction resource.
    /// - 401: The request is not authorized.
    /// - 404: The requested resource does not exist.
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
                let body: Problem = response.json().await?;
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
    ///
    /// Responses:
    /// - 200: Returns a page of transaction history items.
    /// - 400: The request is invalid for the submitted query parameters.
    /// - 401: The request is not authorized.
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
            request = request.query(&[("statuses[]", value)]);
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
            reqwest::StatusCode::BAD_REQUEST => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ListDeprecatedErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Problem = response.json().await?;
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
    /// - `id`
    /// - `transaction_code`
    /// - `foreign_transaction_id`
    /// - `client_transaction_id`
    ///
    /// Responses:
    /// - 200: Returns the requested transaction resource.
    /// - 401: The request is not authorized.
    /// - 404: The requested resource does not exist.
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
                let body: Problem = response.json().await?;
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
    ///
    /// Responses:
    /// - 200: Returns a page of transaction history items.
    /// - 400: The request is invalid for the submitted query parameters.
    /// - 401: The request is not authorized.
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
            request = request.query(&[("statuses[]", value)]);
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
            reqwest::StatusCode::BAD_REQUEST => {
                let body: Error = response.json().await?;
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
