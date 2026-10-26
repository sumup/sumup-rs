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
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum CheckoutCreated {}
impl crate::event::private::Sealed for CheckoutCreated {}
impl crate::events::EventSpec for CheckoutCreated {
    const EVENT_TYPE: &'static str = "checkouts.created";
    type FetchedObject = Checkout;
}
/// Event notification for this event type.
pub type CheckoutCreatedEvent = crate::events::Event<CheckoutCreated>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum CheckoutExpired {}
impl crate::event::private::Sealed for CheckoutExpired {}
impl crate::events::EventSpec for CheckoutExpired {
    const EVENT_TYPE: &'static str = "checkouts.expired";
    type FetchedObject = Checkout;
}
/// Event notification for this event type.
pub type CheckoutExpiredEvent = crate::events::Event<CheckoutExpired>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum CheckoutFailed {}
impl crate::event::private::Sealed for CheckoutFailed {}
impl crate::events::EventSpec for CheckoutFailed {
    const EVENT_TYPE: &'static str = "checkouts.failed";
    type FetchedObject = Checkout;
}
/// Event notification for this event type.
pub type CheckoutFailedEvent = crate::events::Event<CheckoutFailed>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum CheckoutPaid {}
impl crate::event::private::Sealed for CheckoutPaid {}
impl crate::events::EventSpec for CheckoutPaid {
    const EVENT_TYPE: &'static str = "checkouts.paid";
    type FetchedObject = Checkout;
}
/// Event notification for this event type.
pub type CheckoutPaidEvent = crate::events::Event<CheckoutPaid>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum CheckoutUpdated {}
impl crate::event::private::Sealed for CheckoutUpdated {}
impl crate::events::EventSpec for CheckoutUpdated {
    const EVENT_TYPE: &'static str = "checkouts.updated";
    type FetchedObject = Checkout;
}
/// Event notification for this event type.
pub type CheckoutUpdatedEvent = crate::events::Event<CheckoutUpdated>;
/// __Required when payment type is `card`.__ Details of the payment card.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// Three or four-digit card verification value (security code) of the payment card.
    ///
    /// Constraints:
    /// - write-only
    /// - min length: 3
    /// - max length: 4
    ///
    /// Example: `123`
    pub cvv: String,
    /// Two-digit expiration month, from `01` through `12`.
    ///
    /// Constraints:
    /// - write-only
    /// - pattern: `^(0[1-9]|1[0-2])$`
    /// - min length: 2
    /// - max length: 2
    ///
    /// Example: `12`
    pub expiry_month: String,
    /// Two- or four-digit expiration year in `YY` or `YYYY` format.
    ///
    /// Constraints:
    /// - write-only
    /// - pattern: `^[0-9]{2}([0-9]{2})?$`
    ///
    /// Example: `2030`
    pub expiry_year: String,
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
    #[serde(rename = "type")]
    pub r#type: CardType,
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
}
/// Core checkout resource returned by the Checkouts API. A checkout is created before payment processing and then updated as payment attempts, redirects, and resulting transactions are attached to it.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Checkout {
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Merchant-defined reference for the checkout. Use it to correlate the SumUp checkout with your own order, cart, subscription, or payment attempt in your systems.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Merchant-scoped identifier of the customer associated with the checkout. Use it when storing payment instruments or reusing saved customer context for recurring and returning-payer flows.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// The timestamp of when the checkout was created.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Short merchant-defined description shown in SumUp tools and reporting. Use it to make the checkout easier to recognize in dashboards, support workflows, and reconciliation.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of the SumUp-hosted payment page that handles the payment flow. Returned when Hosted Checkout is enabled for the checkout.
    ///
    /// Constraints:
    /// - read-only
    /// - format: `uri`
    ///
    /// Example: `https://checkout.sumup.com/pay/8f9316a3-cda9-42a9-9771-54d534315676`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout_url: Option<String>,
    /// Unique SumUp identifier of the checkout resource.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `4e425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// Short unique identifier for the merchant that receives the payment.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Optional backend callback URL for checkout status notifications. SumUp sends an HTTP POST with `event_type` and the checkout `id`. Retrieve the checkout to verify its current status before updating your order. See the [webhook guide](https://developer.sumup.com/online-payments/webhooks/) for the payload and response requirements.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
    ///
    /// Example: `PENDING`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutStatus>,
    /// Payment attempts and resulting transaction records linked to this checkout. Use the Transactions endpoints when you need the authoritative payment result and event history.
    ///
    /// Constraints:
    /// - items must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CheckoutTransactionsItem>>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2030-12-31T23:59:59Z`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
}
/// Response returned when checkout processing requires an additional payer action, such as a 3DS challenge or a redirect to an external payment method page.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAccepted {
    /// Instructions for the next action the payer or client must take.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<CheckoutAcceptedNextStep>,
}
/// Request body for creating a checkout before processing payment. Define the payment amount, currency, merchant, and optional customer or redirect behavior here.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutCreateRequest {
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    pub amount: f32,
    /// Merchant-defined reference for the new checkout, up to 64 characters. Use it to correlate the checkout with an order or payment attempt in your own system. If a checkout already exists for the supplied unique parameters, creation returns `409` with `DUPLICATED_CHECKOUT`; see the conflict response.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    pub checkout_reference: String,
    pub currency: Currency,
    /// Merchant-scoped customer identifier. Required when setting up recurring payments and useful when the checkout should be linked to a returning payer.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Short merchant-defined description shown in SumUp tools and reporting for easier identification of the checkout.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout: Option<HostedCheckout>,
    /// Short unique identifier for the merchant that should receive the payment.
    ///
    /// Example: `MH4H92C7`
    pub merchant_code: String,
    /// Business purpose of the checkout. Use `CHECKOUT` for a standard payment and `SETUP_RECURRING_PAYMENT` when collecting consent and payment details for future recurring charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CheckoutCreateRequestPurpose>,
    /// URL where the payer should be sent after a redirect-based payment or SCA flow completes. This is required for [APMs](https://developer.sumup.com/online-payments/apm/introduction) and recommended for card checkouts that may require [3DS](https://developer.sumup.com/online-payments/features/3ds). If it is omitted, the [Payment Widget](https://developer.sumup.com/online-payments/checkouts) can render the challenge in an iframe instead of using a full-page redirect.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Optional backend callback URL for checkout status notifications. SumUp sends an HTTP POST with `event_type` and the checkout `id`. Retrieve the checkout to verify its current status before updating your order. See the [webhook guide](https://developer.sumup.com/online-payments/webhooks/) for the payload and response requirements.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com/`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2030-12-31T23:59:59Z`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
}
/// Checkout resource returned after a synchronous processing attempt. In addition to the base checkout fields, it can include the resulting transaction identifiers and any newly created payment instrument token.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccess {
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Merchant-defined reference for the checkout. Use it to correlate the SumUp checkout with your own order, cart, subscription, or payment attempt in your systems.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Merchant-scoped identifier of the customer associated with the checkout. Use it when storing payment instruments or reusing saved customer context for recurring and returning-payer flows.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// The timestamp of when the checkout was created.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Short merchant-defined description shown in SumUp tools and reporting. Use it to make the checkout easier to recognize in dashboards, support workflows, and reconciliation.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of the SumUp-hosted payment page that handles the payment flow. Returned when Hosted Checkout is enabled for the checkout.
    ///
    /// Constraints:
    /// - read-only
    /// - format: `uri`
    ///
    /// Example: `https://checkout.sumup.com/pay/8f9316a3-cda9-42a9-9771-54d534315676`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout_url: Option<String>,
    /// Unique SumUp identifier of the checkout resource.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `4e425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// Short unique identifier for the merchant that receives the payment.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Name of the merchant.
    ///
    /// Example: `Sample Merchant`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// Details of the saved payment instrument created or reused during checkout processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<CheckoutSuccessPaymentInstrument>,
    /// URL where the payer is redirected after a redirect-based payment or SCA flow completes.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Optional backend callback URL for checkout status notifications. SumUp sends an HTTP POST with `event_type` and the checkout `id`. Retrieve the checkout to verify its current status before updating your order. See the [webhook guide](https://developer.sumup.com/online-payments/webhooks/) for the payload and response requirements.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
    ///
    /// Example: `PENDING`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutSuccessStatus>,
    /// Transaction code of the successful transaction with which the payment for the checkout is completed.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Unique identifier of the successful transaction that completed payment for the checkout.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `410fc44a-5956-44e1-b5cc-19c6f8d727a4`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Payment attempts and resulting transaction records linked to this checkout. Use the Transactions endpoints when you need the authoritative payment result and event history.
    ///
    /// Constraints:
    /// - items must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CheckoutSuccessTransactionsItem>>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2030-12-31T23:59:59Z`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
}
/// Request body for updating an existing checkout. Include only the fields that should be changed.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutUpdateRequest {
    /// Updated amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `12.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Updated merchant-defined reference for the checkout.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Updated merchant-scoped customer identifier associated with the checkout.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Updated short merchant-defined description shown in SumUp tools and reporting.
    ///
    /// Example: `Updated purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable.
    ///
    /// Example: `2030-12-31T23:59:59Z`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
}
/// Details of a request validation error.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DetailsError {
    /// Details of the error.
    ///
    /// Example: `One or more of the parameters are invalid.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// List of violated validation constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_constraints: Option<Vec<DetailsErrorFailedConstraintsItem>>,
    /// HTTP status code for the error.
    ///
    /// Example: `400`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<f64>,
    /// Short title of the error.
    ///
    /// Example: `Bad Request`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for DetailsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for DetailsError {}
