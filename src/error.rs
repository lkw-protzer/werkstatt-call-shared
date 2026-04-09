//! Crate-level error type.

/// Errors that can occur within the shared crate.
#[derive(Debug, thiserror::Error)]
pub enum SharedError {
    /// A value failed serialization.
    #[error("serialization error: {0}")]
    Serialization(String),

    /// A value failed deserialization.
    #[error("deserialization error: {0}")]
    Deserialization(String),

    /// The requested wire format is not supported (feature not enabled).
    #[error("unsupported wire format: {0}")]
    UnsupportedFormat(String),

    /// A value failed validation.
    #[error("validation error: {0}")]
    Validation(String),
}
