# Contributing to werkstatt-call-shared

Dieses Dokument beschreibt, wie Änderungen an `werkstatt-call-shared` vorgenommen werden — dem zentralen
Shared-Types-Crate für Server und Client.

---

## Semantic Versioning

Dieses Crate folgt [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

### Versionierungsregeln

| Versionstyp | Wann anwenden | Beispiele |
|---|---|---|
| **PATCH** (`0.1.x`) | Bugfixes, Doku-Korrekturen, Test-Ergänzungen, interne Refactors ohne API-Änderung | Typo in Docs fixen, Test hinzufügen, Clippy-Warning beheben |
| **MINOR** (`0.x.0`) | Neue Types, neue Felder (additive), abwärtskompatible Erweiterungen | Neuer Struct, neues optionales Feld, neue Enum-Variante (non-exhaustive), neue Validation-Regel ohne bestehende Änderung |
| **MAJOR** (`x.0.0`) | Breaking Changes | Feld entfernen oder umbenennen, Enum-Variante entfernen, Typ-Signatur ändern, serialisiertes Format inkompatibel machen |

> **Hinweis:** Da Server und Client dieses Crate als Git-Dependency verwenden, ist jede Änderung, die
> bestehende `Deserialize`-Aufrufe kaputt macht, ein Breaking Change — auch wenn Rust selbst noch kompiliert.

---

## Breaking-Change-Policy

Vor einem MAJOR-Version-Bump (`x.0.0`) **muss** ein ADR (Architecture Decision Record) im
[`werkstatt-call-docs`](https://github.com/power-del/werkstatt-call-docs)-Repo erstellt und gemergt worden sein.

### Breaking-Change-Workflow

1. **ADR öffnen** — Neue Datei `adr/NNNN-<slug>.md` in `werkstatt-call-docs` anlegen (nach dem Muster
   der bestehenden ADRs, z.B. `0006-hmac-webhook-auth.md`). Status initial `proposed`.
2. **ADR reviewen und mergen** — Der ADR muss reviewed und mit Status `accepted` gemergt sein, bevor
   der Breaking Change implementiert wird.
3. **MAJOR-Bump in `Cargo.toml`** — Version erhöhen, z.B. `1.0.0` → `2.0.0`.
4. **CHANGELOG aktualisieren** — Abschnitt `[Unreleased]` um `### Removed` / `### Changed`-Einträge
   ergänzen. Beim Release wird `[Unreleased]` zu `[x.0.0] — YYYY-MM-DD`.
5. **Tracking-Issues öffnen** — In `werkstatt-call-server` und `werkstatt-call-client` je ein Issue
   mit dem Label `needs-update` öffnen, das die betroffenen Types/Felder beschreibt.
6. **Git-Tag setzen** — Nach dem Merge: `git tag vX.0.0 && git push origin vX.0.0`.

---

## Eine Änderung vornehmen

### Ablauf

1. **Rust-Types editieren** in `src/`
2. **Codegen ausführen:**
   ```bash
   cargo run --bin generate-ts
   # Prüfen: generated/types.ts ist aktuell
   cargo run --bin generate-openapi
   # Prüfen: schema/openapi.yaml ist aktuell
   ```
3. **Qualitätsgates grün stellen:**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```
4. **CHANGELOG.md** — `[Unreleased]`-Abschnitt mit dem passenden Typ (`Added` / `Changed` / `Fixed` /
   `Removed`) ergänzen
5. **PR öffnen**

### Codegen-Freshness

Der CI-Workflow prüft, dass `generated/types.ts` und `schema/openapi.yaml` mit den aktuellen Rust-Types
übereinstimmen. Falls die Freshness-Prüfung fehlschlägt: Codegen lokal ausführen und die generierten
Dateien committen.

---

## Cross-Repo-Updates

Nach einer Änderung, die Downstream-Repos betrifft:

| Repo | Aktion |
|---|---|
| `werkstatt-call-server` | Issue mit Label `needs-update` öffnen; `rev =` Pin in `Cargo.toml` auf den neuen Commit-SHA oder Tag aktualisieren |
| `werkstatt-call-client` | Issue mit Label `needs-update` öffnen; generierten TS-Import aus `generated/types.ts` neu einspielen |

### Dependency-Pin aktualisieren (Server)

`werkstatt-call-server/Cargo.toml` pinnt die Shared-Types derzeit auf einen Git-Commit:

```toml
werkstatt-call-shared = { git = "https://github.com/power-del/werkstatt-call-shared.git", rev = "<SHA>" }
```

Für einen stabilen Release-Pin statt eines Commit-SHA kann auf einen Tag verwiesen werden:

```toml
werkstatt-call-shared = { git = "https://github.com/power-del/werkstatt-call-shared.git", tag = "v0.1.0" }
```

---

## Einen Release erstellen

1. Alle Änderungen gemergt, alle Qualitätsgates grün.
2. `CHANGELOG.md`: `[Unreleased]` in `[x.y.z] — YYYY-MM-DD` umbenennen, neuen leeren `[Unreleased]`-
   Abschnitt oben einfügen, Reference-Link-Footer ergänzen.
3. `Cargo.toml`: Version auf `x.y.z` setzen.
4. Commit: `git commit -m "chore: release v x.y.z"`
5. Tag: `git tag vx.y.z && git push origin vx.y.z`
6. GitHub Release aus dem Tag erstellen (aktiviert den CHANGELOG-Reference-Link).

---

## Nicht-funktionale Anforderungen

Gemäß `Anforderung.md`:

- `#![forbid(unsafe_code)]` — kein `unsafe` erlaubt
- `#![deny(missing_docs)]` — alle öffentlichen Items müssen dokumentiert sein
- Kein IO, kein HTTP, keine Async-Operationen — nur Types, Validation, (De)Serialisierung
- MSRV: Rust `1.75` (stable, rolling)
