# JamProject Desktop

Wrapper desktop nativo (Windows/Linux/macOS) para [jamproject.net](https://jamproject.net), construído em **Tauri v2** (Rust + WebView2). Carrega o site remoto numa janela própria e expõe **Discord Rich Presence** com o nosso próprio Application ID — `Playing JamProject`, com título da música, dificuldade e timestamps.

## Por que Tauri (e não Electron)

| | Tauri | Electron |
| --- | --- | --- |
| Tamanho do `.exe` | ~10 MB | ~150 MB |
| RAM idle | ~80 MB | ~300 MB |
| Browser engine | WebView2 do SO (já instalado no Win11) | Chromium próprio |
| Linguagem nativa | Rust | Node.js |

## Plano de implementação

### Fase 1 — MVP wrapper (~1 dia)
- [x] Scaffold Tauri v2
- [ ] Janela única abre `https://jamproject.net`
- [ ] Build NSIS (`.exe` instalador) testado em Win11
- [ ] Ícone + splash com identidade JamProject

### Fase 2 — Discord Rich Presence (~1 dia)
- [ ] Registrar Application no [Discord Developer Portal](https://discord.com/developers/applications) → copiar Client ID pra `src-tauri/src/discord_rpc.rs`
- [ ] Subir art assets (logo grande, ícones de dificuldade) no painel Rich Presence
- [ ] Bridge `window.__JL_PRESENCE__` no Angular → `invoke('update_presence', {...})` no Tauri → `discord-rich-presence` crate seta atividade no IPC
- [ ] Auto-clear presence quando user sai de gameplay

### Fase 3 — Polish (~2 dias)
- [ ] Atalho desktop + entrada no menu iniciar (NSIS automático)
- [ ] Auto-updater Tauri (assina releases com chave própria)
- [ ] Tray icon com "Quit" + "Open JamProject"
- [ ] Deep-link `jamproject://song/<id>`
- [ ] Splash de loading enquanto WebView2 inicializa

### Fase 4 — Distribuição
- [ ] Code signing cert (~$100/ano) pra evitar SmartScreen — opcional, MVP sem
- [ ] Página `/download` no site servindo o `.exe` mais recente
- [ ] CI no GitHub Actions: build + sign + release em tag

## Bridge contract (web ↔ tauri)

No Angular (`@core/services/discord-presence.service.ts`, a criar):

```ts
declare global {
  interface Window {
    __TAURI__?: { invoke: (cmd: string, args: object) => Promise<void> };
  }
}

export class DiscordPresenceService {
  private get isDesktop() { return !!window.__TAURI__; }

  setSongPlaying(opts: { title: string; artist: string; difficulty: string; mode: 'TAP'|'STRUM' }) {
    if (!this.isDesktop) return;
    window.__TAURI__!.invoke('update_presence', {
      payload: {
        details: `${opts.title} — ${opts.artist}`,
        state: `${opts.mode} • ${opts.difficulty}`,
        large_image: 'jamproject_logo',
        start_timestamp: Math.floor(Date.now() / 1000),
      },
    });
  }

  clear() {
    if (!this.isDesktop) return;
    window.__TAURI__!.invoke('clear_presence', {});
  }
}
```

## Setup local

**Pré-requisitos:**
1. Instalar Rust: https://www.rust-lang.org/tools/install (`rustup-init.exe`, default toolchain)
2. WebView2 Runtime — já vem no Windows 11
3. Node 22 LTS (já instalado se você roda o monorepo principal)

**Comandos:**
```bash
npm install
npm run tauri:dev      # dev mode — janela abre apontando pra https://jamproject.net
npm run tauri:build    # produz src-tauri/target/release/bundle/nsis/*.exe
```

## Estrutura

```
jamproject-desktop/
├── dist/                          # stub HTML (Tauri exige frontendDist mesmo carregando URL externa)
├── src-tauri/
│   ├── Cargo.toml                 # deps Rust (tauri 2 + discord-rich-presence)
│   ├── tauri.conf.json            # config: janela, bundle NSIS, identifier
│   ├── build.rs                   # tauri-build standard
│   ├── capabilities/default.json  # permissões v2 — só `core:default` + invoke do nosso plugin
│   ├── icons/                     # ⚠️  precisa ícones reais antes de buildar (32/128/256 + .ico + .icns)
│   └── src/
│       ├── main.rs                # entry point
│       ├── lib.rs                 # tauri::Builder + comandos
│       └── discord_rpc.rs         # cliente IPC + state Mutex
└── package.json                   # scripts wrapper (`tauri:dev`, `tauri:build`)
```

## TODO antes do primeiro build

1. **Instalar Rust** (`rustup-init.exe`) na máquina dev.
2. **Gerar ícones** com `npx @tauri-apps/cli icon path/to/logo.png` — preenche `src-tauri/icons/` automaticamente.
3. **Criar Discord Application** em https://discord.com/developers/applications → copiar Client ID pra constante `DISCORD_CLIENT_ID` em `src-tauri/src/discord_rpc.rs`.
4. **Decidir code signing**: pular no MVP (users vão ver SmartScreen warning) ou comprar cert agora.
