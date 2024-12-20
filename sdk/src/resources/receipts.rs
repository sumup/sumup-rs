// The contents of this file are generated; do not modify them.

use super::common::*;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<ReceiptTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_data: Option<ReceiptMerchantData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_data: Option<ReceiptEmvData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquirer_data: Option<ReceiptAcquirerData>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptCard {
    /// Card last 4 digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    /// Card Scheme.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<AmountEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimestampEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_no: Option<String>,
}
/// Receipt merchant data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptMerchantData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_profile: Option<ReceiptMerchantDataMerchantProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
/// Transaction information.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptTransaction {
    /// Transaction code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
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
    pub timestamp: Option<String>,
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
    pub card: Option<ReceiptCard>,
    /// Number of installments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i64>,
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
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptEmvData {}
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
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_native_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landline: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptMerchantDataMerchantProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ReceiptMerchantDataMerchantProfileAddress>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptTransactionProductsItem {
    /// Product name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Product price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Product quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Quantity x product price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<f64>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ReceiptTransactionVatRatesItem {
    /// Gross
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross: Option<f64>,
    /// Net
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<f64>,
    /// Rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /// Vat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<f64>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetReceiptParams {
    /// Merchant code.
    pub mid: String,
    /// The ID of the transaction event (refund).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_event_id: Option<i64>,
}
use crate::client::Client;
///Client for the Receipts API endpoints.
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
    pub async fn get(
        &self,
        id: impl Into<String>,
        params: GetReceiptParams,
    ) -> Result<Receipt, Box<dyn std::error::Error>> {
        let path = format!("/v1.1/receipts/{}", id.into());
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
        request = request.query(&[("mid", &params.mid)]);
        if let Some(ref value) = params.tx_event_id {
            request = request.query(&[("tx_event_id", value)]);
        }
        let response = request.send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Receipt = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let error: Error = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let error: Error = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            _ => {
                let status = response.status();
                let body = response.text().await?;
                Err(format!("Request failed with status {}: {}", status, body).into())
            }
        }
    }
}
