# werkstatt-call-shared

Gemeinsame Typen, Event-Schemas und API-Contracts für das LKW Protzer Telefon-Assistent Projekt.

Dieses Repo ist die **Single Source of Truth** für alle Datenstrukturen, die zwischen Server und Client ausgetauscht werden. Änderungen hier wirken sich auf `werkstatt-call-server` und `werkstatt-call-client` aus.

## Inhalt

- **`src/`** — Rust-Crate mit allen gemeinsamen Structs, Enums und Error-Types
- **`schema/`** — OpenAPI 3.1 Spezifikation (generiert + handgepflegt)
- **`events/`** — WebSocket-Event-Definitionen (Server → Client)
- **`scripts/`** — TypeScript-Codegen-Scripts für den Client

## Nutzung

### Als Rust-Dependency

```toml
[dependencies]
werkstatt-call-shared = { git = "ssh://git@github.com/power-del/werkstatt-call-shared.git" }
```

### TypeScript-Types generieren

```bash
cargo run --bin generate-ts > ../werkstatt-call-client/src/lib/types/generated.ts
```

## Verknüpfte Repos

- [`werkstatt-call-server`](https://github.com/power-del/werkstatt-call-server) — konsumiert diese Types im HTTP/WS-Layer
- [`werkstatt-call-client`](https://github.com/power-del/werkstatt-call-client) — konsumiert die generierten TS-Types
- [`werkstatt-call-docs`](https://github.com/power-del/werkstatt-call-docs) — Architektur und ADRs

## Lizenz

Proprietär — LKW Protzer GmbH & Co. KG
