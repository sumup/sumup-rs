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
//! # use sumup::{Authorization, Client};
//! // From environment variable SUMUP_API_KEY
//! let client = Client::default();
//!
//! // Explicit token
//! let client = Client::default()
//!     .with_authorization(Authorization::api_key("your_api_key"));
//! ```
//!
//! ### Custom Configuration
//!
//! ```no_run
//! # use sumup::{Authorization, Client};
//! use std::time::Duration;
//!
//! let client = Client::default()
//!     .with_authorization(Authorization::api_key("your_api_key"))
//!     .with_timeout(Duration::from_secs(30));
//! ```
//!
//! To customize reqwest directly, build a configured `reqwest::Client` and pass
//! it to [`Client::with_client`]. Start from [`Client::http_client_builder`] to
//! keep the SDK's default headers:
//!
//! ```no_run
//! # use sumup::Client;
//! # fn build_client() -> Result<Client, reqwest::Error> {
//! let http_client = Client::http_client_builder()
//!     .pool_max_idle_per_host(8)
//!     .build()?;
//!
//! let client = Client::default().with_client(http_client);
//! # Ok(client)
//! # }
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
//! let checkout = client.checkouts().create(checkouts::CreateRequest {
//!     checkout_reference: "unique-ref".to_string(),
//!     amount: 10.0,
//!     currency: Currency::EUR,
//!     merchant_code: "MCODE".to_string(),
//!     description: None,
//!     return_url: None,
//!     customer_id: None,
//!     purpose: None,
//!     valid_until: None,
//!     redirect_url: None,
//!     hosted_checkout: None,
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
//! sumup = "0.5"
//!
//! # Use jiff instead
//! [dependencies]
//! sumup = { version = "0.5", default-features = false, features = ["jiff", "reqwest-default-tls"] }
//!
//! # Use rustls instead of reqwest's default TLS backend
//! [dependencies]
//! sumup = { version = "0.5", default-features = false, features = ["chrono", "reqwest-rustls-tls"] }
//! ```
//!
//! ## Error Handling
//!
//! Failed calls return a [`SdkError`]. Use `problem()` to read typed Problem details
//! when the API provides them:
//!
//! ```no_run
//! # use sumup::Client;
//! # async fn example() {
//! let client = Client::default();
//! if let Err(error) = client.checkouts().get("your-checkout-id").await {
//!     if let Some(problem) = error.problem() {
//!         eprintln!("{}: {}", problem.r#type, problem.detail.as_deref().unwrap_or("Request failed"));
//!     }
//!     eprintln!("{error}");
//! }
//! # }
//! ```
//!
//! Use [`SdkError::status`], [`SdkError::headers`], and [`SdkError::raw_body`]
//! to inspect the HTTP response. [`SdkError::body`] provides other typed error details.
//!
//! ## Features
//!
//! - **chrono** (default): Use chrono for datetime types
//! - **jiff**: Use jiff for datetime types (mutually exclusive with chrono)
//! - **reqwest-default-tls** (default): Use reqwest's default TLS backend
//! - **reqwest-rustls-tls**: Use reqwest's rustls TLS backend
//!
//! ## Resources
//!
//! - [API Documentation](https://developer.sumup.com/docs/)
//! - [GitHub Repository](https://github.com/sumup/sumup-rs)

#![forbid(unsafe_code)]
// Generated endpoint methods intentionally retain typed API response bodies in
// their error variants, which can exceed Clippy's heuristic size threshold.
#![allow(clippy::result_large_err)]

mod string_or_number;

pub mod api_version;
pub mod auth;
pub mod client;
pub mod datetime;
pub mod error;
mod event;
pub mod events;
mod events_handler;
pub mod nullable;
pub mod secret;
pub mod version;

#[allow(deprecated)]
#[allow(clippy::large_enum_variant)]
pub mod resources;

pub use crate::resources::*;
pub use auth::Authorization;
pub use client::Client;
pub use error::{
    ApiErrorResponse, ProblemDetails, ResponseError, SdkError, SdkResult, UnknownApiBody,
};
pub use nullable::Nullable;
pub use secret::Secret;
pub use version::VERSION;
