// The contents of this file are generated; do not modify them.

//! The Payouts model will allow you to track funds you’ve received from SumUp.
//!
//! You can receive a detailed payouts list with information like dates, fees, references and statuses, using the `List payouts` endpoint.
use super::common::*;
/// A single payout-related record.
///
/// A record can represent either:
/// - an actual payout sent to the merchant (`type = PAYOUT`)
/// - a deduction applied against merchant funds for a refund, chargeback, direct debit return, or balance adjustment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FinancialPayout {
    /// Unique identifier of the payout-related record.
    ///
    /// Example: `123456789`
    pub id: i64,
    /// High-level payout record category.
    ///
    /// Example: `PAYOUT`
    #[serde(rename = "type")]
    pub r#type: FinancialPayoutType,
    /// Amount of the payout or deduction in major units.
    ///
    /// Example: `132.45`
    pub amount: f32,
    /// Payout date associated with the record, in `YYYY-MM-DD` format.
    ///
    /// Example: `2024-02-29`
    pub date: crate::datetime::Date,
    /// Three-letter ISO 4217 currency code of the payout.
    ///
    /// Example: `EUR`
    pub currency: String,
    /// Fee amount associated with the payout record, in major units.
    ///
    /// Example: `3.12`
    pub fee: f32,
    /// Merchant-facing outcome of the payout record.
    ///
    /// Example: `SUCCESSFUL`
    pub status: FinancialPayoutStatus,
    /// Processor or payout reference associated with the record.
    ///
    /// Example: `payout-2024-02-29`
    pub reference: String,
    /// Transaction code of the original sale associated with the payout or deduction.
    ///
    /// Example: `TEENSK4W2K`
    pub transaction_code: String,
}
pub type FinancialPayouts = Vec<FinancialPayout>;
/// High-level payout record category.
///
/// Example: `PAYOUT`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FinancialPayoutType {
    #[serde(rename = "PAYOUT")]
    Payout,
    #[serde(rename = "CHARGE_BACK_DEDUCTION")]
    ChargeBackDeduction,
    #[serde(rename = "REFUND_DEDUCTION")]
    RefundDeduction,
    #[serde(rename = "DD_RETURN_DEDUCTION")]
    DdReturnDeduction,
    #[serde(rename = "BALANCE_DEDUCTION")]
    BalanceDeduction,
    #[serde(untagged)]
    Other(String),
}
/// Merchant-facing outcome of the payout record.
///
/// Example: `SUCCESSFUL`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FinancialPayoutStatus {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListParamsFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "csv")]
    Csv,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ListParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(untagged)]
    Other(String),
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListParams {
    /// Start date of the payout period filter, inclusive, in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) `date` format (`YYYY-MM-DD`).
    ///
    /// Example: `2024-02-01`
    pub start_date: crate::datetime::Date,
    /// End date of the payout period filter, inclusive, in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) `date` format (`YYYY-MM-DD`). Must be greater than or equal to `start_date`.
    ///
    /// Example: `2024-02-29`
    pub end_date: crate::datetime::Date,
    /// Response format for the payout list.
    ///
    /// Example: `json`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ListParamsFormat>,
    /// Maximum number of payout records to return.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 9999
    ///
    /// Example: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Sort direction for the returned payouts.
    ///
    /// Example: `desc`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListParamsOrder>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum ListErrorBody {
    BadRequest(crate::error::UnknownApiBody),
    Unauthorized(Problem),
}
/// Client for the Payouts API endpoints.
#[derive(Debug)]
pub struct PayoutsClient<'a> {
    client: &'a Client,
}
impl<'a> PayoutsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// List payouts
    ///
    /// Lists payout and payout-deduction records for the specified merchant account within the requested date range.
    ///
    /// The response can include:
    /// - regular payouts (`type = PAYOUT`)
    /// - deduction records for refunds, chargebacks, direct debit returns, or balance adjustments
    ///
    /// Results are sorted by payout date in the requested `order`.
    ///
    /// Responses:
    /// - 200: Returns the list of payout and deduction records for the requested period.
    /// - 400: The request is invalid for the submitted query parameters.
    /// - 401: The request is not authorized.
    pub async fn list(
        &self,
        merchant_code: impl Into<String>,
        params: ListParams,
    ) -> crate::error::SdkResult<FinancialPayouts, ListErrorBody> {
        let path = format!("/v1.0/merchants/{}/payouts", merchant_code.into());
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
        request = request.query(&[("start_date", &params.start_date)]);
        request = request.query(&[("end_date", &params.end_date)]);
        if let Some(ref value) = params.format {
            request = request.query(&[("format", value)]);
        }
        if let Some(ref value) = params.limit {
            request = request.query(&[("limit", value)]);
        }
        if let Some(ref value) = params.order {
            request = request.query(&[("order", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: FinancialPayouts = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::api(ListErrorBody::BadRequest(body)))
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
}
