# Fix Release Workflows

## Root Cause

The `release-please.yml` workflow is correctly configured and **does** create GitHub Releases when a release PR is merged. The `release.yml` workflow correctly triggers on `v*.*.*` tags. The configuration files (`release-please-config.json`, `.release-please-manifest.json`) are valid.

**The actual problem is that no release PR has been merged since `v0.1.0`.** There are ~70 commits since the tag, but none have triggered a new release because:

1. The `release-please-action@v5` creates/updates a release PR on push to `main`
2. That PR must be **merged** to create the tag + GitHub Release
3. The tag then triggers `release.yml` to build and upload binaries

The workflows themselves are structurally correct. However, there is one **real bug** in `release.yml`:

### Bug: `release.yml` uses `softprops/action-gh-release@v3` which creates a *new* release

When `release-please` creates a GitHub Release and then `release.yml` fires on the tag, `softprops/action-gh-release@v3` will either:
- Upload to the existing release (if it finds it), OR
- Create a duplicate/conflicting release

This is fragile. The `release-please-action` already creates the GitHub Release with changelog notes. The `release.yml` should only **upload assets** to that existing release, not risk creating a new one.

## Changes

### 1. `release.yml` — Use `gh release upload` instead of `softprops/action-gh-release`

Replace `softprops/action-gh-release@v3` with `gh release upload` which reliably uploads to the **existing** release created by release-please. This avoids any race condition or duplicate release creation.

**Before (both upload steps):**
```yaml
- name: Upload to release
  if: startsWith(github.ref, 'refs/tags/')
  uses: softprops/action-gh-release@v3
  with:
    files: |
      ssgg-linux-x86_64.tar.gz
      ...
```

**After:**
```yaml
- name: Upload to release
  if: startsWith(github.ref, 'refs/tags/')
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  run: |
    gh release upload "${{ github.ref_name }}" \
      ssgg-linux-x86_64.tar.gz \
      sha256sums-linux.txt \
      *.pkg.tar.zst \
      sha256sums-arch.txt \
      --clobber
```

Same pattern for the Windows job.

The `--clobber` flag ensures re-runs overwrite existing assets. `gh` is pre-installed on all GitHub-hosted runners.

### 2. `release-please.yml` — No changes needed

The workflow is correctly configured:
- Triggers on push to `main`
- Uses `RELEASE_PLEASE_TOKEN` (PAT) so tag pushes trigger downstream workflows
- References valid config and manifest files
- `release-type: rust` correctly bumps `Cargo.toml` version

### 3. Verify `RELEASE_PLEASE_TOKEN` secret exists

This is a prerequisite, not a code change. The user must ensure this secret is set in the repo settings. Without it, the tag push uses `GITHUB_TOKEN` and won't trigger `release.yml`.

## Files to modify

| File | Change |
|------|--------|
| `.github/workflows/release.yml` | Replace `softprops/action-gh-release@v3` with `gh release upload` in both jobs |
| `.github/workflows/release-please.yml` | No changes |

## Verification

1. `cargo fmt --all -- --check` (no Rust changes, but good practice)
2. Validate YAML syntax of modified workflow
3. After merging, push a conventional commit to `main` → release-please creates/updates release PR → merge PR → tag created → `release.yml` fires → binaries uploaded to the GitHub Release
