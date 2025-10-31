#![forbid(unsafe_code)]

use std::{fs::File, io::Write, path::PathBuf, time::Instant};

use clap::Parser;
use openapiv3::OpenAPI;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "build tasks")]
enum Xtask {
    #[command(about = "generate SDK code")]
    Generate,
}

fn main() -> Result<(), String> {
    let xtask = Xtask::parse();

    match xtask {
        Xtask::Generate => generate(),
    }
}

fn generate() -> Result<(), String> {
    let start = Instant::now();
    let xtask_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_path = xtask_path.parent().unwrap().to_path_buf();
    let mut spec_path = root_path.clone();
    spec_path.push("openapi.yaml");

    println!("[generate sdk] loading OpenAPI spec ...");
    std::io::stdout().flush().unwrap();

    let file = File::open(&spec_path).map_err(|e| format!("Failed to open spec: {}", e))?;
    let spec: OpenAPI = serde_yaml::from_reader(file)
        .map_err(|e| format!("Failed to parse OpenAPI spec: {}", e))?;

    let mut out_path = root_path.clone();
    out_path.push("sdk");

    let generator = codegen::Generator::new(spec, out_path)?;
    generator.generate()?;

    let duration = Instant::now().duration_since(start).as_micros();
    println!(
        "[generate sdk] took {}.{:03}s",
        duration / 1_000_000,
        (duration % 1_000_000) / 1_000
    );

    Ok(())
}
