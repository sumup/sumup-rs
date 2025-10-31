// The contents of this file are generated; do not modify them.

use super::common::*;
/// __Required when payment type is `card`.__ Details of the payment card.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// Name of the cardholder as it appears on the payment card.
    pub name: String,
    /// Number of the payment card (without spaces).
    pub number: String,
    /// Year from the expiration time of the payment card. Accepted formats are `YY` and `YYYY`.
    pub expiry_year: String,
    /// Month from the expiration time of the payment card. Accepted format is `MM`.
    pub expiry_month: String,
    /// Three or four-digit card verification value (security code) of the payment card.
    pub cvv: String,
    /// Required five-digit ZIP code. Applicable only to merchant users in the USA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Last 4 digits of the payment card number.
    pub last_4_digits: String,
    /// Issuing card network of the payment card.
    #[serde(rename = "type")]
    pub type_: String,
}
/// Details of the payment checkout.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Checkout {
    /// Unique ID of the payment checkout specified by the client application when creating the checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    /// Amount of the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Unique identifying code of the merchant profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Short description of the checkout visible in the SumUp dashboard. The description can contribute to reporting, allowing easier identification of a checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL to which the SumUp platform sends the processing status of the payment checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Unique ID of the checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current status of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Date and time of the checkout expiration before which the client application needs to send a processing request. If no value is present, the checkout does not have an expiration time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<crate::datetime::DateTime>,
    /// Unique identification of a customer. If specified, the checkout session and payment instrument are associated with the referenced customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// List of transactions related to the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CheckoutTransactionsItem>>,
}
/// 3DS Response
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAccepted {
    /// Required action processing 3D Secure payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<CheckoutAcceptedNextStep>,
}
/// Details of the payment checkout.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutCreateRequest {
    /// Unique ID of the payment checkout specified by the client application when creating the checkout resource.
    pub checkout_reference: String,
    /// Amount of the payment.
    pub amount: f64,
    pub currency: Currency,
    /// Unique identifying code of the merchant profile.
    pub merchant_code: String,
    /// Short description of the checkout visible in the SumUp dashboard. The description can contribute to reporting, allowing easier identification of a checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL to which the SumUp platform sends the processing status of the payment checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Unique identification of a customer. If specified, the checkout session and payment instrument are associated with the referenced customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Purpose of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Unique ID of the checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current status of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Date and time of the checkout expiration before which the client application needs to send a processing request. If no value is present, the checkout does not have an expiration time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<crate::datetime::DateTime>,
    /// List of transactions related to the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CheckoutCreateRequestTransactionsItem>>,
    /// __Required__ for [APMs](https://developer.sumup.com/online-payments/apm/introduction) and __recommended__ for card payments. Refers to a url where the end user is redirected once the payment processing completes. If not specified, the [Payment Widget](https://developer.sumup.com/online-payments/tools/card-widget) renders [3DS challenge](https://developer.sumup.com/online-payments/features/3ds) within an iframe instead of performing a full-page redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
