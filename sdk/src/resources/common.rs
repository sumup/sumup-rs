// The contents of this file are generated; do not modify them.

/// Profile's personal address information.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
/// Issuing card network of the payment card used for the transaction.
///
/// Example: `VISA`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardType {
    #[serde(rename = "ALELO")]
    Alelo,
    #[serde(rename = "AMEX")]
    Amex,
    #[serde(rename = "BANCOMAT")]
    Bancomat,
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
    #[serde(rename = "SUMUP")]
    Sumup,
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
/// Three-letter [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code of the amount.
///
/// Example: `EUR`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Currency {
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
/// How the payment details were captured, for example `CHIP` or `CONTACTLESS` for card-present payments and `CUSTOMER_ENTRY` for card details entered by the payer. For wallet and alternative payment methods, this can identify the method, such as `APPLE_PAY` or `BLIK`.
///
/// Example: `CUSTOMER_ENTRY`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
/// Details of an API error.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Error {
    /// Platform code for the error.
    ///
    /// Example: `NOT_FOUND`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Short description of the error.
    ///
    /// Example: `Resource not found`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.message.as_deref() {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "{:?}", self),
        }
    }
}
impl std::error::Error for Error {}
/// Error payload with the invalid parameter reference.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrorExtended {
    /// Platform code for the error.
    ///
    /// Example: `NOT_FOUND`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Short description of the error.
    ///
    /// Example: `Resource not found`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Parameter name (with relative location) to which the error applies. Parameters from embedded resources are displayed using dot notation. For example, `card.name` refers to the `name` parameter embedded in the `card` object.
    ///
    /// Example: `card.name`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}
impl std::fmt::Display for ErrorExtended {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.message.as_deref() {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "{:?}", self),
        }
    }
}
impl std::error::Error for ErrorExtended {}
/// Details of an error returned for a forbidden request.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrorForbidden {
    /// Platform code for the error.
    ///
    /// Example: `FORBIDDEN`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Short description of the error.
    ///
    /// Example: `request_not_allowed`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// HTTP status code for the error.
    ///
    /// Example: `403`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}
impl std::fmt::Display for ErrorForbidden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ErrorForbidden {}
/// Pending invitation for membership.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MandateResponse {
    /// Short unique identifier for the merchant for which the mandate is valid.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Current lifecycle status of the mandate.
    ///
    /// Example: `active`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MandateResponseStatus>,
    /// Type of mandate stored for the checkout or payment instrument.
    ///
    /// Example: `recurrent`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
/// The status of the membership.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
/// Payment category recorded on a transaction, for example `POS` for a point-of-sale card payment, `ECOM` for an online card payment, or `RECURRING` for a recurring card payment. These reporting values are separate from the lowercase `payment_type` values used to process checkouts.
///
/// Example: `ECOM`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PersonalDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressLegacy>,
    /// Date of birth of the customer in `YYYY-MM-DD` format, without a time or timezone.
    ///
    /// Example: `1993-12-31`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<crate::datetime::Date>,
    /// Email address of the customer.
    ///
    /// Example: `user@example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
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
    /// Phone number of the customer.
    ///
    /// Example: `+491635559723`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Identification number used for tax purposes, such as a CPF in Brazil.
    ///
    /// Constraints:
    /// - max length: 255
    ///
    /// Example: `423.378.593-47`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}
/// A RFC 9457 problem details object.
///
/// Additional properties specific to the problem type may be present.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Problem {
    /// A human-readable explanation specific to this occurrence of the problem.
    ///
    /// Example: `The requested resource doesn't exist or does not belong to you.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A URI reference that identifies the specific occurrence of the problem.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `https://api.sumup.com/v0.1/checkouts/4e425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// The HTTP status code generated by the origin server for this occurrence of the problem.
    ///
    /// Example: `404`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// A short, human-readable summary of the problem type.
    ///
    /// Example: `Requested resource couldn't be found.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A URI reference that identifies the problem type.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `https://developer.sumup.com/problem/not-found`
    #[serde(rename = "type")]
    pub r#type: String,
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
/// Core details shared by transaction resources.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionBase {
    /// Total amount of the transaction in major units of `currency`, for example `10.1` for EUR 10.10.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Unique identifier of the transaction.
    ///
    /// Example: `6b425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Number of installments for a deferred payment.
    ///
    /// Constraints:
    /// - value >= 1
    ///
    /// Example: `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatus>,
    /// The timestamp of when the transaction was created.
    ///
    /// Example: `2020-02-29T10:56:56.876Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// SumUp transaction code, for example `TEENSK4W2K`. Use it to look up the transaction with the `transaction_code` query parameter. This is separate from the transaction's `id` and the card issuer's `auth_code`.
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
}
/// Checkout-specific fields associated with a transaction.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionCheckoutInfo {
    /// Authorization code for the transaction sent by the payment card issuer or bank. Applicable only to card payments.
    ///
    /// Example: `053201`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<EntryMode>,
    /// Unique code of the registered merchant to whom the payment is made.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Tip included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `3`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f32>,
    /// VAT included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `6`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f32>,
}
/// Numeric identifier of a transaction event. Use it as `tx_event_id` when requesting receipt details for a specific event. This is separate from the transaction ID and the transaction history pagination references.
///
/// Example: `9567461191`
pub type TransactionEventId = i64;
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
///
/// Example: `SUCCESSFUL`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransactionEventStatus {
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
/// Financial event associated with a transaction.
///
/// - `PAYOUT`: Funds from the transaction being prepared for or included in a merchant payout. Check the event status to determine whether they have been paid out.
/// - `REFUND`: Money returned to the payer.
/// - `CHARGE_BACK`: A reversal of the payment following a chargeback.
/// - `PAYOUT_DEDUCTION`: An amount deducted from a merchant payout, for example to cover a refund or chargeback.
///
/// Example: `REFUND`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransactionEventType {
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
/// Unique identifier of the transaction.
///
/// Example: `410fc44a-5956-44e1-b5cc-19c6f8d727a4`
pub type TransactionId = String;
/// Current status of the transaction.
///
/// - `PENDING`: The transaction has been created but its final outcome is not known yet.
/// - `SUCCESSFUL`: The transaction completed successfully.
/// - `CANCELLED`: The transaction was cancelled or otherwise reversed before completion.
/// - `FAILED`: The transaction attempt did not complete successfully.
/// - `REFUNDED`: The transaction was refunded in full or in part.
///
/// Example: `SUCCESSFUL`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(untagged)]
    Other(String),
}
/// Current lifecycle status of the mandate.
///
/// Example: `active`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MandateResponseStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(untagged)]
    Other(String),
}
