# Repository Guidelines

## Project Structure

- Workspace crates: `sdk` (mostly generated from OpenAPI docs using `codegen` which is called from `xtask`), `xtask` (automation), `codegen` (OpenAPI helpers).
- Core client logic lives in `sdk/src/lib.rs` and `sdk/src/client.rs`; API groups are in `sdk/src/resources/*.rs` (one snake_case file per OpenAPI tag).
- `openapi.yaml` defines the surface area; generated code is committed in `sdk/src`.

## Build, Test, and Development

- `cargo check --workspace --all-features` validates all crates quickly.
- `cargo fmt --all` applies the enforced Rustfmt style (CI fails on drift).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` keeps lint debt at zero.
- `cargo test --workspace --all-features` runs unit, doc, and example tests.
- `cargo run --example checkout_card_reader` runs the representative end-to-end example.
- `cargo xtask generate` generates latest SDK from the OpenAPI specs using codegen (CI fails on drift)

## Code Style & Naming Conventions

- Target Rust 1.82, edition 2021, four-space indents, and no `unsafe`.
- Lean on existing client helpers for building requests; keep module-level docs current for new endpoints.
- `cargo fmt` and `cargo clippy` are the single sources of truth for formatting and linting.

## Testing

- Prefer colocated unit tests with `#[cfg(test)]` blocks; add broader coverage under `sdk/tests/` when needed.
- Mock HTTP interactions or serialize payloads to assert shapes - never call live SumUp services.
- Run `cargo test --doc` when updating examples or doctests to keep documentation snippets green.
- Aim for each new endpoint or serializer to have at least one round-trip test validating request/response structs.

## Commit & Pull Request Guidelines

- Follow Conventional Commits (e.g., `fix(sdk):`, `feat(resources):`, `chore:`) as the history demonstrates.
- Keep commits focused and rebased; avoid noisy merge commits in PR branches.
- PRs should summarize scope, note breaking changes, list verification commands, and link tracking issues.
- Add screenshots or external references when behavior impacts dashboards or partner integrations.

## API Spec & Code Generation

- Keep generated code up to date, run `cargo xtask generate` to refresh `sdk/src/resources` and `client.rs`.
- Inspect the diff for unintended churn before committing; regenerated files should not be hand-edited.
- Adjust generation behavior through the `codegen/` crate when the emitted code needs structural changes.
