# Repository Guidelines

## Project Structure & Module Organization
Initialize the workspace with `cargo init` (or `cargo init --lib`) to generate `Cargo.toml`, `src/main.rs`, and supporting folders. Application code lives under `src/`, with additional binaries in `src/bin/`, integration tests in `tests/`, and optional examples or benches in `examples/` and `benches/`. Build output stays in `target/`; keep it out of version control. Group modules by domain rather than layer, and re-export shared types from `lib.rs` so binaries can reuse them without deep relative paths.

## Build, Test, and Development Commands
- `cargo build` / `cargo build --release`: compile the crate in debug or optimized mode. Use release builds before benchmarking.
- `cargo run -- [args]`: compile and execute the main binary with optional CLI arguments.
- `cargo fmt` and `cargo clippy -- -D warnings`: enforce formatting and linting before every commit.
- `cargo doc --open`: confirm public APIs remain documented after refactors.

## Coding Style & Naming Conventions
Stick to Rust’s defaults: four-space indentation, snake_case for functions/modules, CamelCase for types, SCREAMING_SNAKE_CASE for consts. Keep modules short; split files when they exceed ~300 lines or mix concerns. Use `rustfmt.toml` only if the default style blocks readability. Derive common traits (`Debug`, `Clone`, `PartialEq`) for domain structs to simplify testing. Treat `todo!()` as temporary—replace with clear implementations or error handling before merging.

## Testing Guidelines
Place fast unit tests inline with the code using `#[cfg(test)]` modules and integration tests under `tests/`. Name cases after the behavior under verification (e.g., `serializes_minimal_header`). Run `cargo test`, `cargo test -- --ignored`, and `cargo clippy` in CI or locally before opening a PR. When adding features, include regression tests or property checks that fail without the new work. Document test-only fixtures inside `tests/common/mod.rs` to keep helpers reusable.

## Commit & Pull Request Guidelines
Use concise, present-tense summaries (`init: scaffold cargo crate`). Reference issue IDs when applicable (`fix: handle empty payload (#42)`). Each PR should describe intent, testing evidence (`cargo test`, `cargo fmt`), and any follow-up work. Attach screenshots or CLI transcripts when behavior is user-facing. Keep PRs focused; split large refactors from feature work. Request review once CI is green and the branch rebases cleanly onto `main`.
