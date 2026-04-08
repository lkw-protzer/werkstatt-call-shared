//! Call event types representing incoming/outgoing telephone calls.

use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::validation::E164_REGEX;

/// Direction of a telephone call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
pub enum CallDirection {
    /// An inbound call received from an external caller.
    Inbound,
    /// An outbound call placed to an external number.
    Outbound,
}

/// A raw telephone call event as received from Placetel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CallEvent {
    /// Unique Placetel call identifier.
    #[garde(skip)]
    pub call_id: String,

    /// Direction of the call (inbound or outbound).
    #[garde(skip)]
    pub direction: CallDirection,

    /// Caller/callee phone number in E.164 format (e.g. `+4989123456789`).
    #[garde(pattern(E164_REGEX))]
    pub phone_number: String,

    /// UTC timestamp when the call was established.
    #[garde(skip)]
    pub started_at: DateTime<Utc>,

    /// Internal ISDN extension that received or placed the call.
    #[garde(skip)]
    pub extension: String,
}

/// A WERBAS customer record associated with a call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    /// WERBAS customer identifier.
    pub id: String,

    /// Customer display name.
    pub name: String,

    /// WERBAS customer number (Kundennummer), if available.
    pub customer_number: Option<String>,

    /// City of the customer's primary address (Ort), if available.
    pub city: Option<String>,
}

/// A vehicle from WERBAS associated with a customer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    /// WERBAS vehicle identifier.
    pub vehicle_id: String,

    /// Human-readable vehicle label (e.g. `"Mercedes Actros 2023"`).
    pub label: String,

    /// License plate of the vehicle, if known.
    pub license_plate: Option<String>,

    /// Date of the next mandatory vehicle inspection (Hauptuntersuchung/HU).
    pub next_hu_date: Option<DateTime<Utc>>,

    /// Date of the next exhaust emissions test (Abgasuntersuchung/AU).
    pub next_au_date: Option<DateTime<Utc>>,
}

/// An open work order in WERBAS (Offener Auftrag).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    /// WERBAS work order identifier.
    pub id: String,

    /// Human-readable order number shown in WERBAS.
    pub order_number: String,

    /// Short description of the work order, if available.
    pub description: Option<String>,

    /// Current status of the order (e.g. `"Offen"`, `"In Arbeit"`, `"Fertig"`).
    pub status: String,
}

/// An open financial item (Offener Posten) associated with a customer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenItem {
    /// WERBAS invoice or document identifier.
    pub id: String,

    /// Outstanding amount in EUR (positive = customer owes the workshop).
    pub amount_eur: f64,

    /// Short label (e.g. invoice number or description).
    pub label: Option<String>,
}

/// Last contact record for a customer (Letzter Kontakt).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LastContact {
    /// UTC timestamp of the last contact.
    pub date: DateTime<Utc>,

    /// Human-readable contact type (e.g. `"Anruf"`, `"E-Mail"`, `"Termin"`).
    pub contact_type: String,
}

/// A candidate customer match returned when a phone number is unresolved.
///
/// Used to populate the suggestion list when `phone_links` has no entry for
/// the caller's number. Candidates are ranked by area-code proximity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSuggestion {
    /// WERBAS customer identifier.
    pub customer_id: String,

    /// Customer display name, if known from `phone_links` or WERBAS cache.
    pub name: Option<String>,

    /// Known phone number for this customer in E.164 format.
    pub phone_e164: String,

    /// Human-readable reason for the match (e.g. `"area_code_match"`).
    pub match_reason: String,
}

/// A call event enriched with WERBAS customer and vehicle data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedCallEvent {
    /// The underlying raw call event.
    #[garde(dive)]
    pub call_event: CallEvent,

    /// Matched WERBAS customer, if a match was found.
    #[garde(skip)]
    pub customer: Option<Customer>,

    /// Vehicles associated with the matched customer.
    #[garde(skip)]
    pub vehicles: Vec<Vehicle>,

    /// Open work orders in WERBAS for the matched customer.
    #[garde(skip)]
    pub open_orders: Vec<OpenOrder>,

    /// Structured last contact record for this customer, if known.
    #[garde(skip)]
    pub last_contact: Option<LastContact>,

    /// Open financial items (Offene Posten) for the matched customer.
    #[garde(skip)]
    pub open_items: Vec<OpenItem>,

    /// Whether the caller could not be matched to a WERBAS customer.
    ///
    /// When `true`, `customer` / `vehicles` / `open_orders` / `open_items`
    /// are empty and `suggestions` may contain candidate matches.
    #[garde(skip)]
    pub unresolved: bool,

    /// Candidate customer matches when `unresolved = true`.
    ///
    /// Ranked by area-code proximity (Ortsvorwahl-Heuristik). Empty when
    /// the caller was resolved or when no candidates could be found.
    #[garde(skip)]
    pub suggestions: Vec<CustomerSuggestion>,

    /// WERBAS deep-link URL for this customer, constructed by the server.
    ///
    /// `None` until the server has a WERBAS base URL configured
    /// (see issue #9 / werkstatt-call-server settings). The client falls back
    /// to a placeholder when `None`.
    #[garde(skip)]
    pub werbas_url: Option<String>,
}
