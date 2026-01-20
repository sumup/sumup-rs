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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_info_includes_expected_headers() {
        let headers = runtime_info();

        let names: Vec<&str> = headers.iter().map(|(name, _)| *name).collect();
        assert!(names.contains(&"X-Sumup-Api-Version"));
        assert!(names.contains(&"X-SumUp-Lang"));
        assert!(names.contains(&"X-SumUp-Package-Version"));
        assert!(names.contains(&"X-SumUp-Os"));
        assert!(names.contains(&"X-SumUp-Arch"));
        assert!(names.contains(&"X-SumUp-Runtime"));
        assert!(names.contains(&"X-SumUp-Runtime-Version"));
    }

    #[test]
    fn runtime_info_uses_normalized_arch_value() {
        let expected = match std::env::consts::ARCH {
            "x86_64" => "x86_64",
            "x86" | "i686" => "x86",
            "aarch64" => "arm64",
            "arm" => "arm",
            other => other,
        };

        let arch = runtime_info()
            .into_iter()
            .find(|(name, _)| *name == "X-SumUp-Arch")
            .map(|(_, value)| value)
            .expect("runtime info should include X-SumUp-Arch");

        assert_eq!(arch, expected);
    }
}
