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
    let xtask_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|e| format!("Failed to resolve CARGO_MANIFEST_DIR: {e}"))?,
    );
    let root_path = xtask_path
        .parent()
        .ok_or_else(|| {
            format!(
                "Failed to resolve workspace root from {}",
                xtask_path.display()
            )
        })?
        .to_path_buf();
    let mut spec_path = root_path.clone();
    spec_path.push("openapi.json");

    println!("[generate sdk] loading OpenAPI spec ...");
    std::io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {e}"))?;

    let file = File::open(&spec_path).map_err(|e| format!("Failed to open spec: {}", e))?;
    let raw_spec: serde_json::Value = serde_json::from_reader(file)
        .map_err(|e| format!("Failed to parse OpenAPI spec: {}", e))?;
    let mut parser_spec = raw_spec.clone();
    normalize_nullable_type_arrays(&mut parser_spec);
    let spec: OpenAPI = serde_json::from_value(parser_spec)
        .map_err(|e| format!("Failed to parse OpenAPI spec: {}", e))?;

    let mut out_path = root_path.clone();
    out_path.push("sdk");

    let generator = codegen::Generator::new(spec, raw_spec, out_path)?;
    generator.generate()?;

    let duration = Instant::now().duration_since(start).as_micros();
    println!(
        "[generate sdk] took {}.{:03}s",
        duration / 1_000_000,
        (duration % 1_000_000) / 1_000
    );

    Ok(())
}

fn normalize_nullable_type_arrays(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(object) => {
            if let Some(type_value) = object.get_mut("type") {
                if let serde_json::Value::Array(types) = type_value {
                    let non_null_types: Vec<_> = types
                        .iter()
                        .filter_map(serde_json::Value::as_str)
                        .filter(|candidate| *candidate != "null")
                        .map(str::to_owned)
                        .collect();

                    if non_null_types.len() == 1
                        && types
                            .iter()
                            .any(|candidate| candidate.as_str() == Some("null"))
                    {
                        *type_value = serde_json::Value::String(non_null_types[0].clone());
                        object.insert("nullable".to_string(), serde_json::Value::Bool(true));
                    }
                }
            }

            for child in object.values_mut() {
                normalize_nullable_type_arrays(child);
            }
        }
        serde_json::Value::Array(values) => {
            for child in values {
                normalize_nullable_type_arrays(child);
            }
        }
        _ => {}
    }
}
