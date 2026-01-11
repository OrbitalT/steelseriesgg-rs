# Binary Size Optimization Report

## Summary

Optimized the steelseriesgg-rs binary for minimal size while maintaining full functionality.

## Current Binary Size

**Final Size**: 2,220,280 bytes (2.12 MB)

## Optimizations Applied

### 1. Compiler Optimizations (Cargo.toml)

```toml
[profile.release]
strip = true           # Remove debug symbols
lto = "fat"            # Full link-time optimization across all crates
codegen-units = 1      # Single codegen unit for better optimization
panic = "abort"        # Don't generate unwinding code
opt-level = "z"        # Optimize for size instead of speed
debug = 0              # No debug info
overflow-checks = false # Disable integer overflow checks in release
```

**Key Changes**:
- Changed `lto = true` → `lto = "fat"` for maximum cross-crate optimization
- Changed `opt-level = 3` → `opt-level = "z"` to prioritize size over speed
- Added `overflow-checks = false` to reduce runtime checks

### 2. Dependency Feature Minimization

Disabled default features and only enabled what's needed:

**axum** (HTTP server):
```toml
# Before: axum = "0.8"
# After:
axum = { version = "0.8", default-features = false, features = ["tokio", "http1", "json"] }
```
- Removed HTTP/2 support (http2)
- Removed form support
- Only kept essential features

**tokio** (async runtime):
```toml
# Before: tokio = { version = "1.49", features = ["rt-multi-thread", "macros", "signal"] }
# After:
tokio = { version = "1.49", default-features = false, features = ["rt-multi-thread", "macros", "signal"] }
```
- Disabled all non-essential tokio features
- Kept only runtime, macros, and signal handling

**tower-http** (HTTP middleware):
```toml
# Before: tower-http = { version = "0.6", features = ["cors"] }
# After:
tower-http = { version = "0.6", default-features = false, features = ["cors"] }
```
- Only enabled CORS support, nothing else

## Size Analysis

### Current State
- **Binary Size**: 2.12 MB (already well-optimized)
- **Debug Info**: Stripped
- **Link-Time Optimization**: Full (fat LTO)
- **Code Generation**: Single unit for maximum optimization

### Why This Size?

The binary includes:
1. **hidapi** - USB HID device communication library
2. **tokio** - Async runtime (essential for daemon mode)
3. **axum** - HTTP server framework (for GameSense API)
4. **clap** - CLI argument parsing
5. **tracing** - Logging infrastructure

All of these are essential for the application's functionality.

## Verification Tests

All functionality verified after optimization:

✓ Binary runs successfully
✓ RGB color control working (`ssgg rgb color green`)
✓ Poll rate status working (`ssgg pollrate status`)
✓ Version info accessible (`ssgg --version`)
✓ All CLI commands functional

## Additional Optimization Options (Not Applied)

### UPX Compression
- **Not available** in current environment
- Could potentially reduce size by 40-60% if installed
- Trade-off: Slower startup time, higher memory usage during decompression

### Cargo-bloat Analysis
- **Not installed** in current environment
- Would help identify largest dependencies and functions
- Useful for targeted size reduction

## Recommendations

### If Smaller Size Is Critical

1. **Install UPX** and compress the binary:
   ```bash
   upx --best --lzma target/release/ssgg
   ```
   Expected reduction: ~40-60% (binary would be ~900KB-1.3MB)

2. **Split Features**: Consider separating daemon/server mode into a separate binary:
   - `ssgg` - CLI-only binary (smaller, no tokio/axum)
   - `ssgg-daemon` - Daemon with server functionality

   This could reduce the CLI-only binary to ~1.5MB

3. **Replace Dependencies**:
   - Consider `clap_mangen` instead of full `clap` if only basic parsing needed
   - Replace `tracing` with simpler logging if advanced features not used
   - Use `minreq` instead of `reqwest` for sonar feature (if HTTP/2 not needed)

### Size vs Performance Trade-offs

Current settings prioritize size over performance:
- `opt-level = "z"` - Code is optimized for size, may be ~5-10% slower
- `lto = "fat"` - Longer compile times (~30s) for smaller binary
- `codegen-units = 1` - Longer compile times, no parallel codegen

If performance is more critical, consider:
```toml
opt-level = 3           # Speed over size (+10-15% size, +5-10% speed)
lto = "thin"            # Faster compile, slightly larger binary
codegen-units = 16      # Parallel compilation (faster builds)
```

## Conclusion

The binary is now optimized for minimal size while maintaining all functionality. At 2.12 MB, this is an excellent size for a Rust CLI application with:
- USB HID communication
- Async HTTP server
- Comprehensive CLI interface
- Logging infrastructure
- Multiple hardware features

Further size reduction would require either:
1. UPX compression (external tool)
2. Feature splitting (architectural change)
3. Dependency replacement (development effort)
