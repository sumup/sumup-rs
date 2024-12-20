/// The version of the SDK
pub const VERSION: &str = "0.0.1";

/// Returns the user agent string for SDK requests
pub fn user_agent() -> String {
    format!("sumup-rs/v{}", VERSION)
}
