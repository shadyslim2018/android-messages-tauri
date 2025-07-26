# Maintainer: Your Name <your@email.com>
pkgname=android-messages-tauri
pkgver=1.0.0
pkgrel=1
pkgdesc="Unofficial Tauri-based desktop wrapper for Google Messages"
arch=('x86_64')
url="https://github.com/shadyslim2018/android-messages-tauri"
license=('MIT')
depends=('webkit2gtk' 'gtk3')
makedepends=('cargo' 'npm' 'nodejs' 'clang' 'mold')
source=("git+https://github.com/shadyslim2018/android-messages-tauri.git")
sha256sums=('SKIP')

build() {
  cd "$srcdir/$pkgname-$pkgver/src-tauri"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver/src-tauri"
  install -Dm755 "target/release/android-messages-tauri" \
    "$pkgdir/usr/bin/android-messages-tauri"

  # Desktop entry
  install -Dm644 "AndroidMessagesTauri.desktop" \
    "$pkgdir/usr/share/applications/android-messages-tauri.desktop"

  # Icon sizes (loop)
  for size in 512 256 128 64 32; do
    install -Dm644 "icons/${size}x${size}.png" \
      "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/android-messages-tauri.png"
  done
}
