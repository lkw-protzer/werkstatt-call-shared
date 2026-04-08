# Anforderungen: werkstatt-call-shared

## Zweck

Dieses Repo enthält alle **gemeinsamen Datenstrukturen**, die zwischen dem Server (`werkstatt-call-server`) und dem Desktop-Client (`werkstatt-call-client`) ausgetauscht werden. Es ist die verbindliche Single Source of Truth — jede Änderung an einem Feld muss hier passieren und wird in beide abhängigen Repos propagiert.

## Projektkontext

Teil des LKW Protzer **Telefon-Assistent**-Projekts. Eingehende Anrufe aus Placetel werden per Webhook am Server empfangen, mit WERBAS-Daten angereichert und per WebSocket an die Desktop-Clients der Mitarbeiter gepusht. Dort erscheint ein natives Popup, in dem der Mitarbeiter die Gesprächsnotiz und Kundenzuordnung pflegt.

## Funktionale Anforderungen

### F1 — Rust-Crate mit gemeinsamen Types

- [ ] `CallEvent` — eingehender Anruf mit allen relevanten Feldern (Placetel-Call-ID, Richtung, E.164-Nummer, Start-Zeit, interne Nebenstelle)
- [ ] `EnrichedCallEvent` — `CallEvent` + WERBAS-Daten (Kunde, Fahrzeuge, offener Auftrag, letzter Kontakt)
- [ ] `PhoneLink` — Zuordnung Telefonnummer → WERBAS-Kunde (+ Kontaktperson, Rolle, Quelle, Confidence)
- [ ] `ContactPerson` — Ansprechpartner bei einem Kunden (Name, Rolle, Notizen)
- [ ] `CallNote` — vom Mitarbeiter erfasste Gesprächsnotiz (Kategorie, Freitext, Follow-up-Flag, Auftrag-Ref)
- [ ] `ClientHello` — Handshake-Message vom Client beim WebSocket-Connect (Hostname, User, Extensions)
- [ ] `ServerEvent` Enum — alle möglichen Server→Client Push-Events
- [ ] `ClientCommand` Enum — alle möglichen Client→Server Befehle

### F2 — Serialisierung

- [ ] `serde::Serialize` + `serde::Deserialize` auf allen Types
- [ ] JSON als primäres Wire-Format, MessagePack als Option für WebSocket-Traffic
- [ ] Alle Felder mit stabilen Namen (`#[serde(rename_all = "camelCase")]` für TS-Kompatibilität)

### F3 — TypeScript-Codegen

- [ ] `specta` oder `ts-rs` integriert
- [ ] Binary `generate-ts` in `src/bin/generate_ts.rs`
- [ ] Output als einzelne `.ts`-Datei mit allen Types
- [ ] CI-Check: wenn Types geändert werden aber Codegen-Artefakt nicht, Fail

### F4 — OpenAPI-Spec

- [ ] REST-Endpunkte via `utoipa` in Rust annotiert (im Server-Repo, hier die Schemas)
- [ ] Zentrale Schema-Definitionen in `schema/openapi.yaml`
- [ ] Swagger-UI kann daraus generiert werden

### F5 — Validation-Regeln

- [ ] `garde` oder `validator` Derives auf allen Input-Types
- [ ] Telefonnummern: E.164-Format erzwingen (`+[1-9]\d{1,14}`)
- [ ] Max-Längen für Freitextfelder (Notiz: 5000, Name: 200)

### F6 — Versionierung

- [ ] Semantic Versioning
- [ ] Breaking Changes erfordern MAJOR-Bump und ADR im `werkstatt-call-docs`-Repo
- [ ] Changelog in `CHANGELOG.md` pflegen (Keep-a-Changelog Format)

## Nicht-funktionale Anforderungen

### N1 — Stabilität

- Diese Crate darf **keine** IO-Operationen enthalten (kein HTTP, kein DB, kein Dateisystem)
- Nur Types, Validierung, (De)Serialisierung
- `#![forbid(unsafe_code)]`
- `#![deny(missing_docs)]` auf Public API

### N2 — Build

- `cargo clippy -- -D warnings` muss grün sein
- `cargo fmt --check` muss grün sein
- `cargo test` mit Roundtrip-Tests (Serialize → Deserialize → Equality)
- MSRV (Minimum Supported Rust Version): stable, rolling

### N3 — Dependencies

- Minimal halten
- Nur: `serde`, `serde_json`, `chrono`, `uuid`, `thiserror`, `garde`, `specta` oder `ts-rs`, `utoipa`

## Cross-Repo-Verknüpfung

> ⚠️ **Wichtig:** Diese Verknüpfungen zwischen den Repos müssen **immer** erhalten bleiben. Wenn du Informationen aus einem anderen Repo brauchst oder eine Entscheidung mehrere Repos betrifft, erstelle ein Issue im zuständigen Repo und verlinke es hier.

| Betrifft | Repo | Was |
|---|---|---|
| **Architektur-Entscheidungen** | [`werkstatt-call-docs`](https://github.com/power-del/werkstatt-call-docs) | Bei Breaking-Change-Überlegungen zuerst ADR schreiben |
| **Server verwendet diese Types** | [`werkstatt-call-server`](https://github.com/power-del/werkstatt-call-server) | Bei Änderung: Server-Build testen, Issue anlegen mit `needs-update` Label |
| **Client konsumiert generierte TS-Types** | [`werkstatt-call-client`](https://github.com/power-del/werkstatt-call-client) | Nach Codegen: Client muss neu bauen, Issue mit `needs-update` |
| **WERBAS-Datenformat** | werkstatt-api (bestehend) | Feldnamen und Typen müssen zu WERBAS passen — bei Unsicherheit Issue hier |

### Cross-Repo-Issue-Policy

1. **Brauche ich Infos aus einem anderen Repo?** → Issue dort eröffnen, Label `cross-repo-question`, zurück-verlinken hier.
2. **Ändere ich etwas, das andere Repos beeinflusst?** → In jedem betroffenen Repo ein Tracking-Issue öffnen mit Label `needs-update` und Link auf den PR hier.
3. **Gibt es ein Problem, das mehrere Repos betrifft?** → Meta-Issue im `werkstatt-call-docs`-Repo mit Links auf alle betroffenen.

## Definition of Done

- [ ] Alle F-Anforderungen implementiert
- [ ] Tests grün (`cargo test`)
- [ ] Clippy grün (`cargo clippy -- -D warnings`)
- [ ] TS-Codegen erzeugt funktionierende Types (im Client importierbar)
- [ ] Changelog aktualisiert
- [ ] Tag `v0.1.0` gesetzt und gepusht
