# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `ClientHello` now derives `garde::Validate` (all fields `#[garde(skip)]`) — completes F5.1 acceptance criteria (issue #7)

### Changed

- Regenerated `generated/types.ts` to reflect latest Rust types (`ClientHello` fields `user`/`clientVersion`/`connectedAt`; `ServerEvent` variants `phoneLinkUpdated`/`heartbeat`; `ClientCommand` variants `requestCallHistory`/`ping`)
- Updated `schema/openapi.yaml` to match current types: `ClientHello` now uses `user` instead of `userShorthand` and includes `clientVersion`/`connectedAt`; `ServerEvent` and `ClientCommand` schemas include all new variants

## [0.1.0] — 2026-04-08

### Added

#### Core domain types (F1.1)
- `CallDirection` enum: `Inbound`, `Outbound`
- `CallEvent` struct: `call_id`, `direction`, `phone_number` (E.164), `started_at`, `extension`
- `Customer` struct: `id`, `name` (WERBAS customer reference)
- `Vehicle` struct: `vehicle_id`, `label`, `license_plate`
- `EnrichedCallEvent` struct: wraps `CallEvent` with optional WERBAS enrichment
- `NoteCategory` enum: `Termin`, `Panne`, `Reklamation`, `Angebot`, `Sonstiges`
- `CallNote` struct: `call_id`, `created_at`, `created_by`, `category`, `text`, `follow_up`, `order_ref`
- `LinkSource` enum: `Manual`, `AutoWerbas`, `Import`
- `ContactPerson` struct: `name`, `role`, `notes`
- `PhoneLink` struct: `phone_number`, `customer_id`, `contact_person`, `role`, `source`, `confidence`

#### WebSocket protocol types (F1.2)
- `ClientHello` struct: initial handshake from client
- `ServerEvent` tagged enum: `IncomingCall`, `CallEnded`, `NoteConfirmed`, `Error`
- `ClientCommand` tagged enum: `SaveNote`, `LinkPhone`, `Ack`

#### Serialization (F2)
- All types derive `serde::Serialize` + `serde::Deserialize`
- All struct fields serialized as `camelCase` for TypeScript compatibility
- JSON as primary wire format (`serde_json`)

#### TypeScript codegen (F3)
- `ts-rs` integrated; all types derive `TS`
- `src/bin/generate_ts.rs` binary generates a single `generated/types.ts`
- `generated/types.ts` committed as canonical artifact

#### OpenAPI schemas (F4)
- `utoipa` integrated; all types derive `ToSchema`
- `schema/openapi.yaml` with OpenAPI 3.1.0 component schemas for all types
- E.164 regex patterns included in schema for phone number fields

#### Validation (F5)
- `garde` integrated; all input types derive `Validate`
- E.164 pattern enforcement on `phone_number` fields
- Maximum 5 000 bytes on `CallNote.text`
- Maximum 200 bytes on `ContactPerson.name`
- Confidence range `[0.0, 1.0]` on `PhoneLink.confidence`

#### Safety and quality (N1, N2)
- `#![forbid(unsafe_code)]` in `src/lib.rs`
- `#![deny(missing_docs)]` in `src/lib.rs`; all public items documented
- 35 integration tests in `tests/roundtrip.rs` (roundtrip + validation boundary tests)
- CI workflow: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, codegen freshness check

#### Versioning (F6)
- Crate version `0.1.0` with MSRV `1.75`
- This changelog in Keep-a-Changelog format

[0.1.0]: https://github.com/power-del/werkstatt-call-shared/releases/tag/v0.1.0
