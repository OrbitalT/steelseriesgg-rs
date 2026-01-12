# Contributing to steelseriesgg-rs

Thank you for your interest in contributing to steelseriesgg-rs! This guide will help you get started.

## Quick Start

1. **Fork and clone** the repository
2. **Install dependencies** (see README.md)
3. **Make your changes**
4. **Run quality checks**: `cargo fmt && cargo clippy --all-targets --all-features`
5. **Run tests**: `cargo test --all-features`
6. **Submit a pull request**

## Code Quality Standards

### Before Submitting

All PRs must pass the following checks:

```bash
# Format code (required)
cargo fmt

# Lint code (no warnings allowed)
cargo clippy --all-targets --all-features

# Run all tests (must pass)
cargo test --all-features

# Verify compilation with no features
cargo check

# Verify compilation with each feature individually
cargo check --features audio
cargo check --features sonar
```

### Code Style

- **Indentation**: 4 spaces (enforced by `.editorconfig`)
- **Line length**: 100 characters maximum
- **Naming conventions**:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, traits, enums
  - `SCREAMING_SNAKE_CASE` for constants
- **Documentation**: All public APIs must have doc comments
- **Error handling**: Use `Result<T>` and the `?` operator
- **No `unwrap()` or `expect()`** in production code (tests are OK)

### Documentation

- **Public APIs**: Must have `///` doc comments
- **Modules**: Should have `//!` module-level documentation
- **Examples**: Include usage examples in doc comments when helpful
- **Update CLAUDE.md**: If you change architecture or add new workflows

Example:
```rust
/// Calculate the total discount for an order.
///
/// # Arguments
///
/// * `order` - The order to calculate discount for
/// * `customer_tier` - Customer's tier level
///
/// # Returns
///
/// The discount amount as a `Decimal`
///
/// # Examples
///
/// ```
/// let discount = calculate_discount(&order, CustomerTier::VIP);
/// assert_eq!(discount, Decimal::from(10));
/// ```
pub fn calculate_discount(order: &Order, customer_tier: CustomerTier) -> Decimal {
    // Implementation
}
```

## Development Workflow

### Adding a New Device

See the [CLAUDE.md - Adding a New Device](CLAUDE.md#adding-a-new-device) section for detailed instructions.

**TL;DR**:
1. Identify product ID with `lsusb | grep SteelSeries`
2. Add constant to `src/devices/mod.rs` (product_ids module)
3. Update `device_type_from_product_id()` and `device_name_from_product_id()`
4. Test with `cargo run -- devices`
5. If device needs specialized logic, create module in `devices/keyboards/` or `devices/headsets/`

### Adding a New CLI Command

1. Add variant to `Commands` enum in `src/main.rs`
2. Add command handler function
3. Add match arm in `main()` to dispatch to handler
4. Update `README.md` usage section
5. Add tests if applicable

### Adding Tests

All new features should include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        // Arrange
        let input = create_test_data();

        // Act
        let result = my_function(input);

        // Assert
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_edge_case() {
        // Test boundary conditions, empty input, etc.
    }
}
```

**Test coverage goals**:
- All public functions should have at least one test
- Edge cases (empty input, invalid values, etc.) should be tested
- Error conditions should be tested

## Git Workflow

### Branches

- `main` - Stable releases
- `develop` - Development branch (if used)
- `feature/<name>` - New features
- `fix/<name>` - Bug fixes
- `docs/<name>` - Documentation updates

### Commit Messages

Use clear, descriptive commit messages:

```
Add support for Apex 9 keyboard

- Add product ID 0x162A to device mappings
- Update device_name_from_product_id()
- Add product constant APEX_9

Fixes #42
```

**Format**:
- First line: Brief summary (imperative mood, <50 chars)
- Blank line
- Detailed description (what and why, not how)
- Reference issues/PRs if applicable

### Pull Requests

**Before submitting**:
1. Rebase on latest `main`
2. Run all quality checks (see [Before Submitting](#before-submitting))
3. Update documentation if needed
4. Add tests for new functionality

**PR Description Template**:
```markdown
## Summary
Brief description of what this PR does

## Changes
- Bullet point list of changes
- Include file paths for major changes

## Testing
How did you test this? Include:
- Steps to reproduce (for bug fixes)
- Test commands run
- Device tested on (if hardware-specific)

## Related Issues
Fixes #123
Relates to #456
```

## Project Structure

See [CLAUDE.md - Architecture](CLAUDE.md#architecture) for detailed module documentation.

Key modules:
- `devices/` - Device discovery and hardware communication
- `rgb/` - RGB color and lighting effects
- `gamesense/` - GameSense HTTP server
- `audio/` - Audio mixer and Sonar integration (optional features)
- `profiles/` - Configuration persistence
- `config/` - TOML configuration management

## Feature Flags

When working with feature-gated code:

```rust
// Compile-time feature gates
#[cfg(feature = "audio")]
pub mod audio;

#[cfg(any(feature = "audio", feature = "sonar"))]
use crate::audio::AudioMixer;
```

**Test all feature combinations**:
```bash
cargo test                          # No features
cargo test --features audio         # Audio only
cargo test --features sonar         # Sonar (implies audio)
cargo test --all-features           # All features
```

## Common Gotchas

See [CLAUDE.md - Common Gotchas](CLAUDE.md#common-gotchas) for a comprehensive list.

**Top 3**:
1. **HID reports must be exactly 65 bytes** (report ID + 64 data bytes)
2. **Sonar feature requires audio feature** (enforced in Cargo.toml)
3. **Animated RGB effects require daemon mode** (CLI commands are one-shot)

## Testing

### Running Tests

```bash
# All tests
cargo test --all-features

# Specific module
cargo test rgb::tests

# Specific test
cargo test test_effect_engine_static

# With output
cargo test -- --nocapture
```

### Hardware Testing

If you have SteelSeries hardware:

1. **List devices**:
   ```bash
   cargo run -- devices
   ```

2. **Test RGB control**:
   ```bash
   cargo run -- rgb --color red
   cargo run -- rgb --effect breathing --color cyan
   ```

3. **Enable debug logging**:
   ```bash
   RUST_LOG=debug cargo run -- devices
   ```

4. **Test daemon mode**:
   ```bash
   cargo run --release -- daemon
   ```

## Documentation

### User Documentation (README.md)

- Installation instructions
- Usage examples
- Feature descriptions
- Troubleshooting

### Developer Documentation (CLAUDE.md)

- Architecture overview
- Module documentation
- Development workflows
- Common gotchas

### Code Documentation (Rustdoc)

Generate and view documentation:
```bash
cargo doc --all-features --open
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/Ven0m0/steelseriesgg-rs/issues)
- **Questions**: Open a GitHub Discussion
- **Chat**: (if available)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing!** 🎉
