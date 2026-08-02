#![forbid(unsafe_code)]

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use oas3::Spec as OpenAPI;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "build tasks")]
enum Xtask {
    #[command(about = "generate SDK code")]
    Generate,
    #[command(about = "generate developer-portal Rust code samples")]
    GenerateCodeSamples {
        #[arg(short, long, default_value = "code-samples.json")]
        output: PathBuf,
    },
    #[command(about = "compile every generated code sample against the local SDK")]
    ValidateCodeSamples,
}

fn main() -> Result<(), String> {
    let xtask = Xtask::parse();

    match xtask {
        Xtask::Generate => generate(),
        Xtask::GenerateCodeSamples { output } => generate_code_samples(&output),
        Xtask::ValidateCodeSamples => validate_code_samples(),
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|error| format!("Failed to resolve CARGO_MANIFEST_DIR: {error}"))?,
    )
    .parent()
    .map(Path::to_path_buf)
    .ok_or_else(|| "Failed to resolve workspace root".to_string())
}

fn load_spec(root_path: &Path) -> Result<OpenAPI, String> {
    let spec_path = root_path.join("openapi.json");
    let file = File::open(&spec_path)
        .map_err(|error| format!("Failed to open {}: {error}", spec_path.display()))?;
    let spec: OpenAPI = serde_json::from_reader(file)
        .map_err(|error| format!("Failed to parse {}: {error}", spec_path.display()))?;
    spec.validate_version()
        .map_err(|error| format!("Failed to parse {}: {error}", spec_path.display()))?;
    Ok(spec)
}

fn generate() -> Result<(), String> {
    let start = Instant::now();
    let root_path = workspace_root()?;

    println!("[generate sdk] loading OpenAPI spec ...");
    std::io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {e}"))?;

    let spec = load_spec(&root_path)?;

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

fn generate_code_samples(output: &Path) -> Result<(), String> {
    let root = workspace_root()?;
    let catalog = build_code_sample_catalog(&root)?;
    let output = if output.is_absolute() {
        output.to_path_buf()
    } else {
        root.join(output)
    };
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create {}: {error}", parent.display()))?;
    }
    let mut encoded = serde_json::to_vec_pretty(&catalog)
        .map_err(|error| format!("Failed to encode code samples: {error}"))?;
    encoded.push(b'\n');
    std::fs::write(&output, encoded)
        .map_err(|error| format!("Failed to write {}: {error}", output.display()))?;
    println!(
        "[generate code samples] wrote {} samples to {}",
        catalog.samples.len(),
        output.display()
    );
    Ok(())
}

fn build_code_sample_catalog(root: &Path) -> Result<codegen::CodeSampleCatalog, String> {
    let spec = load_spec(root)?;
    codegen::generate_code_samples(&spec, sdk_version(root)?)
}

fn sdk_version(root: &Path) -> Result<String, String> {
    let manifest_path = root.join("sdk/Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .map_err(|error| format!("Failed to read {}: {error}", manifest_path.display()))?;
    parse_sdk_version(&manifest).ok_or_else(|| {
        format!(
            "Failed to find package version in {}",
            manifest_path.display()
        )
    })
}

fn parse_sdk_version(manifest: &str) -> Option<String> {
    let package = manifest
        .split_once("[package]")
        .map(|(_, package)| package)
        .unwrap_or(manifest)
        .split_once("\n[")
        .map_or(manifest, |(package, _)| package);
    package
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("version = ")
                .map(|version| version.trim_matches('"').to_string())
        })
        .filter(|version| !version.is_empty())
}

fn validate_code_samples() -> Result<(), String> {
    let root = workspace_root()?;
    let first = build_code_sample_catalog(&root)?;
    let second = build_code_sample_catalog(&root)?;
    let first_json = serde_json::to_vec(&first)
        .map_err(|error| format!("Failed to encode code samples: {error}"))?;
    let second_json = serde_json::to_vec(&second)
        .map_err(|error| format!("Failed to encode code samples: {error}"))?;
    if first_json != second_json {
        return Err("Code sample generation is not deterministic".to_string());
    }
    if first.samples.is_empty() {
        return Err("Code sample catalog is empty".to_string());
    }
    if first
        .samples
        .windows(2)
        .any(|samples| samples[0].id >= samples[1].id)
    {
        return Err("Code sample IDs are duplicated or unsorted".to_string());
    }

    compile_code_samples(&root, &first.samples)?;
    println!(
        "[validate code samples] compiled {} samples against the local SDK",
        first.samples.len()
    );
    Ok(())
}

fn compile_code_samples(root: &Path, samples: &[codegen::CodeSample]) -> Result<(), String> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("Failed to read system time: {error}"))?
        .as_nanos();
    let temporary = std::env::temp_dir().join(format!(
        "sumup-rust-code-samples-{}-{unique}",
        std::process::id()
    ));
    let source_dir = temporary.join("src/bin");
    std::fs::create_dir_all(&source_dir)
        .map_err(|error| format!("Failed to create {}: {error}", source_dir.display()))?;
    let cleanup = TemporaryDirectory(temporary.clone());

    let sdk_path = root.join("sdk").display().to_string().replace('\\', "\\\\");
    let manifest = format!(
        "[package]\nname = \"sumup-generated-code-samples\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n\n[dependencies]\nsumup = {{ path = \"{sdk_path}\", default-features = false, features = [\"chrono\"] }}\nserde_json = \"1.0\"\ntokio = {{ version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }}\n"
    );
    std::fs::write(temporary.join("Cargo.toml"), manifest)
        .map_err(|error| format!("Failed to write sample manifest: {error}"))?;
    for (index, sample) in samples.iter().enumerate() {
        std::fs::write(
            source_dir.join(format!("sample_{index:03}.rs")),
            &sample.sample,
        )
        .map_err(|error| format!("Failed to write sample {}: {error}", sample.id))?;
    }

    let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
        .args(["check", "--quiet", "--bins", "--manifest-path"])
        .arg(temporary.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()
        .map_err(|error| format!("Failed to run cargo check for generated samples: {error}"))?;
    drop(cleanup);
    if !output.status.success() {
        return Err(format!(
            "Generated samples failed to compile:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

struct TemporaryDirectory(PathBuf);

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_sdk_version_from_package_manifest() {
        let manifest = "[package]\nname = \"sumup\"\nversion = \"1.2.3\"\n\n[dependencies]\n";
        assert_eq!(parse_sdk_version(manifest).as_deref(), Some("1.2.3"));
    }
}
