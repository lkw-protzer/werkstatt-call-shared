//! # werkstatt-call-shared
//!
//! Shared data types for the werkstatt-call system.
//!
//! This crate is the single source of truth for all types exchanged between
//! `werkstatt-call-server` and `werkstatt-call-client`. It contains only
//! types, validation, and (de)serialization — no I/O, no HTTP, no database.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod call_event;
pub mod call_history;
pub mod call_note;
pub mod callback;
pub mod error;
pub mod phone_link;
pub mod validation;
pub mod wire;
pub mod ws_protocol;

pub use call_event::{
    CallDirection, CallEvent, Customer, CustomerSuggestion, EnrichedCallEvent, LastContact,
    OpenItem, OpenOrder, Vehicle,
};
pub use call_history::{CallDetail, CallLogSummary};
pub use call_note::{CallNote, NoteCategory};
pub use callback::Callback;
pub use error::SharedError;
pub use phone_link::{ContactPerson, LinkSource, PhoneLink};
pub use wire::{decode, encode, WireFormat};
pub use ws_protocol::{ClientCommand, ClientHello, ServerEvent};
