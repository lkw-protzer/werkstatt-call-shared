//! WebSocket protocol message types exchanged between server and client.

use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::call_event::EnrichedCallEvent;
use crate::call_note::CallNote;
use crate::phone_link::PhoneLink;
use crate::wire::WireFormat;

/// Initial handshake message sent by the client upon WebSocket connection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientHello {
    /// Hostname of the client machine.
    #[garde(skip)]
    pub hostname: String,

    /// Short identifier of the logged-in employee (e.g. `"MS"`).
    #[garde(skip)]
    pub user: String,

    /// List of optional protocol extensions the client supports.
    #[garde(skip)]
    pub extensions: Vec<String>,

    /// Semantic version of the client application (e.g. `"0.1.0"`).
    #[garde(skip)]
    pub client_version: String,

    /// UTC timestamp of when the client established the WebSocket connection.
    #[garde(skip)]
    pub connected_at: DateTime<Utc>,

    /// Preferred wire format for WebSocket messages.
    ///
    /// Defaults to [`WireFormat::Json`] for backwards compatibility — existing
    /// `ClientHello` payloads without this field deserialise as JSON.
    #[garde(skip)]
    #[serde(default)]
    pub preferred_format: WireFormat,
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

    /// A call has been answered (picked up).
    ///
    /// Pushed by the F1.3 event pipeline when Placetel reports an `answered`
    /// event, allowing clients to update the call's visual status (e.g. remove
    /// the ringing animation) without waiting for the `ended` event.
    CallAnswered {
        /// Placetel call identifier of the call that was answered.
        #[serde(rename = "callId")]
        call_id: String,

        /// Login des Mitarbeiters, der angenommen hat (aus device_registry via peer).
        #[serde(rename = "answeringUser")]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ts(optional)]
        answering_user: Option<String>,
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

    /// A phone number to WERBAS customer link has been created or updated.
    PhoneLinkUpdated {
        /// The updated phone link record.
        link: PhoneLink,
    },

    /// Keep-alive heartbeat from the server; clients should respond with `Ping`.
    Heartbeat,
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

    /// Request the call history for a given phone number.
    RequestCallHistory {
        /// Phone number in E.164 format whose call history is requested.
        #[serde(rename = "phoneNumber")]
        phone_number: String,
    },

    /// Client-side keep-alive; the server should respond with `Heartbeat`.
    Ping,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_answered_serialises_answering_user() {
        let e = ServerEvent::CallAnswered {
            call_id: "c1".into(),
            answering_user: Some("florian".into()),
        };
        let j = serde_json::to_string(&e).unwrap();
        assert!(j.contains("\"answeringUser\":\"florian\""));
        assert!(j.contains("\"type\":\"callAnswered\""));
    }

    #[test]
    fn call_answered_omits_answering_user_when_none() {
        let e = ServerEvent::CallAnswered {
            call_id: "c1".into(),
            answering_user: None,
        };
        let j = serde_json::to_string(&e).unwrap();
        assert!(!j.contains("answeringUser"));
        assert!(j.contains("\"callId\":\"c1\""));
    }
}
