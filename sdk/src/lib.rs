//! # SumUp Rust SDK
//!
//! Official Rust SDK for the SumUp REST API.
//!
//! ## Quick Start
//!
//! ```no_run
//! use sumup::{error::SdkResult, Client};
//! type ListCheckoutsError = SdkResult<(), sumup::resources::common::Error>;
//!
//! #[tokio::main]
//! async fn main() -> ListCheckoutsError {
//!     // Create a client (reads SUMUP_API_KEY from environment)
//!     let client = Client::default();
//!
//!     // Call an API endpoint
//!     let checkouts = client.checkouts().list(Default::default()).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ### Authentication
//!
//! Set your API key via environment variable or explicitly:
//!
//! ```no_run
//! # use sumup::Client;
//! // From environment variable SUMUP_API_KEY
//! let client = Client::default();
//!
//! // Explicit token
//! let client = Client::default()
//!     .with_authorization("your_api_key");
//! ```
//!
//! ### Custom Configuration
//!
//! ```no_run
//! # use sumup::Client;
//! use std::time::Duration;
//!
//! let client = Client::default()
//!     .with_authorization("your_api_key")
//!     .with_timeout(Duration::from_secs(30));
//! ```
//!
//! ## Making API Calls
//!
//! The SDK organizes endpoints by tags:
//!
//! ```no_run
//! # use sumup::{Client, CheckoutCreateRequest, Currency, error::SdkResult};
//! # type Error = SdkResult<(), sumup::resources::common::Error>;
//! # async fn example(client: Client) -> Error {
//! // Create a checkout
//! let checkout = client.checkouts().create(Some(CheckoutCreateRequest {
//!     checkout_reference: "unique-ref".to_string(),
//!     amount: 10.0,
//!     currency: Currency::Eur,
//!     merchant_code: "MCODE".to_string(),
//!     description: None,
//!     return_url: None,
//!     customer_id: None,
//!     purpose: None,
//!     id: None,
//!     status: None,
//!     date: None,
//!     valid_until: None,
//!     transactions: None,
//!     redirect_url: None,
//! })).await;
//! if let Ok(order) = checkout {
//!     println!("created checkout {}", order.id.unwrap_or_default());
//! } else {
//!     eprintln!("create checkout request failed");
//! }
//!
//! // Transactions with query parameters
//! use sumup::resources::transactions::ListTransactionsParams;
//! let transactions = client.transactions().list_deprecated(ListTransactionsParams {
//!     limit: Some(10),
//!     ..Default::default()
//! }).await?;
//! let count = transactions.items.as_ref().map_or(0, |items| items.len());
//! println!("fetched {} historical transactions", count);
//! # Ok(())
//! # }
//! ```
//!
//! ## DateTime Support
//!
//! The SDK supports both [`chrono`](https://docs.rs/chrono) (default) and
//! [`jiff`](https://docs.rs/jiff) for datetime types:
//!
//! ```toml
//! # Use chrono (default)
//! [dependencies]
//! sumup = "0.0.1"
//!
//! # Use jiff instead
//! [dependencies]
//! sumup = { version = "0.0.1", default-features = false, features = ["jiff"] }
//! ```
//!
//! ## Error Handling
//!
//! All SDK calls return a [`SdkResult`] whose error side is a [`SdkError`]. When the
//! SumUp API responds with a non-success status, the SDK builds an
//! `SdkError::Api` containing the HTTP status and either the parsed error schema
//! (when documented) or the raw response body. You can inspect failures like
//! this:
//!
//! ```no_run
//! # use sumup::{Client, error::{ApiErrorBody, SdkError}};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::default();
//! match client.checkouts().list(Default::default()).await {
//!     Ok(checkouts) => println!("retrieved {} checkouts", checkouts.len()),
//!     Err(SdkError::Api(api)) => {
//!         eprintln!("request failed with status {}", api.status());
//!         match api.body() {
//!             ApiErrorBody::Parsed(details) => eprintln!("error payload: {:?}", details),
//!             ApiErrorBody::Raw(body) => eprintln!("raw body: {}", body),
//!         }
//!     }
//!     Err(SdkError::Network(err)) => return Err(Box::new(err)),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **chrono** (default): Use chrono for datetime types
//! - **jiff**: Use jiff for datetime types (mutually exclusive with chrono)
//!
//! ## Resources
//!
//! - [API Documentation](https://developer.sumup.com/docs/)
//! - [GitHub Repository](https://github.com/sumup/sumup-rs)

#![forbid(unsafe_code)]

pub mod client;
pub mod error;

#[allow(deprecated)]
#[allow(clippy::large_enum_variant)]
pub mod resources;

pub mod version;

pub mod datetime;

pub use crate::resources::*;
pub use client::Client;
pub use error::{ApiError, ApiErrorBody, SdkError, SdkResult};
pub use version::VERSION;
