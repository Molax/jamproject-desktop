# CLAUDE.md — jamproject-desktop

Loaded every session. Detail lives in `README.md`.

User rules: no code comments, deliver high-quality code, user is a senior developer.

## What this is

Tauri v2 desktop wrapper for [jamproject.net](https://jamproject.net). Single window pointing at the live site + native **Discord Rich Presence** with our own Application ID. Sister repo to [`jam-legend-revival`](https://github.com/Molax/jam-legend-revival) (the actual web app). This repo is **only the desktop shell** — never duplicate gameplay logic here.

## Stack

- **Rust 1.77+** (MSVC toolchain) — `cargo` + `rustup`
- **Tauri v2** (`tauri = "2"`) — uses Windows WebView2 (Edge Chromium) at runtime, no bundled browser
- **`discord-rich-presence` crate** — local IPC with the Discord client
- **Node 22 LTS** — only for `@tauri-apps/cli` wrapper scripts
- **NSIS** — Windows installer target (`.exe`, perMachine)

No frontend build — the live site is loaded directly via `WebviewUrl::External("https://jamproject.net")`.

## Layout

```
src-tauri/
├── Cargo.toml                     # tauri 2 + discord-rich-presence + serde
├── tauri.conf.json                # window config + bundle config (URL hardcoded)
├── build.rs                       # tauri_build::build()
├── capabilities/default.json      # v2 permissions — `core:default` only
├── icons/                         # generated via `npx tauri icon <src.png>`
└── src/
    ├── main.rs                    # entry — calls lib::run()
    ├── lib.rs                     # tauri::Builder + invoke handlers
    └── discord_rpc.rs             # PresenceState (Mutex<Option<DiscordIpcClient>>)
dist/index.html                    # stub — required by tauri.conf even if unused
```

## How to run

```bash
npm install                        # one-time
npm run tauri:dev                  # debug build + opens window (~5min first compile)
npm run tauri:build                # release .exe in src-tauri/target/release/bundle/nsis/
```

Requires VS 2022 Build Tools w/ VCTools workload (`link.exe`) + Windows 11 SDK. WebView2 runtime ships with Win11.

## Bridge contract (web ↔ tauri)

The web app at jamproject.net detects `window.__TAURI__` and calls:

```ts
window.__TAURI__.core.invoke('update_presence', { payload: PresencePayload });
window.__TAURI__.core.invoke('clear_presence');
```

`PresencePayload` shape lives in [`src-tauri/src/discord_rpc.rs`](src-tauri/src/discord_rpc.rs). Keep the Angular service's payload TS type in sync with the Rust struct — they're a hand-written contract, not auto-generated.

## Conventions

- **No comments** unless a constraint is genuinely non-obvious (Rust-specific gotcha, Tauri v2 quirk).
- Names English, commits Portuguese, end-user strings irrelevant here (UI is the website).
- Strictly typed in both Rust and TS bridge stubs.
- Cargo.lock **is** committed (binary crate convention).
- Icons in `src-tauri/icons/` — current ones are placeholder ("JL" yellow on black). Replace when final art lands; regenerate with `npx tauri icon path/to/logo-1024.png`.

## TODO before first prod build

1. Replace `DISCORD_CLIENT_ID = "REPLACE_WITH_DISCORD_APPLICATION_ID"` in [`src-tauri/src/discord_rpc.rs`](src-tauri/src/discord_rpc.rs) with the real Application ID from https://discord.com/developers/applications.
2. Upload Rich Presence art assets in that Discord application panel (large logo + per-difficulty small icons).
3. Replace placeholder icons (`npx tauri icon`).
4. (Optional) Code signing cert ($100/year) to skip SmartScreen warning.

## Security

- WebView2 loads only `https://jamproject.net` (hardcoded in `tauri.conf.json`). Never widen — that lets arbitrary sites run with our identity.
- `core:default` capability only. **Never** add `shell:execute`, `fs:read-all`, `fs:write-all` etc. without an explicit threat-model review — Tauri's whole pitch is the deny-by-default surface.
- Discord IPC is local-only (named pipes / unix sockets); no network attack surface from RPC.

## Don'ts

- Don't add a frontend build (Vue/React/etc.) — site is remote, not local.
- Don't bundle assets that mirror the website — duplication will rot.
- Don't add gameplay/audio/timing logic — that lives upstream in `jam-legend-revival`.
- Don't `--no-verify` git hooks unless asked.
