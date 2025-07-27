# Maintainer: Abdul Khaliq <you@example.com>
pkgname=android-messages-tauri-git
pkgver=0.0.0
pkgrel=1
pkgdesc="Unofficial Tauri-based desktop wrapper for Google Messages (git)"
arch=('x86_64')
url="https://github.com/shadyslim2018/android-messages-tauri"
license=('MIT')

depends=(
  'webkit2gtk-4.1'
  'libayatana-appindicator'
  'librsvg'
)

makedepends=(
  'git'
  'cargo'
  'clang'
  'mold'
  # 'npm' 'nodejs'  # uncomment if you actually build a JS frontend
)

provides=('android-messages-tauri')
conflicts=('android-messages-tauri')

source=("$pkgname::git+https://github.com/shadyslim2018/android-messages-tauri.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  # Use tag if present, otherwise commit count + short hash
  git describe --tags --long 2>/dev/null || \
    printf "%s.r%s.g%s" "0.0.0" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
  cd "$srcdir/$pkgname"

  # If you have a frontend build step, do it here (adjust path/commands):
  # npm ci
  # npm run build

  cd src-tauri

  # Optional linker speedups
  export CC=clang CXX=clang++
  export RUSTFLAGS="${RUSTFLAGS} -C link-arg=-fuse-ld=mold"

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

  # License (if present one directory above)
  if [[ -f "$srcdir/$pkgname/LICENSE" ]]; then
    install -Dm644 "$srcdir/$pkgname/LICENSE" \
      "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  fi
}
