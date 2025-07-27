# Android Messages (Unofficial) — Tauri Desktop Wrapper

A lightweight desktop wrapper for **Google Messages for Web**, built with **Tauri v2**.  
Runs as a native window with a **system tray** (Show/Hide toggle) and **close‑to‑tray** behavior.

> **Status:** Linux (X11/Wayland) focused. Built and tested on Arch/Manjaro.

---

## ✨ Features

- **Tray icon** with **Show/Hide** menu item
- **Left‑click tray** to toggle visibility
- **Close-to-tray** (window hides instead of quitting)
- Remembers app state during the session
- Minimal resource usage thanks to Tauri + WebKitGTK

> Tip: If you prefer starting minimized to tray, this can be added; open an issue or PR.

---

## 🧩 Runtime Dependencies (Linux)

You need WebKitGTK and an AppIndicator implementation for the tray:

- `webkit2gtk-4.1`
- `libayatana-appindicator`
- `librsvg` (recommended for icon handling)

**Arch/Manjaro**
```bash
sudo pacman -S --needed webkit2gtk-4.1 libayatana-appindicator librsvg
```

---

## 🚀 Install / Build

### Option A: Build from **PKGBUILD** (Arch/Manjaro)

This repo contains a `-git` PKGBUILD that builds from the latest `main`:

```bash
sudo pacman -S --needed base-devel webkit2gtk-4.1 libayatana-appindicator librsvg
git clone https://github.com/shadyslim2018/android-messages-tauri.git
cd android-messages-tauri/packaging/arch/android-messages-tauri-git
makepkg -si
```

This installs:
- Binary: `/usr/bin/android-messages-tauri`
- Desktop entry: `/usr/share/applications/android-messages-tauri.desktop`
- Icons in `hicolor` theme at multiple sizes

### Option B: Build from source (any Linux)

Requirements:
- Rust (stable) and Cargo
- System deps listed above

Build & run:
```bash
git clone https://github.com/shadyslim2018/android-messages-tauri.git
cd android-messages-tauri/src-tauri
cargo run --release
```

### Option C: Create distribution bundles (AppImage, deb, rpm)

```bash
cd android-messages-tauri/src-tauri
cargo install tauri-cli --locked || true
tauri build
```
Artifacts will be in `src-tauri/target/release/bundle/`.

---

## 🧭 App Behavior

- **Tray:** Right‑click shows a menu with **Show/Hide** and **Quit**.
- **Left‑click tray:** Toggles the window.
- **Close button:** Hides the window to the tray instead of quitting.
- The tray menu label stays in sync with the window visibility.

If the window doesn’t raise/focus on your desktop environment, please open an issue with your DE/WM details (KDE Plasma, GNOME, Wayland/X11).

---

## 🐛 Troubleshooting

- **Tray menu says “Hide Window”, but window is hidden**  
  Click **Show Window** once or left‑click the tray icon. If it persists, file an issue with logs from `tauri dev`.

- **Wayland/KDE focus issues**  
  We call `show → unminimize → set_focus`. If your WM still refuses focus, report details so we can add a raise workaround.

- **libayatana warning in terminal**  
  This is a deprecation notice from the library; functionality is unaffected.

---

## 🛠 Development

```bash
# Optional: frontend build step if/when a bundled UI is added
# npm ci && npm run build

# Rust development
cd src-tauri
cargo run
```

Speed up linking (optional):
```bash
export CC=clang CXX=clang++
export RUSTFLAGS="${RUSTFLAGS} -C link-arg=-fuse-ld=mold"
```

---

## 📦 Packaging Notes (Arch)

The VCS PKGBUILD lives at:
```
packaging/arch/android-messages-tauri-git/PKGBUILD
```

When you change packaging, bump `pkgrel` and commit the file.  
For publishing to AUR later, generate `.SRCINFO` with:
```bash
makepkg --printsrcinfo > .SRCINFO
```

---

## 🔒 Privacy

This app simply embeds Google’s official Messages web UI.  
No extra tracking or analytics are added by this project.

---

## 📄 License

MIT — see `LICENSE` if present, or the license header in source files.

---

## 🤝 Contributing

- Issues and PRs are welcome.
- If proposing tray/behavior changes, please include DE/WM + Wayland/X11 details.
