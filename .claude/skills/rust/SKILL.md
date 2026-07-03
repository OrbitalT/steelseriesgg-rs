---
name: rust
description: Rust conventions for steelseriesgg-rs — error handling, unsafe, feature-gated commands. Use when writing or refactoring any .rs file or Cargo.toml.
license: Apache-2.0
compatibility: Designed for Claude Code
allowed-tools: Read, Grep, Glob, Bash
user-invocable: false
metadata:
  version: "3.0.0"
  category: "language"
  status: "active"
  updated: "2026-07-03"
---

# Rust Development

## When to use

- `.rs` files and `Cargo.toml`
- Ownership/lifetime, trait, or performance-sensitive work

## Defaults

- Library boundaries (`src/error.rs` and below): `crate::error::Error` via `thiserror`. `main.rs` and `src/bin/*`: `anyhow` with `.context()`.
- No `unwrap`/`expect` outside `#[cfg(test)]`. Propagate with `?`.
- `unsafe` blocks require an inline `// SAFETY:` comment naming the concrete invariant (see `.claude/rules/unsafe-blocks.md`).
- Favor simple ownership flows (`&T` borrows) over complex lifetimes.

## Quick workflow

1. Identify crate boundaries and public APIs (`src/lib.rs` re-exports).
2. Implement the minimal change with clear types/errors.
3. Add/update tests for changed behavior.
4. Run fmt, clippy, and tests — with `--features <flag>` if the change touches feature-gated code.

## Commands

Match `.github/workflows/ci.yml` exactly — this repo does not use `--all-features` (the `audio` and `sonar` features are independent and CI tests them separately; `audio` needs `libpulse-dev`).

- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --all-targets --locked -- -D warnings` (add `--features sonar` or `--features audio` for feature-gated changes)
- Test: `cargo test --locked` (add `--features sonar` for feature-gated changes; `audio` is excluded from CI test job)
- Release build: `cargo build --release --locked`

For the full matrix across all feature combinations, use the `ci-matrix` skill instead of running these piecemeal.

## Implementation checklist

- `Result<T, E>` and `?` used consistently; no ignored `Result`s.
- No mutable global state.
- New dependencies justified (avoid heavy transitive deps); `hidapi` version pin (`=2.6.6`) never changed without explicit task justification.
- Experimental protocol code labeled per `.claude/rules/experimental-protocol.md`.

## Validation checklist

- `cargo fmt --all -- --check` clean.
- `cargo clippy --all-targets --locked -- -D warnings` clean (relevant feature flags included).
- Tests pass.
