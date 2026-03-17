---
description: 'Rust coding standards: safety, zero-cost abstractions, idiomatic patterns'
applyTo: '**/*.rs'
---

# Rust Standards

## Toolchain

`cargo fmt` | `cargo clippy -- -D warnings` | `cargo test` | `cargo audit`

<Standards>

**Ownership**: Borrow (`&T`) over ownership when possible
**Errors**: `Result<T, E>` with `thiserror` for error types, `anyhow` for applications
**Abstractions**: Traits for interfaces, associated types, iterator chains over manual loops
**Performance**: Stack allocation for small types, `#[inline]` only when profiled
**Unsafe**: Document all `unsafe` blocks with safety invariants

</Standards>

## Patterns

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub fn process(input: &str) -> Result<String, AppError> {
    if input.is_empty() {
        return Err(AppError::NotFound("empty input".into()));
    }
    Ok(input.to_uppercase())
}
```

**Smart Pointers**: `Box<T>` (single owner) | `Rc<T>` (shared, single-thread) | `Arc<T>` (shared, thread-safe) | `Mutex<T>`/`RwLock<T>` (interior mutability)

**Concurrency**: channels (`mpsc`) for message passing | `tokio` for async | `rayon` for data parallelism

<Limitations>

- No `.unwrap()` in production
- No panics in library code
- No mutable global state
- No ignoring `Result` types
- No `unsafe` without safety documentation

</Limitations>

<Security>

- No hardcoded secrets or credentials
- Input validation at system boundaries
- Error messages must not leak implementation details
- Audit dependencies regularly

</Security>
