//! Crate-level error type.

/// Errors that can occur within the shared crate.
#[derive(Debug, thiserror::Error)]
pub enum SharedError {
    /// A value failed serialization or deserialization.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// A value failed validation.
    #[error("validation error: {0}")]
    Validation(String),
}
