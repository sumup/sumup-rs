// The contents of this file are generated; do not modify them.

//! Checkouts represent online payment sessions that you create before attempting to charge a payer. A checkout captures the payment intent, such as the amount, currency, merchant, and optional customer or redirect settings, and then moves through its lifecycle as you process it.
//!
//! Use this tag to:
//! - create a checkout before collecting or confirming payment details
//! - process the checkout with a card, saved card, wallet, or supported alternative payment method
//! - retrieve or list checkouts to inspect their current state and associated payment attempts
//! - deactivate a checkout that should no longer be used
//!
//! Typical workflow:
//! - create a checkout with the order amount, currency, and merchant information
//! - process the checkout through SumUp client tools such as the [Payment Widget and Swift Checkout SDK](https://developer.sumup.com/online-payments/checkouts)
//! - retrieve the checkout or use the Transactions endpoints to inspect the resulting payment record
//!
//! Checkouts are used to initiate and orchestrate online payments. Transactions remain the authoritative record of the resulting payment outcome.
use super::common::*;
/// __Required when payment type is `card`.__ Details of the payment card.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// Name of the cardholder as it appears on the payment card.
    ///
    /// Constraints:
    /// - write-only
    ///
    /// Example: `FIRSTNAME LASTNAME`
    pub name: String,
    /// Number of the payment card (without spaces).
    ///
    /// Constraints:
    /// - write-only
    ///
    /// Example: `1234567890123456`
    pub number: String,
    /// Year from the expiration time of the payment card. Accepted formats are `YY` and `YYYY`.
    ///
    /// Constraints:
    /// - write-only
    /// - min length: 2
    /// - max length: 4
    ///
    /// Example: `2023`
    pub expiry_year: String,
    /// Month from the expiration time of the payment card. Accepted format is `MM`.
    ///
    /// Constraints:
    /// - write-only
    pub expiry_month: CardExpiryMonth,
    /// Three or four-digit card verification value (security code) of the payment card.
    ///
    /// Constraints:
    /// - write-only
    /// - min length: 3
    /// - max length: 4
    ///
    /// Example: `123`
    pub cvv: String,
    /// Required five-digit ZIP code. Applicable only to merchant users in the USA.
    ///
    /// Constraints:
    /// - write-only
    /// - min length: 5
    /// - max length: 5
    ///
    /// Example: `12345`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(rename = "type")]
    pub r#type: CardType,
}
/// Core checkout resource returned by the Checkouts API. A checkout is created before payment processing and then updated as payment attempts, redirects, and resulting transactions are attached to it.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Checkout {
    /// Merchant-defined reference for the checkout. Use it to correlate the SumUp checkout with your own order, cart, subscription, or payment attempt in your systems.
    ///
    /// Constraints:
    /// - max length: 90
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Merchant account that receives the payment.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Short merchant-defined description shown in SumUp tools and reporting. Use it to make the checkout easier to recognize in dashboards, support workflows, and reconciliation.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional backend callback URL used by SumUp to notify your platform about processing updates for the checkout.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Unique SumUp identifier of the checkout resource.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `4e425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
    ///
    /// Example: `PENDING`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutStatus>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
    /// Merchant-scoped identifier of the customer associated with the checkout. Use it when storing payment instruments or reusing saved customer context for recurring and returning-payer flows.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// URL of the SumUp-hosted payment page that handles the payment flow. Returned when Hosted Checkout is enabled for the checkout.
    ///
    /// Constraints:
    /// - read-only
    /// - format: `uri`
    ///
    /// Example: `https://checkout.sumup.com/pay/8f9316a3-cda9-42a9-9771-54d534315676`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout_url: Option<String>,
    /// Payment attempts and resulting transaction records linked to this checkout. Use the Transactions endpoints when you need the authoritative payment result and event history.
    ///
    /// Constraints:
    /// - items must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CheckoutTransactionsItem>>,
}
/// Response returned when checkout processing requires an additional payer action, such as a 3DS challenge or a redirect to an external payment method page.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAccepted {
    /// Instructions for the next action the payer or client must take.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<CheckoutAcceptedNextStep>,
}
/// Request body for creating a checkout before processing payment. Define the payment amount, currency, merchant, and optional customer or redirect behavior here.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutCreateRequest {
    /// Merchant-defined reference for the new checkout. It should be unique enough for you to identify the payment attempt in your own systems.
    ///
    /// Constraints:
    /// - max length: 90
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    pub checkout_reference: String,
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    pub amount: f32,
    pub currency: Currency,
    /// Merchant account that should receive the payment.
    ///
    /// Example: `MH4H92C7`
    pub merchant_code: String,
    /// Short merchant-defined description shown in SumUp tools and reporting for easier identification of the checkout.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional backend callback URL used by SumUp to notify your platform about processing updates for the checkout.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com/`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Merchant-scoped customer identifier. Required when setting up recurring payments and useful when the checkout should be linked to a returning payer.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Business purpose of the checkout. Use `CHECKOUT` for a standard payment and `SETUP_RECURRING_PAYMENT` when collecting consent and payment details for future recurring charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CheckoutCreateRequestPurpose>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
    /// URL where the payer should be sent after a redirect-based payment or SCA flow completes. This is required for [APMs](https://developer.sumup.com/online-payments/apm/introduction) and recommended for card checkouts that may require [3DS](https://developer.sumup.com/online-payments/features/3ds). If it is omitted, the [Payment Widget](https://developer.sumup.com/online-payments/checkouts) can render the challenge in an iframe instead of using a full-page redirect.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout: Option<HostedCheckout>,
}
/// Checkout resource returned after a synchronous processing attempt. In addition to the base checkout fields, it can include the resulting transaction identifiers and any newly created payment instrument token.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccess {
    /// Merchant-defined reference for the checkout. Use it to correlate the SumUp checkout with your own order, cart, subscription, or payment attempt in your systems.
    ///
    /// Constraints:
    /// - max length: 90
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Merchant account that receives the payment.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Short merchant-defined description shown in SumUp tools and reporting. Use it to make the checkout easier to recognize in dashboards, support workflows, and reconciliation.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional backend callback URL used by SumUp to notify your platform about processing updates for the checkout.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Unique SumUp identifier of the checkout resource.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `4e425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
    ///
    /// Example: `PENDING`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutSuccessStatus>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
    /// Merchant-scoped identifier of the customer associated with the checkout. Use it when storing payment instruments or reusing saved customer context for recurring and returning-payer flows.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// URL of the SumUp-hosted payment page that handles the payment flow. Returned when Hosted Checkout is enabled for the checkout.
    ///
    /// Constraints:
    /// - read-only
    /// - format: `uri`
    ///
    /// Example: `https://checkout.sumup.com/pay/8f9316a3-cda9-42a9-9771-54d534315676`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout_url: Option<String>,
    /// Payment attempts and resulting transaction records linked to this checkout. Use the Transactions endpoints when you need the authoritative payment result and event history.
    ///
    /// Constraints:
    /// - items must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CheckoutSuccessTransactionsItem>>,
    /// Transaction code of the successful transaction with which the payment for the checkout is completed.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Transaction ID of the successful transaction with which the payment for the checkout is completed.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `410fc44a-5956-44e1-b5cc-19c6f8d727a4`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Name of the merchant
    ///
    /// Example: `Sample Merchant`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// URL where the payer is redirected after a redirect-based payment or SCA flow completes.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Details of the saved payment instrument created or reused during checkout processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<CheckoutSuccessPaymentInstrument>,
}
/// Error message structure.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DetailsError {
    /// Short title of the error.
    ///
    /// Example: `Bad Request`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Details of the error.
    ///
    /// Example: `One or more of the parameters are invalid.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The status code.
    ///
    /// Example: `400`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<f64>,
    /// List of violated validation constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_constraints: Option<Vec<DetailsErrorFailedConstraintsItem>>,
}
impl std::fmt::Display for DetailsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.title, &self.details) {
            (Some(title), Some(details)) => write!(f, "{}: {}", title, details),
            (Some(title), None) => write!(f, "{}", title),
            (None, Some(details)) => write!(f, "{}", details),
            (None, None) => write!(f, "{:?}", self),
        }
    }
}
impl std::error::Error for DetailsError {}
/// Hosted Checkout configuration. Enable it to receive a SumUp-hosted payment page URL in the checkout response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HostedCheckout {
    /// Whether the checkout should include a SumUp-hosted payment page.
    ///
    /// Example: `true`
    pub enabled: bool,
}
/// Mandate details used when a checkout should create a reusable card token for future recurring or merchant-initiated payments.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MandatePayload {
    /// Type of mandate to create for the saved payment instrument.
    ///
    /// Example: `recurrent`
    #[serde(rename = "type")]
    pub r#type: MandatePayloadType,
    /// Browser or client user agent observed when consent was collected.
    ///
    /// Example: `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.104 Safari/537.36`
    pub user_agent: String,
    /// IP address of the payer when the mandate was accepted.
    ///
    /// Example: `172.217.169.174`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ip: Option<String>,
}
/// Request body for attempting payment on an existing checkout. The required companion fields depend on the selected `payment_type`, for example card details, saved-card data, or payer information required by a specific payment method.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessCheckout {
    /// Payment method used for this processing attempt. It determines which additional request fields are required.
    ///
    /// Example: `card`
    pub payment_type: ProcessCheckoutPaymentType,
    /// Number of installments for deferred payments. Available only to merchant users in Brazil.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 12
    ///
    /// Example: `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandatePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    /// Raw `PaymentData` object received from Google Pay. Send the Google Pay response payload as-is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<serde_json::Value>,
    /// Raw payment token object received from Apple Pay. Send the Apple Pay response payload as-is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<serde_json::Value>,
    /// Saved-card token to use instead of raw card details when processing with a previously stored payment instrument.
    ///
    /// Example: `ba85dfee-c3cf-48a6-84f5-d7d761fbba50`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Customer identifier associated with the saved payment instrument. Required when `token` is provided.
    ///
    /// Example: `MEDKHDTI`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}
