# Contributing

## Code generation

Regenerate the Rust SDK from `openapi.json` with:

```sh
cargo xtask generate
```

Generate the deterministic JSON catalog of developer-portal code samples with:

```sh
cargo xtask generate-code-samples
```

The default output is the ignored `code-samples.json` at the repository root.
Use `--output <path>` to write it elsewhere. Every sample is a complete Rust
program, and named OpenAPI request examples are emitted as separate catalog
entries. To regenerate twice and compile every program against the local SDK,
run:

```sh
cargo xtask validate-code-samples
```

Published releases synchronize this catalog to
`sumup/sumup-developer` as `src/codesamples/rust.json`.
