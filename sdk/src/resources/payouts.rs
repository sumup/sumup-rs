// The contents of this file are generated; do not modify them.

use super::common::*;
pub type FinancialPayouts = Vec<serde_json::Value>;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListPayoutsParams {
    /// Start date (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    pub start_date: String,
    /// End date (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    pub end_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListPayoutsV1Params {
    /// Start date (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    pub start_date: String,
    /// End date (in [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format).
    pub end_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum ListPayoutsErrorBody {
    Unauthorized(Error),
}
#[derive(Debug)]
pub enum ListPayoutsV1ErrorBody {
    Unauthorized(Error),
}
///Client for the Payouts API endpoints.
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
    /// Lists ordered payouts for the merchant profile.
    pub async fn list_deprecated(
        &self,
        params: ListPayoutsParams,
    ) -> crate::error::SdkResult<FinancialPayouts, ListPayoutsErrorBody> {
        let path = "/v0.1/me/financials/payouts";
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
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ListPayoutsErrorBody::Unauthorized(body),
                ))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// List payouts
    ///
    /// Lists ordered payouts for the merchant profile.
    pub async fn list(
        &self,
        merchant_code: impl Into<String>,
        params: ListPayoutsV1Params,
    ) -> crate::error::SdkResult<FinancialPayouts, ListPayoutsV1ErrorBody> {
        let path = format!("/v1.0/merchants/{}/payouts", merchant_code.into());
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
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api(
                    ListPayoutsV1ErrorBody::Unauthorized(body),
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