/// Hosted Checkout configuration. Enable it to receive a SumUp-hosted payment page URL in the checkout response.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HostedCheckout {
    /// Whether the checkout should include a SumUp-hosted payment page.
    ///
    /// Example: `true`
    pub enabled: bool,
}
/// Mandate details used when a checkout should create a reusable card token for future recurring or merchant-initiated payments.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcessCheckout {
    /// Raw payment token object received from Apple Pay. Send the Apple Pay response payload as-is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    /// Customer identifier associated with the saved payment instrument. Required when `token` is provided.
    ///
    /// Example: `MEDKHDTI`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Raw `PaymentData` object received from Google Pay. Send the Google Pay response payload as-is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<serde_json::Value>,
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
    /// Payment method used for this processing attempt. It determines which additional request fields are required.
    ///
    /// Example: `card`
    pub payment_type: ProcessCheckoutPaymentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
    /// Token of a saved payment instrument returned by checkout processing or the customer's payment-instruments endpoint. To charge a saved card, set `payment_type` to `card` and provide both this `token` and the associated `customer_id` instead of raw card details.
    ///
    /// Example: `ba85dfee-c3cf-48a6-84f5-d7d761fbba50`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
/// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
///
/// Example: `PENDING`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutTransactionsItem {
    /// Total amount of the transaction in major units of `currency`, for example `10.1` for EUR 10.10.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Authorization code for the transaction sent by the payment card issuer or bank. Applicable only to card payments.
    ///
    /// Example: `053201`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<EntryMode>,
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
    /// Unique code of the registered merchant to whom the payment is made.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatus>,
    /// The timestamp of when the transaction was created.
    ///
    /// Example: `2020-02-29T10:56:56.876Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Tip included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `3`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f32>,
    /// SumUp transaction code, for example `TEENSK4W2K`. Use it to look up the transaction with the `transaction_code` query parameter. This is separate from the transaction's `id` and the card issuer's `auth_code`.
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// VAT included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `6`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f32>,
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CheckoutAcceptedNextStepMechanismItem {
    #[serde(rename = "iframe")]
    Iframe,
    #[serde(rename = "browser")]
    Browser,
    #[serde(untagged)]
    Other(String),
}
/// Parameters required to complete the next step. The exact keys depend on the payment provider and flow type.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcceptedNextStepPayload {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub additional_properties: std::collections::HashMap<String, String>,
}
/// Instructions for the next action the payer or client must take.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcceptedNextStep {
    /// Allowed presentation mechanisms for the next step. `iframe` means the flow can be embedded, while `browser` means it can be completed through a full-page redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<Vec<CheckoutAcceptedNextStepMechanismItem>>,
    /// HTTP method to use when following the next step.
    ///
    /// Example: `POST`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Parameters required to complete the next step. The exact keys depend on the payment provider and flow type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// Merchant URL where the payer returns after the external flow finishes.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// URL to open or submit in order to continue processing.
    ///
    /// Example: `https://dummy-3ds-gateway.com/cap?RID=1233&VAA=A`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
