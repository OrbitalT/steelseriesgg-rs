# Maintainer: Your Name <your.email@example.com>
pkgname=ssgg
pkgver=0.1.0
pkgrel=1
pkgdesc="A complete open-source SteelSeries GG replacement for Linux - RGB lighting, audio mixer, and GameSense support"
arch=('x86_64')
url="https://github.com/Ven0m0/steelseriesgg-rs"
license=('MIT')
depends=('hidapi' 'glibc')
makedepends=('rust' 'cargo')
install=ssgg.install
source=(
  "$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz"
  "ssgg.service"
  "ssgg.install"
)
sha256sums=('SKIP'  # Update this when creating a release
            'SKIP'  # Checksum for ssgg.service
            'SKIP') # Checksum for ssgg.install

prepare() {
  cd "steelseriesgg-rs-$pkgver"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "steelseriesgg-rs-$pkgver"
  export RUSTUP_TOOLCHAIN=nightly CARGO_TARGET_DIR=target
  # Build with default features only (no audio/sonar dependencies)
  cargo +nightly build -r --frozen
}

check() {
  cd "steelseriesgg-rs-$pkgver"
  export RUSTUP_TOOLCHAIN=nightly
  cargo +nightly test --frozen
}

package() {
  cd "steelseriesgg-rs-$pkgver"
  # Install binary
  install -Dm755 target/release/ssgg "$pkgdir/usr/bin/ssgg"
  # Install systemd user unit
  install -Dm644 "$srcdir/ssgg.service" "$pkgdir/usr/lib/systemd/user/ssgg.service"
  # Install udev rules
  install -Dm644 assets/99-steelseries.rules "$pkgdir/usr/lib/udev/rules.d/99-steelseries.rules"
  # Install license & documentation
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
