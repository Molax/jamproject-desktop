<div align="center">

<img src="src-tauri/icons/128x128.png" alt="JamProject Desktop" width="128" height="128">

# JamProject Desktop

**The web rhythm game [JamProject](https://jamproject.net), packaged as a native desktop app for Windows and Linux — with Discord Rich Presence and lower input latency than the browser.**

[![Latest Release](https://img.shields.io/github/v/release/Molax/jamproject-desktop?style=flat-square&color=blue&label=latest)](https://github.com/Molax/jamproject-desktop/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/Molax/jamproject-desktop/total?style=flat-square&color=green)](https://github.com/Molax/jamproject-desktop/releases)
[![License](https://img.shields.io/badge/license-MIT-purple?style=flat-square)](LICENSE)
[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078D6?style=flat-square&logo=windows)](#-windows-install)
[![Linux](https://img.shields.io/badge/Linux-AppImage%20%C2%B7%20deb%20%C2%B7%20rpm-FCC624?style=flat-square&logo=linux&logoColor=black)](#-linux-install)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-FFC131?style=flat-square&logo=tauri&logoColor=black)](https://tauri.app/)
[![Discord Rich Presence](https://img.shields.io/badge/Discord-Rich%20Presence-5865F2?style=flat-square&logo=discord&logoColor=white)](#-discord-rich-presence)

<p>
  <a href="https://github.com/Molax/jamproject-desktop/releases/latest"><strong>⬇ Download</strong></a> ·
  <a href="https://jamproject.net/download"><strong>Download page</strong></a> ·
  <a href="https://jamproject.net"><strong>Play in browser</strong></a> ·
  <a href="https://github.com/Molax/jamproject-desktop/issues"><strong>Report a bug</strong></a>
</p>

</div>

---

## ⬇ Download

### 🪟 Windows 10/11 (x64)

| Source | Link |
| :--- | :--- |
| **GitHub Releases (NSIS installer)** | [`JamProject-windows-x64.exe`](https://github.com/Molax/jamproject-desktop/releases/latest/download/JamProject-windows-x64.exe) |
| **CDN mirror** (Cloudflare R2) | [`cdn.jamproject.net/desktop/JamProject-latest-windows-x64.exe`](https://cdn.jamproject.net/desktop/JamProject-latest-windows-x64.exe) |

### 🐧 Linux (x86_64)

| Distro | Format | Download |
| :--- | :--- | :--- |
| **Any** (distro-agnostic) | AppImage | [`JamProject-linux-x64.AppImage`](https://github.com/Molax/jamproject-desktop/releases/latest/download/JamProject-linux-x64.AppImage) · [CDN mirror](https://cdn.jamproject.net/desktop/JamProject-latest-linux-x64.AppImage) |
| **Debian, Ubuntu, Mint** | .deb | [`JamProject-linux-x64.deb`](https://github.com/Molax/jamproject-desktop/releases/latest/download/JamProject-linux-x64.deb) · [CDN mirror](https://cdn.jamproject.net/desktop/JamProject-latest-linux-x64.deb) |
| **Fedora, openSUSE, RHEL** | .rpm | [`JamProject-linux-x64.rpm`](https://github.com/Molax/jamproject-desktop/releases/latest/download/JamProject-linux-x64.rpm) · [CDN mirror](https://cdn.jamproject.net/desktop/JamProject-latest-linux-x64.rpm) |

### 🔎 Verification

Every asset ships with a `.sha256` sidecar next to it. The download page at [**jamproject.net/download**](https://jamproject.net/download) renders the live hashes and a step-by-step guide. See also [Verifying your download](#-verifying-your-download) below.

## ✨ Features

- 🪟 **Native window on Windows and Linux** — no browser chrome, proper OS integration
- ⚡ **Lower input latency** than the browser — no extra DOM/process layers between you and the audio clock
- 🎮 **Discord Rich Presence** — your current song, difficulty, mode, and elapsed time show in your status
- 📦 **Tiny footprint** — ~1 MB Windows installer / ~1.4 MB .deb/.rpm; reuses the system WebView2 (Windows) / WebKitGTK (Linux) instead of bundling Chromium
- 🔌 **Always up to date** — the app loads `https://jamproject.net`, so every web update is instant; no auto-updater dance
- 🔒 **Open-source, reproducible build** — audit the code, compile it yourself, diff against the binary

## 🚀 Why Tauri (not Electron)

|                     | **Tauri v2** | Electron |
| ------------------- | :----------: | :------: |
| Installer size      |    ~5 MB     | ~150 MB  |
| RAM at idle         |   ~80 MB     | ~300 MB  |
| Browser engine      | System WebView2 *(preinstalled on Win11)* | Bundled Chromium |
| Backend language    |     Rust     | Node.js  |
| Native API surface  | Granular permission system | Full Node access |

Tauri ships a Rust core plus the OS's own WebView. We get a 30× smaller binary, real OS integration (Discord IPC, deep links, tray, native menus), and a security model that's permission-scoped by default.

## 🎮 Discord Rich Presence

When the app is running, your Discord status shows what you're playing in real time:

```
🎵 Playing JamProject
   Through the Fire and Flames — DragonForce
   Expert · STRUM mode · 00:42
```

The app uses Discord's local IPC socket (no API tokens), so it works offline and never sees your Discord credentials. Implementation lives in [`src-tauri/src/discord_rpc.rs`](src-tauri/src/discord_rpc.rs).

## 💻 Requirements

### Windows
- **OS:** Windows 10 (build 17763+) or Windows 11 — x64
- **Disk:** ~50 MB
- **WebView2 Runtime:** preinstalled on Windows 11; auto-installed by the NSIS bundle on Windows 10 if missing

### Linux
- **Architecture:** x86_64
- **AppImage:** runs on any modern distro (glibc 2.31+, FUSE)
- **.deb:** Debian 11+, Ubuntu 22.04+, Linux Mint 21+
- **.rpm:** Fedora 38+, openSUSE Leap 15.5+, RHEL 9+
- **WebKitGTK 4.1:** preinstalled on most modern desktops; pulled in as a dependency by .deb / .rpm

### Both
- **Disk:** ~50 MB free
- **Discord** *(optional)* — only needed for Rich Presence; the app works fine without it

## 🪟 Windows install

Download the `.exe` and double-click it. The NSIS installer adds a Start menu entry and desktop shortcut. See [About the Windows SmartScreen warning](#%EF%B8%8F-about-the-windows-smartscreen-warning) below if Defender blocks the first launch.

## 🐧 Linux install

```bash
# AppImage (any distro) — no install, just run
chmod +x JamProject-linux-x64.AppImage
./JamProject-linux-x64.AppImage

# Debian / Ubuntu / Mint
sudo dpkg -i JamProject-linux-x64.deb
# (fix missing deps if needed: sudo apt-get install -f)

# Fedora / openSUSE / RHEL
sudo rpm -i JamProject-linux-x64.rpm
# or:  sudo dnf install ./JamProject-linux-x64.rpm
```

Both `.deb` and `.rpm` register the app with your desktop environment (icon, MIME types, `.desktop` launcher entry).

## 🔐 Verifying your download

Every release publishes the SHA-256 hash of each binary next to it (`.sha256` sidecar files). To verify:

**Windows (PowerShell):**

```powershell
Get-FileHash .\JamProject-windows-x64.exe
```

**Linux (bash):**

```bash
sha256sum -c JamProject-linux-x64.AppImage.sha256
# Or manually:
sha256sum JamProject-linux-x64.AppImage
```

Compare the output to the contents of the `.sha256` file. They must match exactly.

For extra paranoia, upload the file to [VirusTotal](https://www.virustotal.com) to scan it against 70+ antivirus engines. The build is reproducible from this repository — feel free to compile yourself and diff against the released binary.

## ⚠️ About the Windows SmartScreen warning

This build is **not yet code-signed** with a paid Microsoft Authenticode certificate, so SmartScreen may show a *"Windows protected your PC"* popup the first time you run it. The file is safe — Windows just doesn't yet recognise the publisher.

To proceed:

1. Click the small **"More info"** link at the top of the popup.
2. A new **"Run anyway"** button appears — click it.

This warning disappears once enough users have installed the build (Microsoft's "reputation" system kicks in). It will also disappear permanently when we sign the binary.

## 🛠️ Building from source

### Prerequisites

| Tool | Version | Purpose |
| --- | --- | --- |
| [Rust](https://www.rust-lang.org/tools/install) | stable (1.78+) | Tauri core compiles to native |
| [Node.js](https://nodejs.org/) | 22 LTS | Tauri CLI runs on Node |
| [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) | latest | Preinstalled on Win11 |

### Commands

```bash
git clone https://github.com/Molax/jamproject-desktop.git
cd jamproject-desktop
npm install

# Dev — opens a window pointing at jamproject.net with hot-reload of Rust code
npm run tauri:dev

# Production build — emits src-tauri/target/release/bundle/nsis/*.exe
npm run tauri:build
```

### Repository structure

```
jamproject-desktop/
├── dist/                      # Stub HTML — Tauri requires frontendDist even when loading a remote URL
├── src-tauri/
│   ├── Cargo.toml             # Rust dependencies (tauri 2 + discord-rich-presence)
│   ├── tauri.conf.json        # Window config, NSIS bundler, identifier, icon paths
│   ├── build.rs               # Standard Tauri build script
│   ├── capabilities/          # Tauri v2 permission scopes (must allow our invoke commands)
│   ├── icons/                 # Generated by `npx @tauri-apps/cli icon path/to/logo.png`
│   └── src/
│       ├── main.rs            # Binary entry point
│       ├── lib.rs             # tauri::Builder + command wiring
│       └── discord_rpc.rs     # Discord IPC client + activity state Mutex
├── package.json               # `tauri:dev` / `tauri:build` scripts
└── README.md                  # You are here
```

## 🤝 Bridge contract (web ↔ desktop)

The web app at `jamproject.net` detects whether it's running inside JamProject Desktop via:

```ts
const isDesktop = typeof (window as any).__TAURI__ !== 'undefined';
```

When `isDesktop` is true, the web app can call into the Tauri runtime:

```ts
import { invoke } from '@tauri-apps/api/core';

await invoke('update_presence', {
  payload: {
    details: 'Through the Fire and Flames — DragonForce',
    state: 'STRUM · Expert',
    large_image: 'jamproject_logo',
    start_timestamp: Math.floor(Date.now() / 1000),
  },
});
```

The web app degrades gracefully when running in a regular browser (`isDesktop === false`).

## 🐛 Reporting bugs

Open an [issue](https://github.com/Molax/jamproject-desktop/issues/new/choose) with:

- Your Windows version (`winver` from the Run dialog)
- App version (file name of the `.exe` you installed)
- Steps to reproduce
- Discord state (running? closed? minimised to tray?) if it's a Rich Presence issue

For web-side bugs (gameplay, scoring, account, etc.), please file them at [jamproject.net](https://jamproject.net) — those live in the web app, not here.

## 🗺️ Roadmap

- [x] **Phase 1 — MVP wrapper** *(shipped in v1.0.0)* — Tauri v2 scaffold, single window loading `jamproject.net`, NSIS installer tested on Win11
- [x] **Phase 2 — Discord Rich Presence** *(shipped in v1.0.0)* — Discord application registered, IPC client wired, web bridge via `invoke('update_presence', ...)`
- [ ] **Phase 3 — Polish** — system tray icon, deep-link handler (`jamproject://song/<id>`), splash screen while WebView2 cold-starts, native menu bar
- [ ] **Phase 4 — Distribution** — code-signing certificate, GitHub Actions release pipeline, Tauri auto-updater, macOS + Linux builds *(if demand exists)*

## 📄 License

[MIT](LICENSE) — go wild. JamProject's web frontend and gameplay assets are governed separately at [jamproject.net](https://jamproject.net).

## 🙏 Acknowledgements

- **[Tauri](https://tauri.app/)** for the most ergonomic native-app framework out there
- **[discord-rich-presence](https://crates.io/crates/discord-rich-presence)** for the Rust IPC client
- **[JamProject Beta 1 Bronze (2011)](https://jamproject.net)** — the Flash-era rhythm game we're keeping alive

---

<div align="center">
  <sub>Built with 🦀 Rust + 🌐 WebView2 · part of the <a href="https://jamproject.net">JamProject</a> family</sub>
</div>