/// Business purpose of the checkout. Use `CHECKOUT` for a standard payment and `SETUP_RECURRING_PAYMENT` when collecting consent and payment details for future recurring charges.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CheckoutCreateRequestPurpose {
    #[serde(rename = "CHECKOUT")]
    Checkout,
    #[serde(rename = "SETUP_RECURRING_PAYMENT")]
    SetupRecurringPayment,
    #[serde(untagged)]
    Other(String),
}
/// Details of the saved payment instrument created or reused during checkout processing.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccessPaymentInstrument {
    /// Unique token of the saved payment instrument.
    ///
    /// Example: `e76d7e5c-9375-4fac-a7e7-b19dc5302fbc`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
/// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
///
/// Example: `PENDING`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccessTransactionsItem {
    /// Total amount of the transaction in major units of `currency`, for example `10.1` for EUR 10.10.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Authorization code for the transaction sent by the payment card issuer or bank. Applicable only to card payments.
    ///
    /// Example: `053201`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<EntryMode>,
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
    /// Unique code of the registered merchant to whom the payment is made.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatus>,
    /// The timestamp of when the transaction was created.
    ///
    /// Example: `2020-02-29T10:56:56.876Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Tip included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `3`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f32>,
    /// SumUp transaction code, for example `TEENSK4W2K`. Use it to look up the transaction with the `transaction_code` query parameter. This is separate from the transaction's `id` and the card issuer's `auth_code`.
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// VAT included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `6`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f32>,
}
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DetailsErrorFailedConstraintsItem {
    /// Human-readable description of the violated constraint.
    ///
    /// Example: `Currency must also be specified when filtering by amount`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Name of the field that violated the constraint.
    ///
    /// Example: `currency`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
/// Type of mandate to create for the saved payment instrument.
///
/// Example: `recurrent`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MandatePayloadType {
    #[serde(rename = "recurrent")]
    Recurrent,
    #[serde(untagged)]
    Other(String),
}
/// Payment method used for this processing attempt. It determines which additional request fields are required.
///
/// Example: `card`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
/// Business purpose of the checkout. Use `CHECKOUT` for a standard payment and `SETUP_RECURRING_PAYMENT` when collecting consent and payment details for future recurring charges.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateRequestPurpose {
    #[serde(rename = "CHECKOUT")]
    Checkout,
    #[serde(rename = "SETUP_RECURRING_PAYMENT")]
    SetupRecurringPayment,
    #[serde(untagged)]
    Other(String),
}
/// Payment method used for this processing attempt. It determines which additional request fields are required.
///
/// Example: `card`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProcessRequestPaymentType {
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
/// Details of the saved payment instrument created or reused during checkout processing.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcessResponse200PaymentInstrument {
    /// Unique token of the saved payment instrument.
    ///
    /// Example: `e76d7e5c-9375-4fac-a7e7-b19dc5302fbc`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
/// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
///
/// Example: `PENDING`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProcessResponse200Status {
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
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcessResponse200TransactionsItem {
    /// Total amount of the transaction in major units of `currency`, for example `10.1` for EUR 10.10.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Authorization code for the transaction sent by the payment card issuer or bank. Applicable only to card payments.
    ///
    /// Example: `053201`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<EntryMode>,
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
    /// Unique code of the registered merchant to whom the payment is made.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatus>,
    /// The timestamp of when the transaction was created.
    ///
    /// Example: `2020-02-29T10:56:56.876Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Tip included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `3`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f32>,
    /// SumUp transaction code, for example `TEENSK4W2K`. Use it to look up the transaction with the `transaction_code` query parameter. This is separate from the transaction's `id` and the card issuer's `auth_code`.
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// VAT included in the total transaction amount, in major units of the transaction's currency.
    ///
    /// Example: `6`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f32>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListAvailablePaymentMethodsResponseAvailablePaymentMethodsItem {
    /// Unique identifier of the payment method.
    ///
    /// Example: `qr_code_pix`
    pub id: String,
}
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListParams {
    /// Filters checkouts by the merchant-defined `checkout_reference` supplied when creating the checkout. This is separate from the SumUp-generated checkout `id`.
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
}
/// Returns a list of checkout resources.
pub type ListResponse = Vec<CheckoutSuccess>;
/// Details for creating a checkout resource.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRequest {
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    pub amount: f32,
    /// Merchant-defined reference for the new checkout, up to 64 characters. Use it to correlate the checkout with an order or payment attempt in your own system. If a checkout already exists for the supplied unique parameters, creation returns `409` with `DUPLICATED_CHECKOUT`; see the conflict response.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    pub checkout_reference: String,
    pub currency: Currency,
    /// Merchant-scoped customer identifier. Required when setting up recurring payments and useful when the checkout should be linked to a returning payer.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Short merchant-defined description shown in SumUp tools and reporting for easier identification of the checkout.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout: Option<HostedCheckout>,
    /// Short unique identifier for the merchant that should receive the payment.
    ///
    /// Example: `MH4H92C7`
    pub merchant_code: String,
    /// Business purpose of the checkout. Use `CHECKOUT` for a standard payment and `SETUP_RECURRING_PAYMENT` when collecting consent and payment details for future recurring charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CreateRequestPurpose>,
    /// URL where the payer should be sent after a redirect-based payment or SCA flow completes. This is required for [APMs](https://developer.sumup.com/online-payments/apm/introduction) and recommended for card checkouts that may require [3DS](https://developer.sumup.com/online-payments/features/3ds). If it is omitted, the [Payment Widget](https://developer.sumup.com/online-payments/checkouts) can render the challenge in an iframe instead of using a full-page redirect.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Optional backend callback URL for checkout status notifications. SumUp sends an HTTP POST with `event_type` and the checkout `id`. Retrieve the checkout to verify its current status before updating your order. See the [webhook guide](https://developer.sumup.com/online-payments/webhooks/) for the payload and response requirements.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com/`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2030-12-31T23:59:59Z`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
}
/// Details for updating a checkout resource.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRequest {
    /// Updated amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `12.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Updated merchant-defined reference for the checkout.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Updated merchant-scoped customer identifier associated with the checkout.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Updated short merchant-defined description shown in SumUp tools and reporting.
    ///
    /// Example: `Updated purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable.
    ///
    /// Example: `2030-12-31T23:59:59Z`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
}
/// Details of the payment instrument for processing the checkout.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcessRequest {
    /// Raw payment token object received from Apple Pay. Send the Apple Pay response payload as-is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    /// Customer identifier associated with the saved payment instrument. Required when `token` is provided.
    ///
    /// Example: `MEDKHDTI`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Raw `PaymentData` object received from Google Pay. Send the Google Pay response payload as-is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<serde_json::Value>,
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
    /// Payment method used for this processing attempt. It determines which additional request fields are required.
    ///
    /// Example: `card`
    pub payment_type: ProcessRequestPaymentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
    /// Token of a saved payment instrument returned by checkout processing or the customer's payment-instruments endpoint. To charge a saved card, set `payment_type` to `card` and provide both this `token` and the associated `customer_id` instead of raw card details.
    ///
    /// Example: `ba85dfee-c3cf-48a6-84f5-d7d761fbba50`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
