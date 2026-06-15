---
description: Pre-PR checklist — run CI checks and protocol review on staged changes, report go/no-go
---

Run the pre-PR checklist for steelseriesgg-rs. Stop at the first failure.

## Steps

1. **Format** — `cargo fmt --all -- --check`
2. **Clippy (default)** — `cargo clippy --all-targets --locked -- -D warnings`
3. **Clippy (sonar)** — `cargo clippy --all-targets --locked --features sonar -- -D warnings`
4. **Tests** — `cargo test --locked`
5. **Protocol review** — run `git diff HEAD` and check the diff against these rules:
   - No raw byte arrays constructed by hand (must use `HidReportBuilder`)
   - Every `unsafe` block has a `// SAFETY:` comment
   - No `.unwrap()` or `.expect()` outside `#[cfg(test)]`
   - No experimental command codes (`0x40`, `0x2D`) without `[EXPERIMENTAL]` label
   - GameSense CORS origin list unchanged

## Output

Report each step as PASS or FAIL with a one-line reason. End with either:
- **GO** — all steps passed, safe to push
- **NO-GO** — list the blockers that must be fixed first
