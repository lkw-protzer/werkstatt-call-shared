//! WebSocket protocol message types exchanged between server and client.

use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::call_event::EnrichedCallEvent;
use crate::call_note::CallNote;
use crate::callback::Callback;
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

    /// Ergebnis einer CTI-Aktion (Annehmen, Ablehnen, Transferieren, Wählen).
    ///
    /// Wird nach Abschluss einer über [`ClientCommand`] ausgelösten CTI-Operation
    /// an den aufrufenden Client zurückgesendet.
    CtiResult {
        /// Placetel-Call-Identifier, auf den sich die Aktion bezog.
        #[serde(rename = "callId")]
        call_id: String,

        /// Bezeichner der ausgeführten Operation: `"answer"`, `"decline"`,
        /// `"transfer"` oder `"dial"`.
        op: String,

        /// `true`, wenn die CTI-Aktion erfolgreich war; sonst `false`.
        ok: bool,

        /// Optionale Fehlerbeschreibung oder Zusatzinformation.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ts(optional)]
        detail: Option<String>,
    },

    /// Ein neuer Rückrufauftrag wurde angelegt und an alle verbundenen Clients
    /// verteilt.
    CallbackCreated {
        /// Der neu angelegte Rückrufauftrag.
        callback: Callback,
    },

    /// Ein bestehender Rückrufauftrag wurde geändert (z. B. Zuweisung oder
    /// Statuswechsel) und wird an alle Clients neu übertragen.
    CallbackUpdated {
        /// Der aktualisierte Rückrufauftrag.
        callback: Callback,
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

    /// Request the call history for a given phone number.
    RequestCallHistory {
        /// Phone number in E.164 format whose call history is requested.
        #[serde(rename = "phoneNumber")]
        phone_number: String,
    },

    /// Client-side keep-alive; the server should respond with `Heartbeat`.
    Ping,

    /// Anforderung, einen laufenden Anruf anzunehmen (CTI-Aktion).
    AnswerCall {
        /// Placetel-Call-Identifier des anzunehmenden Anrufs.
        #[serde(rename = "callId")]
        call_id: String,
    },

    /// Anforderung, einen eingehenden Anruf abzulehnen (CTI-Aktion).
    DeclineCall {
        /// Placetel-Call-Identifier des abzulehnenden Anrufs.
        #[serde(rename = "callId")]
        call_id: String,
    },

    /// Anforderung, einen laufenden Anruf zu einem anderen Ziel weiterzuleiten (CTI-Aktion).
    TransferCall {
        /// Placetel-Call-Identifier des weiterzuleitenden Anrufs.
        #[serde(rename = "callId")]
        call_id: String,

        /// Zielrufnummer oder SIP-Adresse für die Weiterleitung.
        target: String,
    },

    /// Neuen Rückrufauftrag anlegen.
    ///
    /// Der Server legt den Auftrag in der Datenbank an und verteilt ein
    /// [`ServerEvent::CallbackCreated`] an alle verbundenen Clients.
    CreateCallback {
        /// Rufnummer des Kunden in E.164-Format.
        #[serde(rename = "phoneNumber")]
        phone_number: String,

        /// Optionale WERBAS-Kundennummer, falls bereits bekannt.
        #[serde(rename = "customerId")]
        customer_id: Option<String>,

        /// Optionale Notiz zum Rückruf (z. B. Gesprächsanlass).
        note: Option<String>,

        /// Login des Mitarbeiters, dem der Rückruf direkt zugewiesen werden soll.
        #[serde(rename = "assignedTo")]
        assigned_to: Option<String>,
    },

    /// Rückrufauftrag für den eigenen Benutzer beanspruchen (Status → `"claimed"`).
    ClaimCallback {
        /// Eindeutige ID des zu beanspruchenden Rückrufauftrags.
        id: String,
    },

    /// Rückrufauftrag per CTI-Wählen ausführen.
    ///
    /// Der Server löst einen ausgehenden Anruf auf die im Auftrag hinterlegte
    /// Rufnummer aus und setzt den Status auf `"in_progress"`.
    DialCallback {
        /// Eindeutige ID des auszuführenden Rückrufauftrags.
        id: String,
    },
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

    #[test]
    fn cti_result_serialises_with_detail() {
        let e = ServerEvent::CtiResult {
            call_id: "c42".into(),
            op: "answer".into(),
            ok: true,
            detail: Some("accepted".into()),
        };
        let j = serde_json::to_string(&e).unwrap();
        assert!(j.contains("\"type\":\"ctiResult\""));
        assert!(j.contains("\"callId\":\"c42\""));
        assert!(j.contains("\"op\":\"answer\""));
        assert!(j.contains("\"ok\":true"));
        assert!(j.contains("\"detail\":\"accepted\""));
    }

    #[test]
    fn cti_result_omits_detail_when_none() {
        let e = ServerEvent::CtiResult {
            call_id: "c42".into(),
            op: "decline".into(),
            ok: false,
            detail: None,
        };
        let j = serde_json::to_string(&e).unwrap();
        assert!(j.contains("\"type\":\"ctiResult\""));
        assert!(!j.contains("detail"));
    }

    #[test]
    fn answer_call_deserialises_from_json() {
        let cmd: ClientCommand =
            serde_json::from_str(r#"{"type":"answerCall","callId":"abc"}"#).unwrap();
        assert_eq!(
            cmd,
            ClientCommand::AnswerCall {
                call_id: "abc".into()
            }
        );
    }

    #[test]
    fn transfer_call_deserialises_from_json() {
        let cmd: ClientCommand =
            serde_json::from_str(r#"{"type":"transferCall","callId":"xyz","target":"+4989123"}"#)
                .unwrap();
        assert_eq!(
            cmd,
            ClientCommand::TransferCall {
                call_id: "xyz".into(),
                target: "+4989123".into(),
            }
        );
    }

    #[test]
    fn create_callback_deserialises_full() {
        let cmd: ClientCommand = serde_json::from_str(
            r#"{"type":"createCallback","phoneNumber":"+49301234567","customerId":"C1","note":"Bremsen","assignedTo":null}"#,
        )
        .unwrap();
        assert_eq!(
            cmd,
            ClientCommand::CreateCallback {
                phone_number: "+49301234567".into(),
                customer_id: Some("C1".into()),
                note: Some("Bremsen".into()),
                assigned_to: None,
            }
        );
    }

    #[test]
    fn create_callback_deserialises_minimal() {
        // Fehlende optionale Felder werden als None deserialisiert.
        let cmd: ClientCommand =
            serde_json::from_str(r#"{"type":"createCallback","phoneNumber":"+4930999"}"#).unwrap();
        assert_eq!(
            cmd,
            ClientCommand::CreateCallback {
                phone_number: "+4930999".into(),
                customer_id: None,
                note: None,
                assigned_to: None,
            }
        );
    }

    #[test]
    fn claim_callback_roundtrip() {
        let cmd = ClientCommand::ClaimCallback { id: "abc".into() };
        let j = serde_json::to_string(&cmd).unwrap();
        assert!(j.contains("\"type\":\"claimCallback\""));
        assert!(j.contains("\"id\":\"abc\""));
        let back: ClientCommand = serde_json::from_str(&j).unwrap();
        assert_eq!(cmd, back);
    }

    #[test]
    fn dial_callback_roundtrip() {
        let cmd = ClientCommand::DialCallback { id: "xyz".into() };
        let j = serde_json::to_string(&cmd).unwrap();
        assert!(j.contains("\"type\":\"dialCallback\""));
        assert!(j.contains("\"id\":\"xyz\""));
        let back: ClientCommand = serde_json::from_str(&j).unwrap();
        assert_eq!(cmd, back);
    }

    #[test]
    fn callback_struct_roundtrip() {
        use crate::callback::Callback;
        let cb = Callback {
            id: "uuid-1".into(),
            phone_number: "+49301234567".into(),
            customer_id: Some("KUND-42".into()),
            note: Some("Bitte zurückrufen".into()),
            created_by: "MS".into(),
            assigned_to: Some("FP".into()),
            status: "open".into(),
        };
        let j = serde_json::to_string(&cb).unwrap();
        // Struct-level camelCase rename_all — keine per-Feld-Annotationen nötig.
        assert!(j.contains("\"phoneNumber\""));
        assert!(j.contains("\"customerId\""));
        assert!(j.contains("\"createdBy\""));
        assert!(j.contains("\"assignedTo\""));
        let back: Callback = serde_json::from_str(&j).unwrap();
        assert_eq!(cb, back);
    }

    #[test]
    fn callback_created_serialises() {
        use crate::callback::Callback;
        let e = ServerEvent::CallbackCreated {
            callback: Callback {
                id: "uuid-2".into(),
                phone_number: "+49301234567".into(),
                customer_id: Some("KUND-42".into()),
                note: None,
                created_by: "MS".into(),
                assigned_to: None,
                status: "open".into(),
            },
        };
        let j = serde_json::to_string(&e).unwrap();
        assert!(j.contains("\"type\":\"callbackCreated\""));
        assert!(j.contains("\"phoneNumber\""));
        assert!(j.contains("\"createdBy\""));
        assert!(j.contains("\"status\":\"open\""));
        assert!(j.contains("\"customerId\""));
        // Fehlende optionale Felder werden nicht serialisiert (serde default).
        assert!(!j.contains("\"assignedTo\"") || j.contains("\"assignedTo\":null"));
    }
}
