#![forbid(unsafe_code)]

use std::process::Command;

fn main() {
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|version| version.trim().to_string());

    if let Some(version) = rustc_version {
        println!("cargo:rustc-env=SUMUP_RUSTC_VERSION={}", version);
    }
}
