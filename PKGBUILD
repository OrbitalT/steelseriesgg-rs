#!/usr/bin/env bash
# Maintainer: Your Name <your.email@example.com>
# shell: bash
# shellcheck disable=SC2034,SC2154
pkgname=ssgg
pkgver=0.1.0
pkgrel=1
pkgdesc="A complete open-source SteelSeries GG replacement for Linux - RGB lighting, audio mixer, and GameSense support"
arch=('x86_64')
url="https://github.com/Ven0m0/steelseriesgg-rs"
license=('MIT')
depends=('hidapi' 'glibc' 'systemd')
makedepends=('rust' 'cargo')
install=ssgg.install
source=(
  "$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz"
)
# Update this when creating a release
sha256sums=('SKIP')

prepare() {
  cd "steelseriesgg-rs-$pkgver" || return 1
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "steelseriesgg-rs-$pkgver" || return 1
  export CARGO_TARGET_DIR=target
  # Build with default features only (no audio/sonar dependencies)
  cargo build -r --locked
}

check() {
  cd "steelseriesgg-rs-$pkgver" || return 1
  cargo test --locked
}

package() {
  cd "steelseriesgg-rs-$pkgver" || return 1
  # Install binary
  install -Dm755 target/release/ssgg "$pkgdir/usr/bin/ssgg"
  # Install systemd user unit
  install -Dm644 assets/ssgg.service "$pkgdir/usr/lib/systemd/user/ssgg.service"
  # Install udev rules
  install -Dm644 assets/99-steelseries.rules "$pkgdir/usr/lib/udev/rules.d/99-steelseries.rules"
  # Install license & documentation
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
