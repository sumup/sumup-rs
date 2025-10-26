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
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
//! # use sumup::{Client, CheckoutCreateRequest, Currency};
//! # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
//! // Checkouts
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
//! })).await?;
//!
//! // Transactions with query parameters
//! use sumup::resources::transactions::ListTransactionsParams;
//! let transactions = client.transactions().list_deprecated(ListTransactionsParams {
//!     limit: Some(10),
//!     ..Default::default()
//! }).await?;
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

#[allow(deprecated)]
pub mod resources;

pub mod version;

pub mod datetime;

pub use crate::resources::*;
pub use client::Client;
pub use version::VERSION;
