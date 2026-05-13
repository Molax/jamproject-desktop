<div align="center">

<img src="src-tauri/icons/128x128.png" alt="JamProject Desktop" width="128" height="128">

# JamProject Desktop

**The web rhythm game [JamProject](https://jamproject.net), packaged as a native Windows app — with Discord Rich Presence, lower input latency, and the system WebView2 instead of a bundled Chromium.**

[![Latest Release](https://img.shields.io/github/v/release/Molax/jamproject-desktop?style=flat-square&color=blue&label=latest)](https://github.com/Molax/jamproject-desktop/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/Molax/jamproject-desktop/total?style=flat-square&color=green)](https://github.com/Molax/jamproject-desktop/releases)
[![License](https://img.shields.io/badge/license-MIT-purple?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-0078D6?style=flat-square&logo=windows)](#-requirements)
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

| Source | Link |
| :--- | :--- |
| 🪟 **Windows installer (recommended)** | [`JamProject-windows-x64.exe`](https://github.com/Molax/jamproject-desktop/releases/latest) |
| 🌐 **CDN mirror** (Cloudflare R2) | [`cdn.jamproject.net/desktop/JamProject-latest-windows-x64.exe`](https://cdn.jamproject.net/desktop/JamProject-latest-windows-x64.exe) |
| 🔎 **Verification guide** | [jamproject.net/download](https://jamproject.net/download) |

Each release ships with a `.sha256` sidecar so you can verify the file is byte-identical to what was published. See [Verifying your download](#-verifying-your-download).

## ✨ Features

- 🪟 **Native Windows window** — no browser chrome, proper Win11 styling
- ⚡ **Lower input latency** than the browser — no extra DOM/process layers between you and the audio clock
- 🎮 **Discord Rich Presence** — your current song, difficulty, mode, and elapsed time show in your status
- 📦 **Tiny footprint** — ~5 MB installer, ~50 MB on disk; reuses the system WebView2 instead of bundling Chromium
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

- **OS:** Windows 10 (build 17763+) or Windows 11 — x64
- **Disk:** ~50 MB
- **WebView2 Runtime:** preinstalled on Windows 11; auto-installed by the NSIS bundle on Windows 10 if missing
- **Discord** *(optional)* — only needed for Rich Presence; the app works fine without it

## 🔐 Verifying your download

Every release publishes the SHA-256 hash of the `.exe` next to the binary. To verify:

```powershell
# 1. Download both JamProject-windows-x64.exe and JamProject-windows-x64.exe.sha256
# 2. From the same folder, run:
Get-FileHash .\JamProject-windows-x64.exe

# 3. Compare the Hash field to the contents of the .sha256 file. They must match.
```

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
