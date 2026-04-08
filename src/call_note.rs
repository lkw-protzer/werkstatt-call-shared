//! Call note types for employee-recorded conversation notes.

use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::validation::NOTE_TEXT_MAX_LEN;

/// Category of a call note classifying the nature of the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
pub enum NoteCategory {
    /// An appointment was agreed upon.
    Termin,
    /// A breakdown or roadside emergency.
    Panne,
    /// A warranty claim or complaint.
    Reklamation,
    /// A quote or offer discussion.
    Angebot,
    /// Anything else not covered by the other categories.
    Sonstiges,
}

/// A conversation note recorded by an employee after a call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CallNote {
    /// The call this note belongs to (Placetel call ID).
    #[garde(skip)]
    pub call_id: String,

    /// UTC timestamp when the note was created.
    #[garde(skip)]
    pub created_at: DateTime<Utc>,

    /// Shorthand identifier of the employee who created the note.
    #[garde(skip)]
    pub created_by: String,

    /// Category classifying the note topic.
    #[garde(skip)]
    pub category: NoteCategory,

    /// Free-text note content (max 5 000 bytes).
    #[garde(length(max = NOTE_TEXT_MAX_LEN))]
    pub text: String,

    /// Whether a follow-up action is required.
    #[garde(skip)]
    pub follow_up: bool,

    /// Optional reference to a WERBAS work-order number.
    #[garde(skip)]
    pub order_ref: Option<String>,
}