/// Month from the expiration time of the payment card. Accepted format is `MM`.
///
/// Constraints:
/// - write-only
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CardExpiryMonth {
    #[serde(rename = "01")]
    _01,
    #[serde(rename = "02")]
    _02,
    #[serde(rename = "03")]
    _03,
    #[serde(rename = "04")]
    _04,
    #[serde(rename = "05")]
    _05,
    #[serde(rename = "06")]
    _06,
    #[serde(rename = "07")]
    _07,
    #[serde(rename = "08")]
    _08,
    #[serde(rename = "09")]
    _09,
    #[serde(rename = "10")]
    _10,
    #[serde(rename = "11")]
    _11,
    #[serde(rename = "12")]
    _12,
    #[serde(untagged)]
    Other(String),
}
/// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
///
/// Example: `PENDING`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CheckoutStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(untagged)]
    Other(String),
}
/// Current status of the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CheckoutTransactionsItemStatus {
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
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutTransactionsItem {
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
    pub status: Option<CheckoutTransactionsItemStatus>,
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
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CheckoutAcceptedNextStepMechanismItem {
    #[serde(rename = "iframe")]
    Iframe,
    #[serde(rename = "browser")]
    Browser,
    #[serde(untagged)]
    Other(String),
}
/// Parameters required to complete the next step. The exact keys depend on the payment provider and flow type.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcceptedNextStepPayload {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub additional_properties: std::collections::HashMap<String, String>,
}
/// Instructions for the next action the payer or client must take.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcceptedNextStep {
    /// URL to open or submit in order to continue processing.
    ///
    /// Example: `https://dummy-3ds-gateway.com/cap?RID=1233&VAA=A`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// HTTP method to use when following the next step.
    ///
    /// Example: `POST`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Merchant URL where the payer returns after the external flow finishes.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Allowed presentation mechanisms for the next step. `iframe` means the flow can be embedded, while `browser` means it can be completed through a full-page redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<Vec<CheckoutAcceptedNextStepMechanismItem>>,
    /// Parameters required to complete the next step. The exact keys depend on the payment provider and flow type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<CheckoutAcceptedNextStepPayload>,
}
/// Business purpose of the checkout. Use `CHECKOUT` for a standard payment and `SETUP_RECURRING_PAYMENT` when collecting consent and payment details for future recurring charges.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CheckoutCreateRequestPurpose {
    #[serde(rename = "CHECKOUT")]
    Checkout,
    #[serde(rename = "SETUP_RECURRING_PAYMENT")]
    SetupRecurringPayment,
    #[serde(untagged)]
    Other(String),
}
/// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
///
/// Example: `PENDING`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CheckoutSuccessStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(untagged)]
    Other(String),
}
/// Current status of the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CheckoutSuccessTransactionsItemStatus {
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
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccessTransactionsItem {
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
    pub status: Option<CheckoutSuccessTransactionsItemStatus>,
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
}
/// Details of the saved payment instrument created or reused during checkout processing.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccessPaymentInstrument {
    /// Token value
    ///
    /// Example: `e76d7e5c-9375-4fac-a7e7-b19dc5302fbc`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DetailsErrorFailedConstraintsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
