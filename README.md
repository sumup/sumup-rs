<div align="center">

# SumUp Rust SDK

[![SumUp's crates.io badge](https://img.shields.io/crates/v/sumup.svg)](https://crates.io/crates/sumup)
[![SumUp's docs.rs badge](https://docs.rs/sumup/badge.svg)](https://docs.rs/sumup)
[![CI Status](https://github.com/sumup/sumup-rs/workflows/CI/badge.svg)](https://github.com/sumup/sumup-rs/actions/workflows/ci.yml)
[![Documentation][docs-badge]](https://developer.sumup.com)
[![License](https://img.shields.io/github/license/sumup/sumup-rs)](./LICENSE)

</div>

_**IMPORTANT:** This SDK is under heavy development and subject to breaking changes._

The [Rust](https://rust-lang.org/) SDK for the SumUp [API](https://developer.sumup.com).

## Requirements

Rust 1.82.0 or higher. We follow [Firefox MSRV policy](https://firefox-source-docs.mozilla.org/writing-rust-code/update-policy.html).

## Installation

Install with:

```sh
cargo add sumup
```

## Quick Start

```rust
use sumup::Client;

#[tokio::main]
async fn main() {
    // Initialize the client (reads SUMUP_API_KEY from environment)
    let client = Client::default();

    // List existing checkouts
    let checkouts = client
        .checkouts()
        .list(Default::default())
        .await
        .expect("list checkouts");
    println!("retrieved {} checkouts", checkouts.len());
}
```

## Authentication

```rust
use sumup::{Authorization, Client};

// Read SUMUP_API_KEY from the environment
let client = Client::default();

// Or set an explicit API key
let client = Client::default()
    .with_authorization(Authorization::api_key("your_api_key"));
```

## Examples

You can find all examples under [examples/](/examples/). To run an example, use:

```sh
cargo run --example checkout_card_reader
```

[docs-badge]: https://img.shields.io/badge/SumUp-documentation-white.svg?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgY29sb3I9IndoaXRlIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogICAgPHBhdGggZD0iTTIyLjI5IDBIMS43Qy43NyAwIDAgLjc3IDAgMS43MVYyMi4zYzAgLjkzLjc3IDEuNyAxLjcxIDEuN0gyMi4zYy45NCAwIDEuNzEtLjc3IDEuNzEtMS43MVYxLjdDMjQgLjc3IDIzLjIzIDAgMjIuMjkgMFptLTcuMjIgMTguMDdhNS42MiA1LjYyIDAgMCAxLTcuNjguMjQuMzYuMzYgMCAwIDEtLjAxLS40OWw3LjQ0LTcuNDRhLjM1LjM1IDAgMCAxIC40OSAwIDUuNiA1LjYgMCAwIDEtLjI0IDcuNjlabTEuNTUtMTEuOS03LjQ0IDcuNDVhLjM1LjM1IDAgMCAxLS41IDAgNS42MSA1LjYxIDAgMCAxIDcuOS03Ljk2bC4wMy4wM2MuMTMuMTMuMTQuMzUuMDEuNDlaIiBmaWxsPSJjdXJyZW50Q29sb3IiLz4KPC9zdmc+
