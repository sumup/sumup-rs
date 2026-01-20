/// The version of the SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the user agent string for SDK requests
pub fn user_agent() -> String {
    format!("sumup-rs/v{}", VERSION)
}

/// Returns the runtime headers for SDK requests
pub fn runtime_info() -> Vec<(&'static str, String)> {
    vec![
        (
            "X-Sumup-Api-Version",
            crate::api_version::API_VERSION.to_string(),
        ),
        ("X-SumUp-Lang", "rust".to_string()),
        ("X-SumUp-Package-Version", VERSION.to_string()),
        ("X-SumUp-Os", std::env::consts::OS.to_string()),
        ("X-SumUp-Arch", runtime_arch()),
        ("X-SumUp-Runtime", "rust".to_string()),
        ("X-SumUp-Runtime-Version", rustc_version()),
    ]
}

fn rustc_version() -> String {
    option_env!("SUMUP_RUSTC_VERSION")
        .unwrap_or("unknown")
        .to_string()
}

fn runtime_arch() -> String {
    match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "x86" | "i686" => "x86",
        "aarch64" => "arm64",
        "arm" => "arm",
        other => other,
    }
    .to_string()
}
