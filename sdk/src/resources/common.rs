// The contents of this file are generated; do not modify them.

/// Profile's personal address information.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AddressLegacy {
    /// City name from the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two letter country code formatted according to [ISO3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// First line of the address with details of the street name and number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    /// Second line of the address with details of the building, unit, apartment, and floor numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    /// Postal code from the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State name or abbreviation from the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
pub type AmountEvent = f64;
/// Object attributes that are modifiable only by SumUp applications.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Attributes {}
/// Three-letter [ISO4217](https://en.wikipedia.org/wiki/ISO_4217) code of the currency for the amount. Currently supported currency values are enumerated above.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Currency {
    #[serde(rename = "BGN")]
    Bgn,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "CLP")]
    Clp,
    #[serde(rename = "CZK")]
    Czk,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "HRK")]
    Hrk,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "RON")]
    Ron,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "USD")]
    Usd,
}
/// Error message structure.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Error {
    /// Short description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Platform code for the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}", message)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
impl std::error::Error for Error {}
/// Error message for forbidden requests.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErrorForbidden {
    /// Short description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Platform code for the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// HTTP status code for the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}
impl std::fmt::Display for ErrorForbidden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error_message) = &self.error_message {
            write!(f, "{}", error_message)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
impl std::error::Error for ErrorForbidden {}
pub type EventId = i64;
/// Status of the transaction event.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EventStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "PAID_OUT")]
    PaidOut,
}
/// Type of the transaction event.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EventType {
    #[serde(rename = "PAYOUT")]
    Payout,
    #[serde(rename = "CHARGE_BACK")]
    ChargeBack,
    #[serde(rename = "REFUND")]
    Refund,
    #[serde(rename = "PAYOUT_DEDUCTION")]
    PayoutDeduction,
}
/// Pending invitation for membership.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Invite {
    /// Email address of the invited user.
    pub email: String,
    pub expires_at: crate::datetime::DateTime,
}
/// Created mandate
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MandateResponse {
    /// Indicates the mandate type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Mandate status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Merchant code which has the mandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
}
/// The status of the membership.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MembershipStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "unknown")]
    Unknown,
}
/// A set of key-value pairs that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
/// **Warning**: Updating Meta will overwrite the existing data. Make sure to always include the complete JSON object.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Meta {}
/// Set of user-defined key-value pairs attached to the object. Partial updates are not supported. When updating, always submit whole metadata.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Metadata {}
/// Personal details for the customer.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PersonalDetails {
    /// First name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Date of birth of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// An identification number user for tax purposes (e.g. CPF)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressLegacy>,
}
pub type TimestampEvent = String;
pub type TransactionId = String;
/// Details of the transaction.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionMixinBase {
    /// Unique ID of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction code returned by the acquirer/processing entity after processing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Total amount of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Date and time of the creation of the transaction. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Current status of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Payment type used for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// Current number of the installment for deferred payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionMixinCheckout {
    /// Unique code of the registered merchant to whom the payment is made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Amount of the applicable VAT (out of the total transaction amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f64>,
    /// Amount of the tip (out of the total transaction amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f64>,
    /// Entry mode of the payment details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<String>,
    /// Authorization code for the transaction sent by the payment card issuer or bank. Applicable only to card payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// Internal unique ID of the transaction on the SumUp platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<i64>,
}
