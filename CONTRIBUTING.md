# Contributing to JamProject Desktop

Thanks for considering a contribution! This repo is the **Tauri wrapper** around the web app at [jamproject.net](https://jamproject.net). It is intentionally small — most of the rhythm-game logic lives in the web frontend, not here.

## What belongs in this repo

- The Tauri Rust core (window config, IPC commands, native integrations)
- Discord Rich Presence wiring (`src-tauri/src/discord_rpc.rs`)
- Build configuration (`tauri.conf.json`, `Cargo.toml`, NSIS bundler settings)
- Icons, installer artwork, native OS plumbing
- Bridge contract docs (how the web app should call `invoke(...)`)

## What does NOT belong here

- Gameplay code, scoring formulas, charting — that's in the web app
- Song catalog, audio assets — served from the CDN
- API endpoints, database schema — separate backend
- General web-side bug reports — file those at [jamproject.net](https://jamproject.net)

If your contribution touches the web frontend or the API, please open it against the corresponding repo instead.

## Dev setup

```bash
# Install Rust (one-time)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Or on Windows: download rustup-init.exe from https://rustup.rs

git clone https://github.com/Molax/jamproject-desktop.git
cd jamproject-desktop
npm install
npm run tauri:dev
```

The first run takes a couple of minutes — Cargo compiles the entire dependency tree. After that, Rust code hot-reloads, and the WebView2 instance loads `https://jamproject.net` directly.

## Pull request checklist

Before opening a PR:

- [ ] `npm run tauri:build` succeeds locally (Windows)
- [ ] Rust code passes `cargo fmt` and `cargo clippy -- -D warnings` (run inside `src-tauri/`)
- [ ] If you added a new `invoke` command, the corresponding permission is declared in `src-tauri/capabilities/default.json`
- [ ] If the change is user-visible, the README's "Roadmap" or "Features" section is updated
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) — e.g. `feat(tray): add system tray icon`, `fix(rpc): clear presence on window close`

## Style

- **Rust:** stable toolchain, `cargo fmt` defaults, prefer `?`/`Result` over `unwrap()`
- **JS/TS** *(in `dist/` stub or scripts)*: 2-space indent, no semicolons-as-statement-terminators preference enforced
- **TOML:** alphabetised within sections where possible

## Reporting security issues

Please **do not** open a public issue for security-sensitive reports. Instead, contact the maintainer privately via Discord or email — details on [jamproject.net/contact](https://jamproject.net/contact).

## Code of conduct

Be kind. Disagree on technical things, not on people. Anyone making the community worse will be removed.
