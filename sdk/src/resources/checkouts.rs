// The contents of this file are generated; do not modify them.

use super::common::*;
/// __Required when payment type is `card`.__ Details of the payment card.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// Name of the cardholder as it appears on the payment card.
    ///
    /// Constraints:
    /// - write-only
    pub name: String,
    /// Number of the payment card (without spaces).
    ///
    /// Constraints:
    /// - write-only
    pub number: String,
    /// Year from the expiration time of the payment card. Accepted formats are `YY` and `YYYY`.
    ///
    /// Constraints:
    /// - write-only
    /// - min length: 2
    /// - max length: 4
    pub expiry_year: String,
    /// Month from the expiration time of the payment card. Accepted format is `MM`.
    ///
    /// Constraints:
    /// - write-only
    pub expiry_month: String,
    /// Three or four-digit card verification value (security code) of the payment card.
    ///
    /// Constraints:
    /// - write-only
    /// - min length: 3
    /// - max length: 4
    pub cvv: String,
    /// Required five-digit ZIP code. Applicable only to merchant users in the USA.
    ///
    /// Constraints:
    /// - write-only
    /// - min length: 5
    /// - max length: 5
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    /// Amount to be charged to the payer, expressed in major units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Merchant account that receives the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Short merchant-defined description shown in SumUp tools and reporting. Use it to make the checkout easier to recognize in dashboards, support workflows, and reconciliation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional backend callback URL used by SumUp to notify your platform about processing updates for the checkout.
    ///
    /// Constraints:
    /// - format: `uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Unique SumUp identifier of the checkout resource.
    ///
    /// Constraints:
    /// - read-only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
    /// Merchant-scoped identifier of the customer associated with the checkout. Use it when storing payment instruments or reusing saved customer context for recurring and returning-payer flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
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
    pub checkout_reference: String,
    /// Amount to be charged to the payer, expressed in major units.
    pub amount: f32,
    pub currency: Currency,
    /// Merchant account that should receive the payment.
    pub merchant_code: String,
    /// Short merchant-defined description shown in SumUp tools and reporting for easier identification of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional backend callback URL used by SumUp to notify your platform about processing updates for the checkout.
    ///
    /// Constraints:
    /// - format: `uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Merchant-scoped customer identifier. Required when setting up recurring payments and useful when the checkout should be linked to a returning payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Business purpose of the checkout. Use `CHECKOUT` for a standard payment and `SETUP_RECURRING_PAYMENT` when collecting consent and payment details for future recurring charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
    /// URL where the payer should be sent after a redirect-based payment or SCA flow completes. This is required for [APMs](https://developer.sumup.com/online-payments/apm/introduction) and recommended for card checkouts that may require [3DS](https://developer.sumup.com/online-payments/features/3ds). If it is omitted, the [Payment Widget](https://developer.sumup.com/online-payments/checkouts) can render the challenge in an iframe instead of using a full-page redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
/// Checkout resource returned after a synchronous processing attempt. In addition to the base checkout fields, it can include the resulting transaction identifiers and any newly created payment instrument token.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccess {
    /// Merchant-defined reference for the checkout. Use it to correlate the SumUp checkout with your own order, cart, subscription, or payment attempt in your systems.
    ///
    /// Constraints:
    /// - max length: 90
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    /// Amount to be charged to the payer, expressed in major units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Merchant account that receives the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Short merchant-defined description shown in SumUp tools and reporting. Use it to make the checkout easier to recognize in dashboards, support workflows, and reconciliation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional backend callback URL used by SumUp to notify your platform about processing updates for the checkout.
    ///
    /// Constraints:
    /// - format: `uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Unique SumUp identifier of the checkout resource.
    ///
    /// Constraints:
    /// - read-only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current high-level state of the checkout. `PENDING` means the checkout exists but is not yet completed, `PAID` means a payment succeeded, `FAILED` means the latest processing attempt failed, and `EXPIRED` means the checkout can no longer be processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Optional expiration timestamp. The checkout must be processed before this moment, otherwise it becomes unusable. If omitted, the checkout does not have an explicit expiry time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    pub valid_until: Option<crate::Nullable<crate::datetime::DateTime>>,
    /// Merchant-scoped identifier of the customer associated with the checkout. Use it when storing payment instruments or reusing saved customer context for recurring and returning-payer flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Transaction ID of the successful transaction with which the payment for the checkout is completed.
    ///
    /// Constraints:
    /// - read-only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Name of the merchant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// URL where the payer is redirected after a redirect-based payment or SCA flow completes.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Details of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The status code.
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
/// Mandate details used when a checkout should create a reusable card token for future recurring or merchant-initiated payments.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MandatePayload {
    /// Type of mandate to create for the saved payment instrument.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Browser or client user agent observed when consent was collected.
    pub user_agent: String,
    /// IP address of the payer when the mandate was accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ip: Option<String>,
}
/// Request body for attempting payment on an existing checkout. The required companion fields depend on the selected `payment_type`, for example card details, saved-card data, or payer information required by a specific payment method.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessCheckout {
    /// Payment method used for this processing attempt. It determines which additional request fields are required.
    pub payment_type: String,
    /// Number of installments for deferred payments. Available only to merchant users in Brazil.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 12
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Customer identifier associated with the saved payment instrument. Required when `token` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutTransactionsItem {
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
}
/// Parameters required to complete the next step. The exact keys depend on the payment provider and flow type.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcceptedNextStepPayload {
    #[serde(rename = "PaReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pareq: Option<serde_json::Value>,
    #[serde(rename = "MD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md: Option<serde_json::Value>,
    #[serde(rename = "TermUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termurl: Option<serde_json::Value>,
}
/// Instructions for the next action the payer or client must take.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcceptedNextStep {
    /// URL to open or submit in order to continue processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// HTTP method to use when following the next step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Merchant URL where the payer returns after the external flow finishes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Allowed presentation mechanisms for the next step. `iframe` means the flow can be embedded, while `browser` means it can be completed through a full-page redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<Vec<String>>,
    /// Parameters required to complete the next step. The exact keys depend on the payment provider and flow type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<CheckoutAcceptedNextStepPayload>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccessTransactionsItem {
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
}
/// Details of the saved payment instrument created or reused during checkout processing.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccessPaymentInstrument {
    /// Token value
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListAvailablePaymentMethodsResponseAvailablePaymentMethodsItem {
    /// The ID of the payment method.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The currency for which the payment methods should be eligible.
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
    BadRequest,
    Unauthorized(Problem),
    NotFound(Error),
    Conflict(Error),
}
#[derive(Debug)]
pub enum ListAvailablePaymentMethodsErrorBody {
    BadRequest(DetailsError),
}
///Client for the Checkouts API endpoints.
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
    ///
    /// Follow by processing a checkout to charge the provided payment instrument.
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
                Err(crate::error::SdkError::api(ProcessErrorBody::BadRequest))
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
}
