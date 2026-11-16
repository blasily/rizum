# Project instructions

## Project overview

Rizum is an early-development Rust library/framework for rhythm-game chart data. The crate uses Rust 2024 and Serde/serde_json. The README says it is not yet usable as a complete framework, so avoid describing unfinished behavior as supported.

## Project layout

- `src/lib.rs` declares the internal modules and re-exports the public chart types.
- `src/chart.rs`, `src/metadata.rs`, `src/timing.rs`, and `src/note.rs` define the chart data model and timing conversions.
- `charts/` contains JSON chart examples; `tests/` contains integration tests for chart serialization and deserialization.
- `Cargo.toml` and `Cargo.lock` define the crate and its dependencies.

## Development guidance

- Keep the public API deliberate: update `src/lib.rs` re-exports when adding public types or modules.
- Keep chart JSON serialization compatible with the Rust data model and update `charts/` examples and integration tests when that format changes.
- Put focused unit tests next to implementation code and crate-level behavior tests in `tests/`.
- Use standard Rust formatting. Useful local commands are `cargo fmt --check`, `cargo check`, and `cargo test`.
- Do not commit generated build output from `target/`.

## GitHub

Do not push anything to GitHub without me stating it explicitly.
