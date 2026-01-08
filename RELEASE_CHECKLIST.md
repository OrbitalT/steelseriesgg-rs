# Release Checklist for v0.1.0

This document outlines the steps needed to create the first Arch Linux release.

## Pre-Release Steps

### 1. Update PKGBUILD Checksums

Before creating a release tag, you need to generate the actual checksums for the source files:

1. Create a test tarball from the current code:
```bash
git archive --format=tar.gz --prefix=steelseriesgg-rs-0.1.0/ HEAD > /tmp/ssgg-0.1.0.tar.gz
```

2. Generate checksums for the tarball:
```bash
sha256sum /tmp/ssgg-0.1.0.tar.gz
```

3. Generate checksums for systemd unit and install script:
```bash
sha256sum assets/ssgg.service
sha256sum ssgg.install
```

4. Update the `sha256sums` array in PKGBUILD with the actual values (replace the 'SKIP' placeholders).

### 2. Test PKGBUILD Locally

Test the PKGBUILD before pushing:

```bash
# Create a clean build directory
mkdir -p /tmp/ssgg-test
cd /tmp/ssgg-test

# Copy PKGBUILD and related files
cp /path/to/steelseriesgg-rs/PKGBUILD .
cp /path/to/steelseriesgg-rs/assets/ssgg.service .
cp /path/to/steelseriesgg-rs/ssgg.install .

# Update source to point to local tarball or a test branch
# Edit PKGBUILD: source=("file:///tmp/ssgg-0.1.0.tar.gz" ...)

# Build the package
makepkg -s

# Test install
sudo pacman -U ssgg-0.1.0-1-x86_64.pkg.tar.zst

# Test the binary
ssgg --version
ssgg devices

# Test systemd service
systemctl --user status ssgg.service
systemctl --user start ssgg.service
journalctl --user -u ssgg.service

# Clean up
systemctl --user stop ssgg.service
sudo pacman -R ssgg
```

### 3. Update Repository URL (if needed)

The repository URL in Cargo.toml and PKGBUILD currently points to:
```
https://github.com/Ven0m0/steelseriesgg-rs
```

If this is incorrect or has changed, update:
- `Cargo.toml` → `repository` field
- `PKGBUILD` → `url` variable
- `assets/ssgg.service` → `Documentation` field

### 4. Create a Git Tag

Once everything is tested and ready:

```bash
# Make sure all changes are committed
git add .
git commit -m "Prepare for v0.1.0 release"

# Create and push the tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin main
git push origin v0.1.0
```

The `v0.1.0` tag push will automatically trigger the GitHub Actions workflow to build and publish the Arch package.

## Post-Release Steps

### 1. Verify GitHub Release

After pushing the tag:

1. Go to: https://github.com/Ven0m0/steelseriesgg-rs/releases
2. Verify the release was created automatically
3. Check that the following files are attached:
   - `ssgg-0.1.0-1-x86_64.pkg.tar.zst`
   - `sha256sums.txt`

### 2. Test the Released Package

Download and test the released package:

```bash
wget https://github.com/Ven0m0/steelseriesgg-rs/releases/download/v0.1.0/ssgg-0.1.0-1-x86_64.pkg.tar.zst
sudo pacman -U ssgg-0.1.0-1-x86_64.pkg.tar.zst
```

### 3. Update AUR (Optional)

If you want to publish to the AUR:

1. Clone the AUR repository (or create a new one):
```bash
git clone ssh://aur@aur.archlinux.org/ssgg.git ssgg-aur
cd ssgg-aur
```

2. Copy the updated PKGBUILD, .install, and generate .SRCINFO:
```bash
cp /path/to/steelseriesgg-rs/PKGBUILD .
cp /path/to/steelseriesgg-rs/ssgg.install .
makepkg --printsrcinfo > .SRCINFO
```

3. Commit and push to AUR:
```bash
git add PKGBUILD ssgg.install .SRCINFO
git commit -m "Update to v0.1.0"
git push
```

## What Was Changed

### New Files Added
- `.github/workflows/ci.yml` - CI workflow for testing on push/PR
- `.github/workflows/release-arch.yml` - Arch package build on tag push
- `assets/ssgg.service` - Systemd user unit for daemon
- `ssgg.install` - Post-install script for udev/systemd reload
- `RELEASE_CHECKLIST.md` - This file

### Modified Files
- `PKGBUILD` - Production-ready with proper source, checksums, installs
- `Cargo.toml` - Added tokio `signal` feature for graceful shutdown
- `src/main.rs` - Added SIGTERM/SIGINT handling to daemon
- `README.md` - Added Arch installation, systemd, and feature documentation

### Removed Files
- `.github/workflows/rust.yml` - Removed incorrect Windows workflow

## Notes

- The package builds with **default features only** (no audio/sonar)
- The systemd unit runs as a **user service** (not system-wide)
- udev rules allow non-root device access via `input` group
- Graceful shutdown works with both Ctrl+C and `systemctl stop`
