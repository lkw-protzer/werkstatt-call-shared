//! Phone number to WERBAS customer link types.

use garde::Validate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::validation::{E164_REGEX, NAME_MAX_LEN};

/// Source of a phone-number-to-customer link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
pub enum LinkSource {
    /// Link was created manually by a user.
    Manual,
    /// Link was resolved automatically via WERBAS lookup.
    AutoWerbas,
    /// Link was created via a bulk import.
    Import,
}

/// A contact person associated with a customer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContactPerson {
    /// Full name of the contact person (max 200 bytes).
    #[garde(length(max = NAME_MAX_LEN))]
    pub name: String,

    /// Role or job title of the contact person.
    #[garde(skip)]
    pub role: Option<String>,

    /// Additional notes about this contact.
    #[garde(skip)]
    pub notes: Option<String>,
}

/// A mapping from a phone number to a WERBAS customer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PhoneLink {
    /// Phone number in E.164 format (e.g. `+4989123456789`).
    #[garde(pattern(E164_REGEX))]
    pub phone_number: String,

    /// WERBAS customer identifier this number is linked to.
    #[garde(skip)]
    pub customer_id: String,

    /// Optional contact person at the customer site.
    #[garde(dive)]
    pub contact_person: Option<ContactPerson>,

    /// Role of the contact in the context of this link.
    #[garde(skip)]
    pub role: Option<String>,

    /// How this link was established.
    #[garde(skip)]
    pub source: LinkSource,

    /// Confidence score of the automatic match, in the range `[0.0, 1.0]`.
    #[garde(range(min = 0.0, max = 1.0))]
    pub confidence: f64,
}
