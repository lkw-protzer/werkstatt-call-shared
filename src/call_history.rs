//! Call history response types for the REST API (F4.1).
//!
//! These types are returned by `GET /api/calls/recent` and `GET /api/calls/{id}`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::call_event::CallDirection;
use crate::call_note::CallNote;

/// Summary of a single logical call for the call-history list endpoint.
///
/// Aggregates all `call_log` rows with the same `placetel_call_id` into one entry.
/// Returned by `GET /api/calls/recent`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CallLogSummary {
    /// Placetel call identifier (path parameter for other API calls).
    pub call_id: String,

    /// Inbound or outbound.
    pub direction: CallDirection,

    /// E.164 phone number of the external party.
    pub phone_number: String,

    /// Extension (Nebenstelle) that handled the call, if known.
    pub extension: Option<String>,

    /// UTC timestamp when the call was initiated (from the first `ringing` event).
    pub started_at: Option<DateTime<Utc>>,

    /// UTC timestamp of the most recent event for this call.
    pub received_at: DateTime<Utc>,

    /// Most recent event type seen (e.g. `"ringing"`, `"answered"`, `"ended"`).
    pub latest_event: String,

    /// Whether a conversation note has been recorded for this call.
    pub has_note: bool,
}

/// Full detail view for a single call, including the conversation note if present.
///
/// Returned by `GET /api/calls/{id}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CallDetail {
    /// Placetel call identifier.
    pub call_id: String,

    /// Inbound or outbound.
    pub direction: CallDirection,

    /// E.164 phone number of the external party.
    pub phone_number: String,

    /// Extension (Nebenstelle) that handled the call, if known.
    pub extension: Option<String>,

    /// UTC timestamp when the call was initiated.
    pub started_at: Option<DateTime<Utc>>,

    /// UTC timestamp of the most recent event for this call.
    pub received_at: DateTime<Utc>,

    /// All event types recorded for this call in chronological order
    /// (e.g. `["ringing", "answered", "ended"]`).
    pub events: Vec<String>,

    /// Conversation note recorded by an employee, if any.
    pub note: Option<CallNote>,
}
