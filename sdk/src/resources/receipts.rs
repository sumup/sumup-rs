// The contents of this file are generated; do not modify them.

//! The Receipts model obtains receipt-like details for specific transactions.
use super::common::*;
/// Receipt details for a transaction.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<ReceiptTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_data: Option<ReceiptMerchantData>,
    /// EMV-specific metadata returned for card-present payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_data: Option<serde_json::Value>,
    /// Acquirer-specific metadata related to the card authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquirer_data: Option<ReceiptAcquirerData>,
}
/// Payment card details displayed on the receipt.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptCard {
    /// Card last 4 digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    /// Card Scheme.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
/// Transaction event details as rendered on the receipt.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptEvent {
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
    ///
    /// Constraints:
    /// - format: `double`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub amount: Option<f64>,
    /// Date and time of the transaction event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Receipt number associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_no: Option<String>,
}
/// Receipt merchant data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptMerchantData {
    /// Merchant profile details displayed on the receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_profile: Option<ReceiptMerchantDataMerchantProfile>,
    /// Locale used for rendering localized receipt fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
/// Card reader details displayed on the receipt.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptReader {
    /// Reader serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Reader type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
/// Transaction information.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptTransaction {
    /// Transaction code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
    /// Merchant code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Transaction VAT amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<String>,
    /// Tip amount (included in transaction amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<String>,
    /// Transaction currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Time created at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::datetime::DateTime>,
    /// Transaction processing status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Transaction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// Transaction entry mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<String>,
    /// Cardholder verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_reader: Option<ReceiptReader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ReceiptCard>,
    /// Number of installments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
    /// Debit/Credit.
    ///
    /// Example: `CREDIT`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_as: Option<ReceiptTransactionProcessAs>,
    /// Products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<ReceiptTransactionProductsItem>>,
    /// Vat rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rates: Option<Vec<ReceiptTransactionVatRatesItem>>,
    /// Events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ReceiptEvent>>,
    /// Receipt number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_no: Option<String>,
}
/// Acquirer-specific metadata related to the card authorization.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptAcquirerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptMerchantDataMerchantProfileAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_native_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landline: Option<String>,
}
/// Merchant profile details displayed on the receipt.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptMerchantDataMerchantProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ReceiptMerchantDataMerchantProfileAddress>,
}
/// Debit/Credit.
///
/// Example: `CREDIT`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReceiptTransactionProcessAs {
    #[serde(rename = "CREDIT")]
    Credit,
    #[serde(rename = "DEBIT")]
    Debit,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptTransactionProductsItem {
    /// Product name
    ///
    /// Example: `Coffee`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Product description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Product price
    ///
    /// Constraints:
    /// - format: `double`
    ///
    /// Example: `150.0`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub price: Option<f64>,
    /// VAT rate
    ///
    /// Constraints:
    /// - format: `double`
    ///
    /// Example: `0.0`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub vat_rate: Option<f64>,
    /// VAT amount for a single product
    ///
    /// Constraints:
    /// - format: `double`
    ///
    /// Example: `0.0`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub single_vat_amount: Option<f64>,
    /// Product price including VAT
    ///
    /// Constraints:
    /// - format: `double`
    ///
    /// Example: `150.0`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub price_with_vat: Option<f64>,
    /// VAT amount
    ///
    /// Constraints:
    /// - format: `double`
    ///
    /// Example: `0.0`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub vat_amount: Option<f64>,
    /// Product quantity
    ///
    /// Example: `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Quantity x product price
    ///
    /// Constraints:
    /// - format: `double`
    ///
    /// Example: `150.0`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub total_price: Option<f64>,
    /// Total price including VAT
    ///
    /// Constraints:
    /// - format: `double`
    ///
    /// Example: `150.0`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::string_or_number::deserialize_option"
    )]
    pub total_with_vat: Option<f64>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptTransactionVatRatesItem {
    /// Gross
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross: Option<f32>,
    /// Net
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<f32>,
    /// Rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    /// Vat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<f32>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetParams {
    /// Merchant code.
    pub mid: String,
    /// The ID of the transaction event (refund).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_event_id: Option<i64>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum GetErrorBody {
    BadRequest(Error),
    Unauthorized(Problem),
    NotFound(Error),
}
/// Client for the Receipts API endpoints.
#[derive(Debug)]
pub struct ReceiptsClient<'a> {
    client: &'a Client,
}
impl<'a> ReceiptsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Retrieve receipt details
    ///
    /// Retrieves receipt specific data for a transaction.
    ///
    /// Responses:
    /// - 200: Returns receipt details for the requested transaction.
    /// - 400: The request is invalid for the submitted parameters.
    /// - 401: The request is not authorized.
    /// - 404: The requested transaction event does not exist for the provided transaction.
    pub async fn get(
        &self,
        id: impl Into<String>,
        params: GetParams,
    ) -> crate::error::SdkResult<Receipt, GetErrorBody> {
        let path = format!("/v1.1/receipts/{}", id.into());
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
        request = request.query(&[("mid", &params.mid)]);
        if let Some(ref value) = params.tx_event_id {
            request = request.query(&[("tx_event_id", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Receipt = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(GetErrorBody::BadRequest(body)))
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
}
