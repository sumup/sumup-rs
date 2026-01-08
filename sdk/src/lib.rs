//! # SumUp Rust SDK
//!
//! Official Rust SDK for the SumUp REST API.
//!
//! ## Quick Start
//!
//! ```no_run
//! use sumup::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a client (reads SUMUP_API_KEY from environment)
//!     let client = Client::default();
//!
//!     // Call an API endpoint
//!     let checkouts = client
//!         .checkouts()
//!         .list(Default::default())
//!         .await
//!         .expect("list checkouts request failed");
//!     println!("found {} checkouts", checkouts.len());
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
//! # use sumup::{Client, Currency, checkouts};
//! # async fn example(client: Client) {
//! // Create a checkout
//! let checkout = client.checkouts().create(checkouts::CheckoutCreateRequest {
//!     checkout_reference: "unique-ref".to_string(),
//!     amount: 10.0,
//!     currency: Currency::EUR,
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
//! })
//! .await
//! .expect("create checkout");
//! println!("created checkout {}", checkout.id.unwrap_or_default());
//!
//! // Transactions with query parameters
//! use sumup::resources::transactions::ListParams;
//! let transactions = client
//!     .transactions()
//!     .list(
//!         "MERCHANT_CODE",
//!         ListParams {
//!             limit: Some(10),
//!             ..Default::default()
//!         },
//!     )
//!     .await
//!     .expect("list transactions");
//! let count = transactions.items.as_ref().map_or(0, |items| items.len());
//! println!("fetched {} historical transactions", count);
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
//! `SdkError::Api` containing an endpoint-specific payload (e.g. a `Unauthorized`
//! enum variant). Any undocumented status codes fall back to
//! `SdkError::Unexpected`, which preserves the HTTP status and best-effort body
//! parsing. You can inspect failures like this:
//!
//! ```no_run
//! # use sumup::{Client, error::SdkError};
//! # use sumup::resources::checkouts::ListErrorBody;
//! # async fn example() {
//! let client = Client::default();
//! match client.checkouts().list(Default::default()).await {
//!     Ok(checkouts) => println!("retrieved {} checkouts", checkouts.len()),
//!     Err(SdkError::Api(body)) => match body {
//!         ListErrorBody::Unauthorized(details) => eprintln!("unauthorized: {:?}", details),
//!     },
//!     Err(SdkError::Unexpected(status, body)) => {
//!         eprintln!("unexpected {} response: {}", status, body);
//!     }
//!     Err(SdkError::Network(err)) => panic!("network error: {}", err),
//! }
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
pub mod datetime;
pub mod error;
pub mod nullable;
pub mod secret;
pub mod version;

#[allow(deprecated)]
#[allow(clippy::large_enum_variant)]
pub mod resources;

pub use crate::resources::*;
pub use client::Client;
pub use error::{SdkError, SdkResult, UnknownApiBody};
pub use nullable::Nullable;
pub use secret::Secret;
pub use version::VERSION;
