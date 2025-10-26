#![forbid(unsafe_code)]

pub mod client;

#[allow(deprecated)]
pub mod resources;

pub mod version;

pub mod datetime;

pub use crate::resources::*;
pub use client::Client;
pub use version::VERSION;
