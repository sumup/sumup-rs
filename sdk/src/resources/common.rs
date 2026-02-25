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
/// Object attributes that are modifiable only by SumUp applications.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Attributes {}
/// Issuing card network of the payment card used for the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CardType {
    #[serde(rename = "ALELO")]
    Alelo,
    #[serde(rename = "AMEX")]
    Amex,
    #[serde(rename = "CONECS")]
    Conecs,
    #[serde(rename = "CUP")]
    Cup,
    #[serde(rename = "DINERS")]
    Diners,
    #[serde(rename = "DISCOVER")]
    Discover,
    #[serde(rename = "EFTPOS")]
    Eftpos,
    #[serde(rename = "ELO")]
    Elo,
    #[serde(rename = "ELV")]
    Elv,
    #[serde(rename = "GIROCARD")]
    Girocard,
    #[serde(rename = "HIPERCARD")]
    Hipercard,
    #[serde(rename = "INTERAC")]
    Interac,
    #[serde(rename = "JCB")]
    Jcb,
    #[serde(rename = "MAESTRO")]
    Maestro,
    #[serde(rename = "MASTERCARD")]
    Mastercard,
    #[serde(rename = "PLUXEE")]
    Pluxee,
    #[serde(rename = "SWILE")]
    Swile,
    #[serde(rename = "TICKET")]
    Ticket,
    #[serde(rename = "VISA")]
    Visa,
    #[serde(rename = "VISA_ELECTRON")]
    VisaElectron,
    #[serde(rename = "VISA_VPAY")]
    VisaVpay,
    #[serde(rename = "VPAY")]
    Vpay,
    #[serde(rename = "VR")]
    Vr,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(untagged)]
    Other(String),
}
/// Three-letter [ISO4217](https://en.wikipedia.org/wiki/ISO_4217) code of the currency for the amount. Currently supported currency values are enumerated above.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Currency {
    BGN,
    BRL,
    CHF,
    CLP,
    COP,
    CZK,
    DKK,
    EUR,
    GBP,
    HRK,
    HUF,
    NOK,
    PLN,
    RON,
    SEK,
    USD,
    #[serde(untagged)]
    Other(String),
}
/// Entry mode of the payment details.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EntryMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "magstripe")]
    Magstripe,
    #[serde(rename = "chip")]
    Chip,
    #[serde(rename = "manual entry")]
    ManualEntry,
    #[serde(rename = "customer entry")]
    CustomerEntry,
    #[serde(rename = "magstripe fallback")]
    MagstripeFallback,
    #[serde(rename = "contactless")]
    Contactless,
    #[serde(rename = "moto")]
    Moto,
    #[serde(rename = "contactless magstripe")]
    ContactlessMagstripe,
    #[serde(rename = "boleto")]
    Boleto,
    #[serde(rename = "direct debit")]
    DirectDebit,
    #[serde(rename = "sofort")]
    Sofort,
    #[serde(rename = "ideal")]
    Ideal,
    #[serde(rename = "bancontact")]
    Bancontact,
    #[serde(rename = "eps")]
    Eps,
    #[serde(rename = "mybank")]
    Mybank,
    #[serde(rename = "satispay")]
    Satispay,
    #[serde(rename = "blik")]
    Blik,
    #[serde(rename = "p24")]
    P24,
    #[serde(rename = "giropay")]
    Giropay,
    #[serde(rename = "pix")]
    Pix,
    #[serde(rename = "qr code pix")]
    QrCodePix,
    #[serde(rename = "apple pay")]
    ApplePay,
    #[serde(rename = "google pay")]
    GooglePay,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "na")]
    Na,
    #[serde(untagged)]
    Other(String),
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
    #[serde(untagged)]
    Other(String),
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
    #[serde(untagged)]
    Other(String),
}
/// Pending invitation for membership.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Invite {
    /// Email address of the invited user.
    ///
    /// Constraints:
    /// - format: `email`
    pub email: String,
    pub expires_at: crate::datetime::DateTime,
}
/// Created mandate
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MandateResponse {
    /// Indicates the mandate type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
    #[serde(untagged)]
    Other(String),
}
/// Set of user-defined key-value pairs attached to the object. Partial updates are not supported. When updating, always submit whole metadata. Maximum of 64 parameters are allowed in the object.
///
/// Constraints:
/// - max properties: 64
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Metadata {}
/// Payment type used for the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PaymentType {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "POS")]
    Pos,
    #[serde(rename = "ECOM")]
    Ecom,
    #[serde(rename = "RECURRING")]
    Recurring,
    #[serde(rename = "BITCOIN")]
    Bitcoin,
    #[serde(rename = "BALANCE")]
    Balance,
    #[serde(rename = "MOTO")]
    Moto,
    #[serde(rename = "BOLETO")]
    Boleto,
    #[serde(rename = "DIRECT_DEBIT")]
    DirectDebit,
    #[serde(rename = "APM")]
    Apm,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(untagged)]
    Other(String),
}
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
    pub birth_date: Option<crate::datetime::Date>,
    /// An identification number user for tax purposes (e.g. CPF)
    ///
    /// Constraints:
    /// - max length: 255
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressLegacy>,
}
/// A RFC 9457 problem details object.
///
/// Additional properties specific to the problem type may be present.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Problem {
    /// A URI reference that identifies the problem type.
    ///
    /// Constraints:
    /// - format: `uri`
    #[serde(rename = "type")]
    pub r#type: String,
    /// A short, human-readable summary of the problem type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The HTTP status code generated by the origin server for this occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// A human-readable explanation specific to this occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A URI reference that identifies the specific occurrence of the problem.
    ///
    /// Constraints:
    /// - format: `uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.title, &self.detail) {
            (Some(title), Some(detail)) => write!(f, "{}: {}", title, detail),
            (Some(title), None) => write!(f, "{}", title),
            (None, Some(detail)) => write!(f, "{}", detail),
            (None, None) => write!(f, "{:?}", self),
        }
    }
}
impl std::error::Error for Problem {}
/// Details of the transaction.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionBase {
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
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionCheckoutInfo {
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
}
pub type TransactionId = String;
