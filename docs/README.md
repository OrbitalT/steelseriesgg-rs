# Documentation Directory

This directory contains development documentation and archived session reports.

## Structure

### `/development`

Active development documentation:

- **`OPTIMIZATION_REPORT.md`** - Performance optimization findings and recommendations
- **`DEPENDENCY_AUDIT_REPORT.md`** - Security audit results for dependencies
- **`todo.md`** - Development notes and future work ideas

### `/archive`

Historical session documentation and verification reports:

- RGB verification reports (multiple iterations)
- RALPH iteration summaries (debugging sessions)
- Physical verification documents
- Final status reports

**Note**: Archive files are excluded from git tracking (see `.gitignore`) to keep the repository clean.

## Main Documentation

User-facing and contributor documentation is in the repository root:

- **`README.md`** - User guide (installation, usage, features)
- **`CLAUDE.md`** - Developer guide (architecture, workflows, gotchas)
- **`CONTRIBUTING.md`** - Contribution guidelines
- **`LICENSE`** - MIT license

## Generating Code Documentation

View auto-generated Rust documentation:

```bash
cargo doc --all-features --open
```

This generates comprehensive API documentation from inline doc comments.
