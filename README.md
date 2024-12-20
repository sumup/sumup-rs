<div align="center">

# sumup-rs

[![SumUp's crates.io badge](https://img.shields.io/crates/v/sumup.svg)](https://crates.io/crates/sumup)
[![SumUp's docs.rs badge](https://docs.rs/sumup/badge.svg)](https://docs.rs/sumup)
[![CI Status](https://github.com/sumup/sumup-rs/workflows/CI/badge.svg)](https://github.com/sumup/sumup-rs/actions/workflows/ci.yml)
[![Documentation][docs-badge]](https://developer.sumup.com)
[![License](https://img.shields.io/github/license/sumup/sumup-rs)](./LICENSE)


</div>

_**IMPORTANT:** This SDK is under heavy development and subject to breaking changes._

The Rust SDK for the SumUp [API](https://developer.sumup.com).

## Requirements

Rust 1.82.0 or higher. We follow [Firefox MSRV policy](https://firefox-source-docs.mozilla.org/writing-rust-code/update-policy.html).

## Installation

Install with:

```sh
cargo add sumup
```

## Examples

You can find all examples under [examples/](/examples/). To run an example, use:

```sh
cargo run --example checkout_card_reader
```

[docs-badge]: https://img.shields.io/badge/SumUp-documentation-white.svg?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgY29sb3I9IndoaXRlIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogICAgPHBhdGggZD0iTTIyLjI5IDBIMS43Qy43NyAwIDAgLjc3IDAgMS43MVYyMi4zYzAgLjkzLjc3IDEuNyAxLjcxIDEuN0gyMi4zYy45NCAwIDEuNzEtLjc3IDEuNzEtMS43MVYxLjdDMjQgLjc3IDIzLjIzIDAgMjIuMjkgMFptLTcuMjIgMTguMDdhNS42MiA1LjYyIDAgMCAxLTcuNjguMjQuMzYuMzYgMCAwIDEtLjAxLS40OWw3LjQ0LTcuNDRhLjM1LjM1IDAgMCAxIC40OSAwIDUuNiA1LjYgMCAwIDEtLjI0IDcuNjlabTEuNTUtMTEuOS03LjQ0IDcuNDVhLjM1LjM1IDAgMCAxLS41IDAgNS42MSA1LjYxIDAgMCAxIDcuOS03Ljk2bC4wMy4wM2MuMTMuMTMuMTQuMzUuMDEuNDlaIiBmaWxsPSJjdXJyZW50Q29sb3IiLz4KPC9zdmc+
