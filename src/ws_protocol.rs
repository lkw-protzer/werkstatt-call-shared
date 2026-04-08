//! WebSocket protocol message types exchanged between server and client.

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::call_event::EnrichedCallEvent;
use crate::call_note::CallNote;

/// Initial handshake message sent by the client upon WebSocket connection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientHello {
    /// Hostname of the client machine.
    pub hostname: String,

    /// Short identifier of the logged-in employee (e.g. `"MS"`).
    pub user_shorthand: String,

    /// List of optional protocol extensions the client supports.
    pub extensions: Vec<String>,
}

/// Events pushed from the server to connected clients.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
pub enum ServerEvent {
    /// An incoming or outgoing call has been established.
    IncomingCall {
        /// The enriched call event with WERBAS data.
        event: EnrichedCallEvent,
    },

    /// A call has ended.
    CallEnded {
        /// Placetel call identifier of the call that ended.
        #[serde(rename = "callId")]
        call_id: String,
    },

    /// A call note has been confirmed and persisted by the server.
    NoteConfirmed {
        /// Placetel call identifier whose note was saved.
        #[serde(rename = "callId")]
        call_id: String,
    },

    /// A server-side error has occurred.
    Error {
        /// Human-readable error description.
        message: String,
    },
}

/// Commands sent from a client to the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
pub enum ClientCommand {
    /// Save a call note for a specific call.
    SaveNote {
        /// Placetel call identifier this note belongs to.
        #[serde(rename = "callId")]
        call_id: String,
        /// The note to persist.
        note: CallNote,
    },

    /// Link a phone number to a WERBAS customer.
    LinkPhone {
        /// Phone number in E.164 format.
        #[serde(rename = "phoneNumber")]
        phone_number: String,
        /// WERBAS customer identifier to link to.
        #[serde(rename = "customerId")]
        customer_id: String,
    },

    /// Acknowledge receipt of a server event.
    Ack {
        /// Placetel call identifier of the acknowledged event.
        #[serde(rename = "callId")]
        call_id: String,
    },
}
