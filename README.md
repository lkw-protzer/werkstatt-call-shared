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

## Versionierung

Dieses Crate folgt [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

| Versionstyp | Wann |
|---|---|
| **PATCH** (`0.1.x`) | Bugfixes, Doku-Korrekturen, Tests — keine API-Änderung |
| **MINOR** (`0.x.0`) | Neue Types/Felder, abwärtskompatible Erweiterungen |
| **MAJOR** (`x.0.0`) | Breaking Changes (Felder/Varianten entfernt, Typen umbenannt, Serialisierungsformat geändert) |

**Breaking-Change-Policy:** Vor jedem MAJOR-Bump muss ein ADR im
[`werkstatt-call-docs`](https://github.com/power-del/werkstatt-call-docs)-Repo erstellt und gemergt sein.

Details zum Contribution-Workflow: [`CONTRIBUTING.md`](CONTRIBUTING.md)

## Lizenz

Proprietär — LKW Protzer GmbH & Co. KG
