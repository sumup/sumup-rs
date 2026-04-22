// The contents of this file are generated; do not modify them.

/// Profile's personal address information.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AddressLegacy {
    /// City name from the address.
    ///
    /// Example: `Berlin`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two letter country code formatted according to [ISO3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    ///
    /// Example: `DE`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// First line of the address with details of the street name and number.
    ///
    /// Example: `Sample street`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    /// Second line of the address with details of the building, unit, apartment, and floor numbers.
    ///
    /// Example: `ap. 5`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    /// Postal code from the address.
    ///
    /// Example: `10115`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State name or abbreviation from the address.
    ///
    /// Example: `Berlin`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
/// Object attributes that are modifiable only by SumUp applications.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
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
///
/// Example: `EUR`
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
    #[serde(rename = "TWINT")]
    Twint,
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
/// Error message structure.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Error {
    /// Short description of the error.
    ///
    /// Example: `Resource not found`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Platform code for the error.
    ///
    /// Example: `NOT_FOUND`
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
/// Error payload with the invalid parameter reference.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErrorExtended {
    /// Short description of the error.
    ///
    /// Example: `Resource not found`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Platform code for the error.
    ///
    /// Example: `NOT_FOUND`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Parameter name (with relative location) to which the error applies. Parameters from embedded resources are displayed using dot notation. For example, `card.name` refers to the `name` parameter embedded in the `card` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}
impl std::fmt::Display for ErrorExtended {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}", message)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
impl std::error::Error for ErrorExtended {}
/// Error message for forbidden requests.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErrorForbidden {
    /// Short description of the error.
    ///
    /// Example: `request_not_allowed`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Platform code for the error.
    ///
    /// Example: `FORBIDDEN`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// HTTP status code for the error.
    ///
    /// Example: `403`
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
///
/// Not every value is used for every event type.
///
/// - `PENDING`: The event has been created but is not final yet. Used for events that are still being processed and whose final outcome is not known yet.
/// - `SCHEDULED`: The event is planned for a future payout cycle but has not been executed yet. This applies to payout events before money is actually sent out.
/// - `RECONCILED`: The underlying payment has been matched with settlement data and is ready to continue through payout processing, but the funds have not been paid out yet. This applies to payout events.
/// - `PAID_OUT`: The payout event has been completed and the funds were included in a merchant payout.
/// - `REFUNDED`: A refund event has been accepted and recorded in the refund flow. This is the status returned for refund events once the transaction amount is being or has been returned to the payer.
/// - `SUCCESSFUL`: The event completed successfully. Use this as the generic terminal success status for event types that do not expose a more specific business outcome such as `PAID_OUT` or `REFUNDED`.
/// - `FAILED`: The event could not be completed. Typical examples are a payout that could not be executed or an event that was rejected during processing.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EventStatus {
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PAID_OUT")]
    PaidOut,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "RECONCILED")]
    Reconciled,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "SUCCESSFUL")]
    Successful,
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
    ///
    /// Example: `boaty.mcboatface@sumup.com`
    pub email: String,
    pub expires_at: crate::datetime::DateTime,
}
/// Details of the mandate linked to the saved payment instrument.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MandateResponse {
    /// Type of mandate stored for the checkout or payment instrument.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Current lifecycle status of the mandate.
    ///
    /// Example: `active`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MandateResponseStatus>,
    /// Merchant account for which the mandate is valid.
    ///
    /// Example: `MH4H92C7`
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
pub struct Metadata {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
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
    ///
    /// Example: `John`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the customer.
    ///
    /// Example: `Doe`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email address of the customer.
    ///
    /// Example: `user@example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number of the customer.
    ///
    /// Example: `+491635559723`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Date of birth of the customer.
    ///
    /// Example: `1993-12-31`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<crate::datetime::Date>,
    /// An identification number user for tax purposes (e.g. CPF)
    ///
    /// Constraints:
    /// - max length: 255
    ///
    /// Example: `423.378.593-47`
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
    ///
    /// Example: `https://developer.sumup.com/problem/not-found`
    #[serde(rename = "type")]
    pub r#type: String,
    /// A short, human-readable summary of the problem type.
    ///
    /// Example: `Requested resource couldn't be found.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The HTTP status code generated by the origin server for this occurrence of the problem.
    ///
    /// Example: `404`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// A human-readable explanation specific to this occurrence of the problem.
    ///
    /// Example: `The requested resource doesn't exist or does not belong to you.`
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
    pub status: Option<TransactionBaseStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    /// Current number of the installment for deferred payments.
    ///
    /// Constraints:
    /// - value >= 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
}
/// Checkout-specific fields associated with a transaction.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TransactionCheckoutInfo {
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
}
pub type TransactionId = String;
/// Current lifecycle status of the mandate.
///
/// Example: `active`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MandateResponseStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(untagged)]
    Other(String),
}
/// Current status of the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionBaseStatus {
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
