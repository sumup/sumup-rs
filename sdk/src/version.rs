/// The version of the SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the user agent string for SDK requests
pub fn user_agent() -> String {
    format!("sumup-rs/v{}", VERSION)
}