/// Details of the payment instrument for processing the checkout.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutProcessMixin {
    /// Describes the payment method used to attempt processing
    pub payment_type: String,
    /// Number of installments for deferred payments. Available only to merchant users in Brazil.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandatePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    /// __Required when using a tokenized card to process a checkout.__ Unique token identifying the saved payment card for a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// __Required when `token` is provided.__ Unique ID of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSuccess {
    /// Unique ID of the payment checkout specified by the client application when creating the checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    /// Amount of the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Unique identifying code of the merchant profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Short description of the checkout visible in the SumUp dashboard. The description can contribute to reporting, allowing easier identification of a checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL to which the SumUp platform sends the processing status of the payment checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Unique ID of the checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current status of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Date and time of the checkout expiration before which the client application needs to send a processing request. If no value is present, the checkout does not have an expiration time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<crate::datetime::DateTime>,
    /// Unique identification of a customer. If specified, the checkout session and payment instrument are associated with the referenced customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// List of transactions related to the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CheckoutSuccessTransactionsItem>>,
    /// Transaction code of the successful transaction with which the payment for the checkout is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    /// Transaction ID of the successful transaction with which the payment for the checkout is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Name of the merchant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// Refers to a url where the end user is redirected once the payment processing completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Object containing token information for the specified payment instrument
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
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErrorExtended {
    /// Short description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Platform code for the error.
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
/// Mandate is passed when a card is to be tokenized
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MandatePayload {
    /// Indicates the mandate type
    #[serde(rename = "type")]
    pub type_: String,
    /// Operating system and web client used by the end-user
    pub user_agent: String,
    /// IP address of the end user. Supports IPv4 and IPv6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ip: Option<String>,
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
/// Contains parameters essential for form redirection. Number of object keys and their content can vary.
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
/// Required action processing 3D Secure payments.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcceptedNextStep {
    /// Where the end user is redirected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Method used to complete the redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Refers to a url where the end user is redirected once the payment processing completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Indicates allowed mechanisms for redirecting an end user. If both values are provided to ensure a redirect takes place in either.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<Vec<String>>,
    /// Contains parameters essential for form redirection. Number of object keys and their content can vary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<CheckoutAcceptedNextStepPayload>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutCreateRequestTransactionsItem {
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
/// Object containing token information for the specified payment instrument
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
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DeactivateCheckoutResponseTransactionsItem {
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetPaymentMethodsResponseAvailablePaymentMethodsItem {
    /// The ID of the payment method.
    pub id: String,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListCheckoutsParams {
    /// Filters the list of checkout resources by the unique ID of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
}
/// OK
pub type ListCheckoutsResponse = Vec<CheckoutSuccess>;
/// OK
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DeactivateCheckoutResponse {
    /// Unique ID of the payment checkout specified by the client application when creating the checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    /// Unique ID of the checkout resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Amount of the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Unique identifying code of the merchant profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Short description of the checkout visible in the SumUp dashboard. The description can contribute to reporting, allowing easier identification of a checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Purpose of the checkout creation initially
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Current status of the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time of the creation of the payment checkout. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::datetime::DateTime>,
    /// Date and time of the checkout expiration before which the client application needs to send a processing request. If no value is present, the checkout does not have an expiration time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<crate::datetime::DateTime>,
    /// Merchant name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// The merchant's country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_country: Option<String>,
    /// List of transactions related to the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<DeactivateCheckoutResponseTransactionsItem>>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProcessCheckoutResponse {
    Status200(CheckoutSuccess),
    Status202(CheckoutAccepted),
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetPaymentMethodsParams {
    /// The amount for which the payment methods should be eligible, in major units. Note that currency must also be provided when filtering by amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The currency for which the payment methods should be eligible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
/// Available payment methods
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetPaymentMethodsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payment_methods:
        Option<Vec<GetPaymentMethodsResponseAvailablePaymentMethodsItem>>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum ListCheckoutsErrorBody {
    Unauthorized(Error),
}
#[derive(Debug)]
pub enum CreateCheckoutErrorBody {
    BadRequest(ErrorExtended),
    Unauthorized(Error),
    Forbidden(ErrorForbidden),
    Conflict(Error),
}
#[derive(Debug)]
pub enum DeactivateCheckoutErrorBody {
    Unauthorized(Error),
    NotFound(Error),
    Conflict(Error),
}
#[derive(Debug)]
pub enum GetCheckoutErrorBody {
    Unauthorized(Error),
    NotFound(Error),
}
#[derive(Debug)]
pub enum ProcessCheckoutErrorBody {
    BadRequest,
    Unauthorized(Error),
    NotFound(Error),
    Conflict(Error),
}
#[derive(Debug)]
pub enum GetPaymentMethodsErrorBody {
    BadRequest(DetailsError),
    Unauthorized(Error),
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
        params: ListCheckoutsParams,
    ) -> crate::error::SdkResult<ListCheckoutsResponse, ListCheckoutsErrorBody> {
        let path = "/v0.1/checkouts";
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(ref value) = params.checkout_reference {
            request = request.query(&[("checkout_reference", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListCheckoutsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ListCheckoutsErrorBody::Unauthorized(body),
                ))
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
        body: Option<CheckoutCreateRequest>,
    ) -> crate::error::SdkResult<Checkout, CreateCheckoutErrorBody> {
        let path = "/v0.1/checkouts";
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .post(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(body) = body {
            request = request.json(&body);
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
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let body: ErrorForbidden = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::Forbidden(body),
                ))
            }
            reqwest::StatusCode::CONFLICT => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    CreateCheckoutErrorBody::Conflict(body),
                ))
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
    ) -> crate::error::SdkResult<DeactivateCheckoutResponse, DeactivateCheckoutErrorBody> {
        let path = format!("/v0.1/checkouts/{}", id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .delete(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: DeactivateCheckoutResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    DeactivateCheckoutErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    DeactivateCheckoutErrorBody::NotFound(body),
                ))
            }
            reqwest::StatusCode::CONFLICT => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    DeactivateCheckoutErrorBody::Conflict(body),
                ))
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
    ) -> crate::error::SdkResult<CheckoutSuccess, GetCheckoutErrorBody> {
        let path = format!("/v0.1/checkouts/{}", id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: CheckoutSuccess = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetCheckoutErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(GetCheckoutErrorBody::NotFound(
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
    /// Process a checkout
    ///
    /// Processing a checkout will attempt to charge the provided payment instrument for the amount of the specified checkout resource initiated in the `Create a checkout` endpoint.
    ///
    /// Follow this request with `Retrieve a checkout` to confirm its status.
    pub async fn process(
        &self,
        id: impl Into<String>,
        body: Option<CheckoutProcessMixin>,
    ) -> crate::error::SdkResult<ProcessCheckoutResponse, ProcessCheckoutErrorBody> {
        let path = format!("/v0.1/checkouts/{}", id.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .put(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: CheckoutSuccess = response.json().await?;
                Ok(ProcessCheckoutResponse::Status200(data))
            }
            reqwest::StatusCode::ACCEPTED => {
                let data: CheckoutAccepted = response.json().await?;
                Ok(ProcessCheckoutResponse::Status202(data))
            }
            reqwest::StatusCode::BAD_REQUEST => Err(crate::error::SdkError::api(
                ProcessCheckoutErrorBody::BadRequest,
            )),
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ProcessCheckoutErrorBody::Unauthorized(body),
                ))
            }
            reqwest::StatusCode::NOT_FOUND => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ProcessCheckoutErrorBody::NotFound(body),
                ))
            }
            reqwest::StatusCode::CONFLICT => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ProcessCheckoutErrorBody::Conflict(body),
                ))
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
        params: GetPaymentMethodsParams,
    ) -> crate::error::SdkResult<GetPaymentMethodsResponse, GetPaymentMethodsErrorBody> {
        let path = format!("/v0.1/merchants/{}/payment-methods", merchant_code.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
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
                let data: GetPaymentMethodsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: DetailsError = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetPaymentMethodsErrorBody::BadRequest(body),
                ))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    GetPaymentMethodsErrorBody::Unauthorized(body),
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
