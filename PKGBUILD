# Maintainer: Your Name <your@email.com>
pkgname=android-messages-tauri
pkgver=1.0.0
pkgrel=1
pkgdesc="Unofficial Tauri-based desktop wrapper for Google Messages"
arch=('x86_64')
url="https://github.com/shadyslim2018/android-messages-tauri"
license=('MIT')

# Runtime deps
depends=(
  'webkit2gtk-4.1'
  'libayatana-appindicator'   # or: libappindicator-gtk3
  'librsvg'                   # optional but recommended
  # 'gtk3'                    # optional; pulled by webkit2gtk-4.1
)

# Build deps
makedepends=('cargo' 'npm' 'nodejs' 'clang' 'mold')

source=("git+https://github.com/shadyslim2018/android-messages-tauri.git")
sha256sums=('SKIP')

build() {
  cd "$srcdir/$pkgname/src-tauri"

  # If you have a frontend build step, do it before cargo:
  # pushd .. && npm ci && npm run build && popd

  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname/src-tauri"
  install -Dm755 "target/release/android-messages-tauri" \
    "$pkgdir/usr/bin/android-messages-tauri"

  install -Dm644 "AndroidMessagesTauri.desktop" \
    "$pkgdir/usr/share/applications/android-messages-tauri.desktop"

  for size in 512 256 128 64 32; do
    install -Dm644 "icons/${size}x${size}.png" \
      "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/android-messages-tauri.png"
  done

  # Optionally ship license
  # install -Dm644 "../LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
