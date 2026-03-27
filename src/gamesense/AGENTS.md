<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/gamesense/

## Purpose

GameSense HTTP server — an Axum-based HTTP server on `127.0.0.1:27301` that implements a subset of the SteelSeries GameSense API. Allows third-party games/apps to register events and trigger RGB callbacks.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | Module exports — `GameSenseServer`, `ServerState`, re-exports handlers |
| `server.rs` | Axum router setup, CORS middleware configuration, `Arc<RwLock<ServerState>>`, server lifecycle |
| `handlers.rs` | HTTP route handlers — game registration, event binding, event dispatch, RGB callbacks |

## For AI Agents

### Working In This Directory

- **CRITICAL**: CORS is restricted to `127.0.0.1` only — do **NOT** loosen this constraint for any reason
- The server state uses `Arc<RwLock<ServerState>>` — use `write()` only for mutations, `read()` for queries
- Security test `tests/cors_security.rs` must pass after any change to CORS config
- Axum version is `0.8.8` — check Axum 0.8.x docs (not 0.7.x or 0.6.x) for handler signatures
- `tower-http` CORS middleware version `0.6.8`

### Testing Requirements

```bash
cargo test cors                     # CORS security tests (critical)
cargo test                          # full test suite
```

### Common Patterns

```rust
// State access pattern
async fn handler(State(state): State<Arc<RwLock<ServerState>>>) -> impl IntoResponse {
    let state = state.read();       // read-only
    // ...
}

// CORS — same-origin only, do not change
let cors = CorsLayer::new()
    .allow_origin("http://127.0.0.1:27301".parse::<HeaderValue>().unwrap());
```

## Dependencies

### Internal
- `src/rgb/` — RGB callbacks trigger color/effect changes via `RgbController`
- `src/devices/` — device handles for RGB output
- `src/error.rs` — error types

### External
- `axum 0.8.8` — HTTP framework
- `tower-http 0.6.8` — CORS middleware
- `tokio` — async runtime
- `serde`/`serde_json` — JSON request/response serialization

<!-- MANUAL: -->