/// Returns the checkout resource after a processing attempt.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcessResponse200 {
    /// Amount to be charged to the payer, expressed in major units.
    ///
    /// Example: `10.1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Merchant-defined reference for the checkout. Use it to correlate the SumUp checkout with your own order, cart, subscription, or payment attempt in your systems.
    ///
    /// Constraints:
    /// - max length: 64
    ///
    /// Example: `f00a8f74-b05d-4605-bd73-2a901bae5802`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Merchant-scoped identifier of the customer associated with the checkout. Use it when storing payment instruments or reusing saved customer context for recurring and returning-payer flows.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// The timestamp of when the checkout was created.
    ///
    /// Example: `2020-02-29T10:56:56+00:00`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Short merchant-defined description shown in SumUp tools and reporting. Use it to make the checkout easier to recognize in dashboards, support workflows, and reconciliation.
    ///
    /// Example: `Purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of the SumUp-hosted payment page that handles the payment flow. Returned when Hosted Checkout is enabled for the checkout.
    ///
    /// Constraints:
    /// - read-only
    /// - format: `uri`
    ///
    /// Example: `https://checkout.sumup.com/pay/8f9316a3-cda9-42a9-9771-54d534315676`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_checkout_url: Option<String>,
    /// Unique SumUp identifier of the checkout resource.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `4e425463-3e1b-431d-83fa-1e51c2925e99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// Short unique identifier for the merchant that receives the payment.
    ///
    /// Example: `MH4H92C7`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Name of the merchant.
    ///
    /// Example: `Sample Merchant`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// Details of the saved payment instrument created or reused during checkout processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<ProcessResponse200PaymentInstrument>,
    /// URL where the payer is redirected after a redirect-based payment or SCA flow completes.
    ///
    /// Example: `https://mysite.com/completed_purchase`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Optional backend callback URL for checkout status notifications. SumUp sends an HTTP POST with `event_type` and the checkout `id`. Retrieve the checkout to verify its current status before updating your order. See the [webhook guide](https://developer.sumup.com/online-payments/webhooks/) for the payload and response requirements.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `http://example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
    ///
    /// Example: `PENDING`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProcessResponse200Status>,
    /// Transaction code of the successful transaction with which the payment for the checkout is completed.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `TEENSK4W2K`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Unique identifier of the successful transaction that completed payment for the checkout.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `410fc44a-5956-44e1-b5cc-19c6f8d727a4`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Payment attempts and resulting transaction records linked to this checkout. Use the Transactions endpoints when you need the authoritative payment result and event history.
    ///
    /// Constraints:
    /// - items must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<ProcessResponse200TransactionsItem>>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    ///
    /// Example: `2030-12-31T23:59:59Z`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProcessResponse {
    Status200(ProcessResponse200),
    Status202(CheckoutAccepted),
}
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListAvailablePaymentMethodsParams {
    /// Payment amount in major units, for example `9.99` for EUR 9.99. When filtering by `amount`, also provide `currency`.
    ///
    /// Example: `9.99`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Three-letter ISO 4217 currency code for which the payment methods should be eligible, for example `EUR`.
    ///
    /// Example: `EUR`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
/// Available payment methods
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListAvailablePaymentMethodsResponse {
    /// Payment methods available to the merchant for the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payment_methods:
        Option<Vec<ListAvailablePaymentMethodsResponseAvailablePaymentMethodsItem>>,
}
/// Merchant validation details from the Apple Pay session in the payer's browser.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateApplePaySessionRequest {
    /// Hostname of the website displaying the Apple Pay payment sheet, without a URL scheme or path. Use the domain registered for Apple Pay.
    ///
    /// Constraints:
    /// - format: `hostname`
    ///
    /// Example: `example.com`
    pub context: String,
    /// Apple Pay validation URL received as `validationURL` in the browser's `onvalidatemerchant` event.
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
    BadRequest(ErrorExtended),
    Unauthorized(Problem),
    Forbidden(ErrorForbidden),
    Conflict(Error),
}
impl crate::error::ProblemDetails for CreateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum DeactivateErrorBody {
    Unauthorized(Problem),
    NotFound(Error),
    Conflict(Error),
}
impl crate::error::ProblemDetails for DeactivateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum GetErrorBody {
    Unauthorized(Problem),
    NotFound(Error),
}
impl crate::error::ProblemDetails for GetErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum UpdateErrorBody {
    Unauthorized(Problem),
    NotFound(Error),
}
impl crate::error::ProblemDetails for UpdateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ProcessErrorBody {
    BadRequest(crate::error::UnknownApiBody),
    Unauthorized(Problem),
    NotFound(Error),
    Conflict(Error),
}
impl crate::error::ProblemDetails for ProcessErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ListAvailablePaymentMethodsErrorBody {
    BadRequest(DetailsError),
}
impl crate::error::ProblemDetails for ListAvailablePaymentMethodsErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        None
    }
}
#[derive(Debug, PartialEq)]
pub enum CreateApplePaySessionErrorBody {
    BadRequest(crate::error::UnknownApiBody),
    NotFound(Error),
}
impl crate::error::ProblemDetails for CreateApplePaySessionErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        None
    }
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(ListErrorBody::Unauthorized)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Create a checkout
    ///
    /// Creates a payment checkout for the specified merchant, amount, and currency. Supply a `checkout_reference` to identify the payment attempt in your own systems. Creating a checkout does not charge a payment instrument.
    ///
    /// Set `hosted_checkout.enabled` to `true` to receive a [Hosted Checkout](https://developer.sumup.com/online-payments/checkouts/hosted-checkout/) URL where the customer can complete the payment.
    /// Use `redirect_url` for redirect-based payment and 3DS flows. If `return_url` is provided, SumUp sends processing updates to that backend callback URL.
    ///
    /// Complete the payment through [Hosted Checkout](https://developer.sumup.com/online-payments/checkouts/hosted-checkout/) or the [Payment Widget](https://developer.sumup.com/online-payments/checkouts/card-widget).
    ///
    /// Responses:
    /// - 201: Returns the created checkout resource.
    /// - 400: The request body is invalid.
    /// - 401: The request is not authorized.
    /// - 403: The request isn't sufficiently authorized to create a checkout.
    /// - 409: A checkout already exists for the provided unique parameters.
    pub async fn create(
        &self,
        body: CreateRequest,
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ErrorExtended>(raw).map(CreateErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(CreateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ErrorForbidden>(raw).map(CreateErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::CONFLICT => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw).map(CreateErrorBody::Conflict)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
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
        checkout_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<Checkout, DeactivateErrorBody> {
        let path = format!("/v0.1/checkouts/{}", checkout_id.as_ref());
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(DeactivateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw).map(DeactivateErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::CONFLICT => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw).map(DeactivateErrorBody::Conflict)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Retrieve a checkout
    ///
    /// Retrieves a checkout by its SumUp `checkout_id`. After processing a payment, returning from a redirect, or receiving a checkout notification, retrieve the checkout to confirm its current `status` before updating your order or displaying the payment outcome to the payer.
    ///
    /// Responses:
    /// - 200: Returns the requested checkout resource.
    /// - 401: The request is not authorized.
    /// - 404: The requested resource does not exist.
    pub async fn get(
        &self,
        checkout_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<CheckoutSuccess, GetErrorBody> {
        let path = format!("/v0.1/checkouts/{}", checkout_id.as_ref());
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw).map(GetErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Update a checkout
    ///
    /// Updates the amount, currency, description, reference, expiration, or customer associated with an existing checkout. Only the supplied fields are updated.
    ///
    /// This request changes the checkout details; it does not charge a payment instrument. Process the checkout separately to attempt a payment.
    ///
    /// Responses:
    /// - 200: Returns the updated checkout resource.
    /// - 401: The request is not authorized.
    /// - 404: The requested resource does not exist.
    pub async fn update(
        &self,
        checkout_id: impl AsRef<str>,
        body: UpdateRequest,
    ) -> crate::error::SdkResult<Checkout, UpdateErrorBody> {
        let path = format!("/v0.1/checkouts/{}", checkout_id.as_ref());
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
                let data: Checkout = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw).map(UpdateErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Process a checkout
    ///
    /// :::caution[PCI DSS compliance required]
    /// When you submit raw card details directly to the Checkout API, your systems store, process, or transmit cardholder data and are therefore subject to applicable [PCI DSS requirements](https://www.pcisecuritystandards.org/document_library/). You should only use this integration if your environment is appropriately PCI DSS compliant.
    /// :::
    ///
    /// Processing a checkout will attempt to charge the provided payment instrument for the amount of the specified checkout resource initiated in the `Create a checkout` endpoint.
    ///
    /// A processing response can require an additional payer action, such as a 3DS challenge or a payment-provider redirect. If `next_step` is returned, follow its instructions to continue the payment flow.
    ///
    /// Retrieve the checkout afterwards to confirm its payment status. Acceptance of the processing request does not by itself mean the checkout is paid.
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
        checkout_id: impl AsRef<str>,
        body: ProcessRequest,
    ) -> crate::error::SdkResult<ProcessResponse, ProcessErrorBody> {
        let path = format!("/v0.1/checkouts/{}", checkout_id.as_ref());
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
                let data: ProcessResponse200 = response.json().await?;
                Ok(ProcessResponse::Status200(data))
            }
            reqwest::StatusCode::ACCEPTED => {
                let data: CheckoutAccepted = response.json().await?;
                Ok(ProcessResponse::Status202(data))
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    Ok(ProcessErrorBody::BadRequest(
                        crate::error::UnknownApiBody::from_bytes(raw),
                    ))
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(ProcessErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw).map(ProcessErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::CONFLICT => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw).map(ProcessErrorBody::Conflict)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Get available payment methods
    ///
    /// Lists the payment methods available to the merchant for checkout payments. Use the optional amount and currency filters to check eligibility for a particular payment before presenting payment options to the payer.
    ///
    /// Responses:
    /// - 200: Available payment methods
    /// - 400: The request is invalid for the submitted query parameters.
    pub async fn list_available_payment_methods(
        &self,
        merchant_code: impl AsRef<str>,
        params: ListAvailablePaymentMethodsParams,
    ) -> crate::error::SdkResult<
        ListAvailablePaymentMethodsResponse,
        ListAvailablePaymentMethodsErrorBody,
    > {
        let path = format!("/v0.1/merchants/{}/payment-methods", merchant_code.as_ref());
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<DetailsError>(raw)
                        .map(ListAvailablePaymentMethodsErrorBody::BadRequest)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
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
    ///   that should be forwarded to the Apple Pay JS SDK to complete merchant
    ///   validation and continue the payment flow.
    /// - 400: Bad Request
    /// - 404: The requested resource does not exist.
    pub async fn create_apple_pay_session(
        &self,
        checkout_id: impl AsRef<str>,
        body: Option<CreateApplePaySessionRequest>,
    ) -> crate::error::SdkResult<CreateApplePaySessionResponse, CreateApplePaySessionErrorBody>
    {
        let path = format!("/v0.2/checkouts/{}/apple-pay-session", checkout_id.as_ref());
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
                Err(crate::error::SdkError::from_response(response, |raw| {
                    Ok(CreateApplePaySessionErrorBody::BadRequest(
                        crate::error::UnknownApiBody::from_bytes(raw),
                    ))
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw)
                        .map(CreateApplePaySessionErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
