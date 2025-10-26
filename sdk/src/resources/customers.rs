// The contents of this file are generated; do not modify them.

use super::common::*;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    /// Unique ID of the customer.
    pub customer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}
/// Payment Instrument Response
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentInstrumentResponse {
    /// Unique token identifying the saved payment card for a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Indicates whether the payment instrument is active and can be used for payments. To deactivate it, send a `DELETE` request to the resource endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Type of the payment instrument.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Details of the payment card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentInstrumentResponseCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateResponse>,
    /// Creation date of payment instrument. Response format expressed according to [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<crate::datetime::DateTime>,
}
/// Details of the payment card.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentInstrumentResponseCard {
    /// Last 4 digits of the payment card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    /// Issuing card network of the payment card.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateCustomerBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}
/// OK
pub type ListPaymentInstrumentsResponse = Vec<PaymentInstrumentResponse>;
use crate::client::Client;
///Client for the Customers API endpoints.
#[derive(Debug)]
pub struct CustomersClient<'a> {
    client: &'a Client,
}
impl<'a> CustomersClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Create a customer
    ///
    /// Creates a new saved customer resource which you can later manipulate and save payment instruments to.
    pub async fn create(
        &self,
        body: Option<Customer>,
    ) -> Result<Customer, Box<dyn std::error::Error>> {
        let path = "/v0.1/customers";
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
        match response.status() {
            reqwest::StatusCode::CREATED => {
                let data: Customer = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let error: Error = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error: ErrorForbidden = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::CONFLICT => {
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
    /// Retrieve a customer
    ///
    /// Retrieves an identified saved customer resource through the unique `customer_id` parameter, generated upon customer creation.
    pub async fn get(
        &self,
        customer_id: impl Into<String>,
    ) -> Result<Customer, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/customers/{}", customer_id.into());
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
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Customer = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let error: Error = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error: ErrorForbidden = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::NOT_FOUND => {
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
    /// Update a customer
    ///
    /// Updates an identified saved customer resource's personal details.
    ///
    /// The request only overwrites the parameters included in the request, all other parameters will remain with their initially assigned values.
    pub async fn update(
        &self,
        customer_id: impl Into<String>,
        body: Option<UpdateCustomerBody>,
    ) -> Result<Customer, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/customers/{}", customer_id.into());
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
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: Customer = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let error: Error = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error: ErrorForbidden = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::NOT_FOUND => {
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
    /// List payment instruments
    ///
    /// Lists all payment instrument resources that are saved for an identified customer.
    pub async fn list_payment_instruments(
        &self,
        customer_id: impl Into<String>,
    ) -> Result<ListPaymentInstrumentsResponse, Box<dyn std::error::Error>> {
        let path = format!("/v0.1/customers/{}/payment-instruments", customer_id.into());
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
        match response.status() {
            reqwest::StatusCode::OK => {
                let data: ListPaymentInstrumentsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let error: Error = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error: ErrorForbidden = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::NOT_FOUND => {
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
    /// Deactivate a payment instrument
    ///
    /// Deactivates an identified card payment instrument resource for a customer.
    pub async fn deactivate_payment_instrument(
        &self,
        customer_id: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!(
            "/v0.1/customers/{}/payment-instruments/{}",
            customer_id.into(),
            token.into()
        );
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
        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED => {
                let error: Error = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error: ErrorForbidden = response.json().await?;
                Err(Box::new(error) as Box<dyn std::error::Error>)
            }
            reqwest::StatusCode::NOT_FOUND => {
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
