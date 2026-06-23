//! Rückruf-Typen für das CTI-Rückruf-Modul (Phase C).

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

/// Ein offener Rückrufauftrag: Kundennummer, Rufnummer und Bearbeitungsstatus.
///
/// Wird sowohl als Command-Payload (Erstellung) als auch als Event-Payload
/// (Bestätigung/Update) über das WebSocket-Protokoll übertragen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Callback {
    /// Eindeutige ID des Rückrufauftrags (UUID als String).
    pub id: String,

    /// Rufnummer, unter der der Kunde zurückgerufen werden soll.
    pub phone_number: String,

    /// Optionale WERBAS-Kundennummer, falls bekannt.
    pub customer_id: Option<String>,

    /// Optionale Notiz zum Rückruf (z. B. Gesprächsanlass).
    pub note: Option<String>,

    /// Login des Mitarbeiters, der den Rückruf angelegt hat.
    pub created_by: String,

    /// Login des Mitarbeiters, dem der Rückruf zugewiesen wurde.
    pub assigned_to: Option<String>,

    /// Bearbeitungsstatus des Rückrufs (z. B. `"open"`, `"claimed"`, `"done"`).
    pub status: String,
}
