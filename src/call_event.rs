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

    /// Open work-order identifier in WERBAS, if any.
    #[garde(skip)]
    pub open_order: Option<String>,

    /// UTC timestamp of the last contact with this customer, if known.
    #[garde(skip)]
    pub last_contact: Option<DateTime<Utc>>,

    /// Whether the customer has unresolved open orders.
    #[garde(skip)]
    pub unresolved: bool,
}
