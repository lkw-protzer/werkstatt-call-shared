//! Validation constants and helpers shared across domain types.

use once_cell::sync::Lazy;
use regex::Regex;

/// Compiled E.164 telephone number regex (`+[1-9]` followed by 1–14 digits).
///
/// Use this in `#[garde(pattern(crate::validation::E164_REGEX))]`.
pub static E164_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\+[1-9]\d{1,14}$").expect("E164_REGEX is valid"));

/// Maximum byte length for free-text note fields (5 000 bytes).
pub const NOTE_TEXT_MAX_LEN: usize = 5_000;

/// Maximum byte length for name fields (200 bytes).
pub const NAME_MAX_LEN: usize = 200;
