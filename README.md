# Android Messages Tauri

**Unofficial, privacy-friendly Tauri-based desktop client for [Google Messages for Web](https://messages.google.com/web).**

> ⚠️ This project is not affiliated with Google.

---

## Features

- Native desktop app for Google Messages (Tauri + WebKitGTK)
- Lightweight (Rust backend, no Electron/Chromium bundle)
- No telemetry or background daemons
- Secure, minimal permissions

---

## Installation

### Arch Linux / Manjaro (Recommended)

Build and install via `makepkg` directly from this repo.

```bash
# 1) Install build deps (if needed)
sudo pacman -S --needed base-devel git rustup nodejs npm clang mold webkit2gtk gtk3

# 2) Clone and build
git clone https://github.com/shadyslim2018/android-messages-tauri.git
cd android-messages-tauri
makepkg -si
```

> The `PKGBUILD` will compile the Tauri binary and install the desktop entry and icons system‑wide.

#### Update later
```bash
cd android-messages-tauri
git pull
makepkg -si
```

---

### Other Linux (Manual Build)

**Prerequisites**

- Rust (via `rustup`), e.g.:
  ```bash
  sudo pacman -S --needed rustup && rustup default stable
  ```
- Node.js & npm (only if you modify the frontend)
- System libs: `webkit2gtk` `gtk3` and friends
- (Optional, faster linking) `clang` and `mold`

**Steps**

```bash
git clone https://github.com/shadyslim2018/android-messages-tauri.git
cd android-messages-tauri/src-tauri

# Optional: faster link and ld relax workaround (seen on some systems)
env RUSTFLAGS="-C link-arg=-fuse-ld=mold -C link-arg=-Wl,--no-relax" \
  CC=clang CXX=clang++ \
  cargo build --release

# Run the app binary
./target/release/android-messages-tauri
```

> If you prefer debug hot‑reload during development:
> ```bash
> cd android-messages-tauri/src-tauri
> cargo tauri dev
> ```

---

## Desktop Integration

A `.desktop` file and icons are provided and installed by the PKGBUILD:
- Desktop entry: `AndroidMessagesTauri.desktop`
- Icons placed in `hicolor` under: `/usr/share/icons/hicolor/<size>/apps/android-messages-tauri.png`

If building manually, you can copy these from `src-tauri/` to system locations or run the app directly from the build folder.

---

## Troubleshooting

- **Linker / ld errors** (e.g., GOTPCREL/relax issues): try building with `mold` and `clang` as shown above and add `-Wl,--no-relax`.
- **Missing GTK/WebKit2**: install `webkit2gtk` and `gtk3` via your distro package manager.
- **AppImage bundling**: prefer distro packages/PKGBUILD on Arch. AppImage creation can be flaky across systems.
- **Slow first build**: Rust will compile many crates (GTK/WebKit bindings). Subsequent builds are fast.

---

## Project Layout

```
android-messages-tauri/
├─ public/                     # minimal static frontend
├─ src-tauri/                  # Tauri (Rust) project
│  ├─ src/main.rs              # main window + URL
│  ├─ icons/*.png              # app icons (32–512)
│  ├─ AndroidMessagesTauri.desktop
│  └─ tauri.conf.json
└─ PKGBUILD                    # Arch/Manjaro packaging script
```

---

## Why Tauri (vs. Electron)?

- Uses system WebKitGTK (smaller footprint)
- Rust backend (better memory/perf characteristics)
- Lower disk usage & RAM at runtime
- Native packaging targets per-OS

---

## License

MIT — see `LICENSE` (or choose a license of your preference).