/// Type of mandate to create for the saved payment instrument.
///
/// Example: `recurrent`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MandatePayloadType {
    #[serde(rename = "recurrent")]
    Recurrent,
    #[serde(untagged)]
    Other(String),
}
/// Payment method used for this processing attempt. It determines which additional request fields are required.
///
/// Example: `card`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ProcessCheckoutPaymentType {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "boleto")]
    Boleto,
    #[serde(rename = "ideal")]
    Ideal,
    #[serde(rename = "blik")]
    Blik,
    #[serde(rename = "bancontact")]
    Bancontact,
    #[serde(rename = "google_pay")]
    GooglePay,
    #[serde(rename = "apple_pay")]
    ApplePay,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListAvailablePaymentMethodsResponseAvailablePaymentMethodsItem {
    /// The ID of the payment method.
    ///
    /// Example: `qr_code_pix`
    pub id: String,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListParams {
    /// Filters the list of checkout resources by the unique ID of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
}
/// Returns a list of checkout resources.
pub type ListResponse = Vec<CheckoutSuccess>;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProcessResponse {
    Status200(CheckoutSuccess),
    Status202(CheckoutAccepted),
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListAvailablePaymentMethodsParams {
    /// The amount for which the payment methods should be eligible, in major units.
    ///
    /// Example: `9.99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The currency for which the payment methods should be eligible.
    ///
    /// Example: `EUR`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
/// Available payment methods
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListAvailablePaymentMethodsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payment_methods:
        Option<Vec<ListAvailablePaymentMethodsResponseAvailablePaymentMethodsItem>>,
}
/// The data needed to create an apple pay session for a checkout.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateApplePaySessionBody {
    /// the context to create this apple pay session.
    ///
    /// Constraints:
    /// - format: `hostname`
    ///
    /// Example: `example.com`
    pub context: String,
    /// The target url to create this apple pay session.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `https://apple-pay-gateway-cert.apple.com/paymentservices/startSession`
    pub target: String,
}
/// Successful request. Returns the Apple Pay merchant session object
/// that should be forwarded to the Apple Pay JS SDK to complete merchant
/// validation and continue the payment flow.
pub type CreateApplePaySessionResponse = serde_json::Value;
use crate::client::Client;
#[derive(Debug)]
pub enum ListErrorBody {
    Unauthorized(Problem),
}
#[derive(Debug)]
pub enum CreateErrorBody {
    BadRequest(ErrorExtended),
    Unauthorized(Problem),
    Forbidden(ErrorForbidden),
    Conflict(Error),
}
#[derive(Debug)]
pub enum DeactivateErrorBody {
    Unauthorized(Problem),
    NotFound(Error),
    Conflict(Error),
}
#[derive(Debug)]
pub enum GetErrorBody {
    Unauthorized(Problem),
    NotFound(Error),
}
#[derive(Debug)]
pub enum ProcessErrorBody {
    BadRequest(crate::error::UnknownApiBody),
    Unauthorized(Problem),
    NotFound(Error),
    Conflict(Error),
}
#[derive(Debug)]
pub enum ListAvailablePaymentMethodsErrorBody {
    BadRequest(DetailsError),
}
#[derive(Debug)]
pub enum CreateApplePaySessionErrorBody {
    BadRequest(crate::error::UnknownApiBody),
    NotFound(Error),
}
/// Client for the Checkouts API endpoints.
#[derive(Debug)]
pub struct CheckoutsClient<'a> {
    client: &'a Client,
}
impl<'a> CheckoutsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// List checkouts
    ///
    /// Lists created checkout resources according to the applied `checkout_reference`.
    ///
    /// Responses:
    /// - 200: Returns a list of checkout resources.
    /// - 401: The request is not authorized.
    pub async fn list(
        &self,
        params: ListParams,
    ) -> crate::error::SdkResult<ListResponse, ListErrorBody> {
        let path = "/v0.1/checkouts";
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
        if let Some(ref value) = params.checkout_reference {
            request = request.query(&[("checkout_reference", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListResponse = response.json().await?;
                Ok(data)
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
    /// Create a checkout
    ///
    /// Creates a new payment checkout resource. The unique `checkout_reference` created by this request, is used for further manipulation of the checkout.
    ///
    /// For 3DS checkouts, add the `redirect_url` parameter to your request body schema.
    /// To use the [Hosted Checkout](https://developer.sumup.com/online-payments/checkouts/hosted-checkout/) page, set the `hosted_checkout.enabled` to `true`.
    ///
    /// Follow by processing a checkout to charge the provided payment instrument.
    ///
    /// Responses:
    /// - 201: Returns the created checkout resource.
    /// - 400: The request body is invalid.
    /// - 401: The request is not authorized.
    /// - 403: The request isn't sufficiently authorized to create a checkout.
    /// - 409: A checkout already exists for the provided unique parameters.
    pub async fn create(
        &self,
        body: CheckoutCreateRequest,
    ) -> crate::error::SdkResult<Checkout, CreateErrorBody> {
        let path = "/v0.1/checkouts";
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
                let data: Checkout = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: ErrorExtended = response.json().await?;
                Err(crate::error::SdkError::api(CreateErrorBody::BadRequest(
                    body,
                )))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(CreateErrorBody::Unauthorized(
                    body,
                )))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let body: ErrorForbidden = response.json().await?;
                Err(crate::error::SdkError::api(CreateErrorBody::Forbidden(
                    body,
                )))
            }
            reqwest::StatusCode::CONFLICT => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(CreateErrorBody::Conflict(body)))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Deactivate a checkout
    ///
    /// Deactivates an identified checkout resource. If the checkout has already been processed it can not be deactivated.
    ///
    /// Responses:
    /// - 200: Returns the checkout object after successful deactivation.
    /// - 401: The request is not authorized.
    /// - 404: The requested resource does not exist.
    /// - 409: The request conflicts with the current state of the resource.
    pub async fn deactivate(
        &self,
        id: impl Into<String>,
    ) -> crate::error::SdkResult<Checkout, DeactivateErrorBody> {
        let path = format!("/v0.1/checkouts/{}", id.into());
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
            reqwest::StatusCode::OK => {
                let data: Checkout = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(
                    DeactivateErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(DeactivateErrorBody::NotFound(
                    body,
                )))
            }
            reqwest::StatusCode::CONFLICT => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(DeactivateErrorBody::Conflict(
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
    /// Retrieve a checkout
    ///
    /// Retrieves an identified checkout resource. Use this request after processing a checkout to confirm its status and inform the end user respectively.
    ///
    /// Responses:
    /// - 200: Returns the requested checkout resource.
    /// - 401: The request is not authorized.
    /// - 404: The requested resource does not exist.
    pub async fn get(
        &self,
        id: impl Into<String>,
    ) -> crate::error::SdkResult<CheckoutSuccess, GetErrorBody> {
        let path = format!("/v0.1/checkouts/{}", id.into());
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
                let data: CheckoutSuccess = response.json().await?;
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
    /// Process a checkout
    ///
    /// Processing a checkout will attempt to charge the provided payment instrument for the amount of the specified checkout resource initiated in the `Create a checkout` endpoint.
    ///
    /// Follow this request with `Retrieve a checkout` to confirm its status.
    ///
    /// Responses:
    /// - 200: Returns the checkout resource after a processing attempt.
    /// - 202: Returns the next required action for asynchronous checkout processing.
    /// - 400: The request body is invalid for processing the checkout.
    /// - 401: The request is not authorized.
    /// - 404: The requested resource does not exist.
    /// - 409: The request conflicts with the current state of the resource.
    pub async fn process(
        &self,
        id: impl Into<String>,
        body: ProcessCheckout,
    ) -> crate::error::SdkResult<ProcessResponse, ProcessErrorBody> {
        let path = format!("/v0.1/checkouts/{}", id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .put(&url)
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
                let data: CheckoutSuccess = response.json().await?;
                Ok(ProcessResponse::Status200(data))
            }
            reqwest::StatusCode::ACCEPTED => {
                let data: CheckoutAccepted = response.json().await?;
                Ok(ProcessResponse::Status202(data))
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::api(ProcessErrorBody::BadRequest(
                    body,
                )))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Problem = response.json().await?;
                Err(crate::error::SdkError::api(ProcessErrorBody::Unauthorized(
                    body,
                )))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(ProcessErrorBody::NotFound(
                    body,
                )))
            }
            reqwest::StatusCode::CONFLICT => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(ProcessErrorBody::Conflict(
                    body,
                )))
            }
            _ => {
                let body = response.text().await?;
                let body = crate::error::UnknownApiBody::from_text(body);
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Get available payment methods
    ///
    /// Get payment methods available for the given merchant to use with a checkout.
    ///
    /// Responses:
    /// - 200: Available payment methods
    /// - 400: The request is invalid for the submitted query parameters.
    pub async fn list_available_payment_methods(
        &self,
        merchant_code: impl Into<String>,
        params: ListAvailablePaymentMethodsParams,
    ) -> crate::error::SdkResult<
        ListAvailablePaymentMethodsResponse,
        ListAvailablePaymentMethodsErrorBody,
    > {
        let path = format!("/v0.1/merchants/{}/payment-methods", merchant_code.into());
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
        if let Some(ref value) = params.amount {
            request = request.query(&[("amount", value)]);
        }
        if let Some(ref value) = params.currency {
            request = request.query(&[("currency", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListAvailablePaymentMethodsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: DetailsError = response.json().await?;
                Err(crate::error::SdkError::api(
                    ListAvailablePaymentMethodsErrorBody::BadRequest(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Create an Apple Pay session
    ///
    /// Creates an Apple Pay merchant session for the specified checkout.
    ///
    /// Use this endpoint after the customer selects Apple Pay and before calling
    /// `ApplePaySession.completeMerchantValidation(...)` in the browser.
    /// SumUp validates the merchant session request and returns the Apple Pay
    /// session object that your frontend should pass to Apple's JavaScript API.
    ///
    /// Responses:
    /// - 200: Successful request. Returns the Apple Pay merchant session object
    /// that should be forwarded to the Apple Pay JS SDK to complete merchant
    /// validation and continue the payment flow.
    /// - 400: Bad Request
    /// - 404: The requested resource does not exist.
    pub async fn create_apple_pay_session(
        &self,
        id: impl Into<String>,
        body: Option<CreateApplePaySessionBody>,
    ) -> crate::error::SdkResult<CreateApplePaySessionResponse, CreateApplePaySessionErrorBody>
    {
        let path = format!("/v0.2/checkouts/{}/apple-pay-session", id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .put(&url)
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
            reqwest::StatusCode::OK => {
                let data: CreateApplePaySessionResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::api(
                    CreateApplePaySessionErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateApplePaySessionErrorBody::NotFound(body),
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
