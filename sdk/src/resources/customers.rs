// The contents of this file are generated; do not modify them.

//! Customers represent payers in your integration. Create a customer with your own `customer_id` to associate their personal details and saved payment instruments with your business records.
//!
//! To save a card, create a checkout for that customer with `purpose = SETUP_RECURRING_PAYMENT`, then process it with the payer's consent and mandate details. See the [tokenization guide](https://developer.sumup.com/online-payments/guides/tokenization-with-payment-sdk/).
//!
//! Use the Customers endpoints to create, retrieve, or update customer details and to list or deactivate saved payment instruments. For subsequent payments, process a new checkout with the saved instrument's `token` and its associated `customer_id`.
use super::common::*;
/// Saved payer details identified by the `customer_id` supplied by your integration. A customer can have saved payment instruments for subsequent payments.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Customer {
    /// Identifier you supply when creating the customer. Use an ID from your own system and retain it for subsequent customer, checkout, and saved-payment-instrument requests.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[builder(setter(into, doc = "Sets [`customer_id`](Customer::customer_id)."))]
    pub customer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`personal_details`](Customer::personal_details)."
        )
    )]
    pub personal_details: Option<PersonalDetails>,
}
/// Details of a customer validation error.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CustomerValidationError {
    /// Platform code for the error.
    ///
    /// Example: `INVALID`
    #[builder(setter(
        into,
        doc = "Sets [`error_code`](CustomerValidationError::error_code)."
    ))]
    pub error_code: String,
    /// Short description of the error.
    ///
    /// Example: `customer_id`
    #[builder(setter(
        into,
        doc = "Sets [`error_message`](CustomerValidationError::error_message)."
    ))]
    pub error_message: String,
    /// Unique identifier of this error occurrence.
    ///
    /// Example: `32a44c6c-85d3-49e8-86bf-a5bba98c4621`
    #[builder(setter(into, doc = "Sets [`instance`](CustomerValidationError::instance)."))]
    pub instance: String,
}
/// Details of a saved payment instrument.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PaymentInstrumentResponse {
    /// Indicates whether the payment instrument is active and can be used for payments. To deactivate it, send a `DELETE` request to the resource endpoint.
    ///
    /// Constraints:
    /// - read-only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`active`](PaymentInstrumentResponse::active)."
        )
    )]
    pub active: Option<bool>,
    /// Details of the payment card.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`card`](PaymentInstrumentResponse::card).")
    )]
    pub card: Option<PaymentInstrumentResponseCard>,
    /// The timestamp of when the payment instrument was created.
    ///
    /// Example: `2021-03-30T10:06:07.000+00:00`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`created_at`](PaymentInstrumentResponse::created_at)."
        )
    )]
    pub created_at: Option<crate::datetime::DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`mandate`](PaymentInstrumentResponse::mandate)."
        )
    )]
    pub mandate: Option<MandateResponse>,
    /// Token identifying the customer's saved payment card. Pass it as `token`, together with the associated `customer_id` and `payment_type = card`, when processing a checkout with this instrument.
    ///
    /// Constraints:
    /// - read-only
    ///
    /// Example: `bcfc8e5f-3b47-4cb9-854b-3b7a4cce7be3`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`token`](PaymentInstrumentResponse::token)."
        )
    )]
    pub token: Option<String>,
    /// Type of the payment instrument.
    ///
    /// Example: `card`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`r#type`](PaymentInstrumentResponse::r#type)."
        )
    )]
    pub r#type: Option<PaymentInstrumentResponseType>,
}
/// Details of the payment card.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PaymentInstrumentResponseCard {
    /// Last 4 digits of the payment card number.
    ///
    /// Constraints:
    /// - read-only
    /// - min length: 4
    /// - max length: 4
    ///
    /// Example: `3456`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`last_4_digits`](PaymentInstrumentResponseCard::last_4_digits)."
        )
    )]
    pub last_4_digits: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`r#type`](PaymentInstrumentResponseCard::r#type)."
        )
    )]
    pub r#type: Option<CardType>,
}
/// Type of the payment instrument.
///
/// Example: `card`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PaymentInstrumentResponseType {
    #[serde(rename = "card")]
    Card,
    #[serde(untagged)]
    Other(String),
}
/// Details of the customer.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CreateRequest {
    /// Identifier you supply when creating the customer. Use an ID from your own system and retain it for subsequent customer, checkout, and saved-payment-instrument requests.
    ///
    /// Example: `831ff8d4cd5958ab5670`
    #[builder(setter(into, doc = "Sets [`customer_id`](CreateRequest::customer_id)."))]
    pub customer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`personal_details`](CreateRequest::personal_details)."
        )
    )]
    pub personal_details: Option<PersonalDetails>,
}
/// Customer fields to update.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct UpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`personal_details`](UpdateRequest::personal_details)."
        )
    )]
    pub personal_details: Option<PersonalDetails>,
}
/// Returns the list of saved payment instruments for the customer.
pub type ListPaymentInstrumentsResponse = Vec<PaymentInstrumentResponse>;
use crate::client::Client;
#[derive(Debug, PartialEq)]
pub enum CreateErrorBody {
    BadRequest(crate::error::UnknownApiBody),
    Unauthorized(Problem),
    Forbidden(ErrorForbidden),
    Conflict(Error),
}
impl crate::error::ProblemDetails for CreateErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::BadRequest(body) => crate::error::ProblemDetails::problem(body),
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum GetErrorBody {
    Unauthorized(Problem),
    Forbidden(ErrorForbidden),
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
    Forbidden(ErrorForbidden),
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
pub enum ListPaymentInstrumentsErrorBody {
    Unauthorized(Problem),
    Forbidden(ErrorForbidden),
    NotFound(Error),
}
impl crate::error::ProblemDetails for ListPaymentInstrumentsErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum DeactivatePaymentInstrumentErrorBody {
    BadRequest(Error),
    Unauthorized(Problem),
    Forbidden(ErrorForbidden),
    NotFound(Error),
}
impl crate::error::ProblemDetails for DeactivatePaymentInstrumentErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        match self {
            Self::Unauthorized(problem) => Some(problem),
            _ => None,
        }
    }
}
/// Client for the Customers API endpoints.
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
    /// Creates a customer using the `customer_id` you supply. Choose an identifier that maps to the payer in your own system and reuse it when retrieving the customer or associating checkouts and saved payment instruments with them.
    ///
    /// Responses:
    /// - 201: Returns the customer resource.
    /// - 400: The request body is invalid.
    /// - 401: The request is not authorized.
    /// - 403: The request is authenticated but not permitted for this operation.
    /// - 409: A customer with the provided identifier already exists.
    pub async fn create(
        &self,
        body: CreateRequest,
    ) -> crate::error::SdkResult<Customer, CreateErrorBody> {
        let path = "/v0.1/customers";
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
                let data: Customer = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    Ok(CreateErrorBody::BadRequest(
                        crate::error::UnknownApiBody::from_bytes(raw),
                    ))
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
    /// Retrieve a customer
    ///
    /// Retrieves a saved customer using the `customer_id` you supplied when creating the customer.
    ///
    /// Responses:
    /// - 200: Returns the customer resource.
    /// - 401: The request is not authorized.
    /// - 403: The request is authenticated but not permitted for this operation.
    /// - 404: The requested resource does not exist.
    pub async fn get(
        &self,
        customer_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<Customer, GetErrorBody> {
        let path = format!("/v0.1/customers/{}", customer_id.as_ref());
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
                let data: Customer = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(GetErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ErrorForbidden>(raw).map(GetErrorBody::Forbidden)
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
    /// Update a customer
    ///
    /// Updates an identified saved customer resource's personal details.
    ///
    /// The request only overwrites the parameters included in the request, all other parameters will remain with their initially assigned values.
    ///
    /// Responses:
    /// - 200: Returns the customer resource.
    /// - 401: The request is not authorized.
    /// - 403: The request is authenticated but not permitted for this operation.
    /// - 404: The requested resource does not exist.
    pub async fn update(
        &self,
        customer_id: impl AsRef<str>,
        body: UpdateRequest,
    ) -> crate::error::SdkResult<Customer, UpdateErrorBody> {
        let path = format!("/v0.1/customers/{}", customer_id.as_ref());
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
                let data: Customer = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw).map(UpdateErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ErrorForbidden>(raw).map(UpdateErrorBody::Forbidden)
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
    /// List payment instruments
    ///
    /// Lists all payment instrument resources that are saved for an identified customer.
    ///
    /// Responses:
    /// - 200: Returns the list of saved payment instruments for the customer.
    /// - 401: The request is not authorized.
    /// - 403: The request is authenticated but not permitted for this operation.
    /// - 404: The requested resource does not exist.
    pub async fn list_payment_instruments(
        &self,
        customer_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<ListPaymentInstrumentsResponse, ListPaymentInstrumentsErrorBody>
    {
        let path = format!(
            "/v0.1/customers/{}/payment-instruments",
            customer_id.as_ref()
        );
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
                let data: ListPaymentInstrumentsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(ListPaymentInstrumentsErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ErrorForbidden>(raw)
                        .map(ListPaymentInstrumentsErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw)
                        .map(ListPaymentInstrumentsErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Deactivate a payment instrument
    ///
    /// Deactivates an identified card payment instrument resource for a customer.
    ///
    /// Responses:
    /// - 204: Returns an empty response body when the operation succeeds.
    /// - 400: The request is invalid.
    /// - 401: The request is not authorized.
    /// - 403: The request is authenticated but not permitted for this operation.
    /// - 404: The requested resource does not exist.
    pub async fn deactivate_payment_instrument(
        &self,
        customer_id: impl AsRef<str>,
        token: impl AsRef<str>,
    ) -> crate::error::SdkResult<(), DeactivatePaymentInstrumentErrorBody> {
        let path = format!(
            "/v0.1/customers/{}/payment-instruments/{}",
            customer_id.as_ref(),
            token.as_ref()
        );
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
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw)
                        .map(DeactivatePaymentInstrumentErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Problem>(raw)
                        .map(DeactivatePaymentInstrumentErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ErrorForbidden>(raw)
                        .map(DeactivatePaymentInstrumentErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<Error>(raw)
                        .map(DeactivatePaymentInstrumentErrorBody::NotFound)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
