---
applyTo: "**/*.rs"
---

# Rust Coding Guidelines

## General
- Use idiomatic Rust code.
- Prefer `Result` over `panic!` for error handling.
- Use `anyhow` for application-level error handling.
- Use `clap` for CLI argument parsing.

## Documentation
- All public structs and functions must have documentation comments (`///`).
- Include examples in documentation where possible.

## Formatting
- Run `cargo fmt` before committing.
