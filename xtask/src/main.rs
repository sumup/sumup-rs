#![forbid(unsafe_code)]

use std::{fs::File, io::Write, path::PathBuf, time::Instant};

use clap::Parser;
use heck::ToSnakeCase;
use openapiv3::OpenAPI;
use quote::quote;

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

    println!("[generate sdk] analyzing tags and operations ...");
    std::io::stdout().flush().unwrap();

    // Group operations by tags
    let schemas_by_tag = codegen::collect_schemas_by_tag(&spec)?;

    println!(
        "[generate sdk] found {} tags, {} common schemas",
        schemas_by_tag.tag_schemas.len(),
        schemas_by_tag.common_schemas.len()
    );
    std::io::stdout().flush().unwrap();

    let mut out_path = root_path.clone();
    out_path.push("sdk");
    out_path.push("src");

    // Create resources directory
    let mut resources_path = out_path.clone();
    resources_path.push("resources");
    std::fs::create_dir_all(&resources_path)
        .map_err(|e| format!("Failed to create resources directory: {}", e))?;

    // Generate common.rs for shared schemas
    if !schemas_by_tag.common_schemas.is_empty() {
        println!(
            "[generate sdk] generating common.rs with {} shared schemas ({} error schemas) ...",
            schemas_by_tag.common_schemas.len(),
            schemas_by_tag.common_error_schemas.len()
        );
        std::io::stdout().flush().unwrap();
        codegen::generate_common_file(&out_path, &spec, &schemas_by_tag)?;
    }

    // Sort tags alphabetically for deterministic output
    let mut sorted_tags: Vec<_> = schemas_by_tag.tag_schemas.iter().collect();
    sorted_tags.sort_by_key(|(tag, _)| *tag);

    // Generate a file for each tag (with schemas and client)
    for (tag, tag_data) in sorted_tags {
        println!(
            "[generate sdk] generating {} with {} schemas ({} error schemas) ...",
            tag,
            tag_data.all_schemas.len(),
            tag_data.error_schemas.len()
        );
        std::io::stdout().flush().unwrap();

        let schema_tokens = codegen::generate_structs_for_schemas(
            &spec,
            &tag_data.all_schemas,
            &tag_data.error_schemas,
        )?;
        let body_tokens = codegen::generate_operation_bodies(&spec, tag)?;
        let client_tokens = codegen::generate_tag_client(&spec, tag)?;

        // Add import for common schemas if this tag references them in schemas or operations
        let use_common = if !schemas_by_tag.common_schemas.is_empty()
            && (codegen::does_reference_common_schemas(
                &spec,
                &tag_data.all_schemas,
                &schemas_by_tag.common_schemas,
            ) || codegen::does_tag_operations_reference_common(
                &spec,
                tag,
                &schemas_by_tag.common_schemas,
            )) {
            quote! {
                use super::common::*;
            }
        } else {
            quote! {}
        };

        // Combine schemas, bodies, and client
        let combined_tokens = quote! {
            #use_common

            #schema_tokens

            #body_tokens

            #client_tokens
        };

        let contents = codegen::format_generated_code(combined_tokens);

        let file_name = format!("{}.rs", tag.to_snake_case());
        let mut tag_out_path = resources_path.clone();
        tag_out_path.push(&file_name);

        std::fs::write(&tag_out_path, &contents)
            .map_err(|e| format!("Failed to write {}: {}", file_name, e))?;
    }

    // Generate client.rs
    println!("[generate sdk] generating client.rs ...");
    std::io::stdout().flush().unwrap();
    codegen::generate_client_file(&out_path, &schemas_by_tag.tag_schemas)?;

    // Generate resources/mod.rs to export all modules
    codegen::generate_mod_file(&out_path, &schemas_by_tag)?;

    println!("[generate sdk] ... done");

    let duration = Instant::now().duration_since(start).as_micros();
    println!(
        "[generate sdk] took {}.{:03}s",
        duration / 1_000_000,
        (duration % 1_000_000) / 1_000
    );

    Ok(())
}
