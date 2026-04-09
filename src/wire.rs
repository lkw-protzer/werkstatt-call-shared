//! Wire format selection and encode/decode helpers.
//!
//! Use [`encode`] and [`decode`] to serialise/deserialise any serde-compatible
//! type in either JSON or MessagePack format without performing any I/O.

use serde::{de::DeserializeOwned, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::error::SharedError;

/// Selects the wire format for WebSocket messages.
///
/// The client signals its preference in [`crate::ws_protocol::ClientHello`].
/// JSON is the default and always available. MessagePack requires the `msgpack`
/// Cargo feature to be enabled.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, TS, ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum WireFormat {
    /// JSON text frames. Always available. Default.
    #[default]
    Json,
    /// MessagePack binary frames. Requires the `msgpack` Cargo feature.
    MsgPack,
}

/// Encode `msg` into bytes using the specified wire format.
///
/// Returns [`SharedError::Serialization`] on encode failure.
/// Returns [`SharedError::UnsupportedFormat`] when [`WireFormat::MsgPack`] is
/// requested but the `msgpack` Cargo feature is not enabled.
pub fn encode<T: Serialize>(msg: &T, format: WireFormat) -> Result<Vec<u8>, SharedError> {
    match format {
        WireFormat::Json => {
            serde_json::to_vec(msg).map_err(|e| SharedError::Serialization(e.to_string()))
        }
        WireFormat::MsgPack => {
            #[cfg(feature = "msgpack")]
            {
                rmp_serde::to_vec_named(msg).map_err(|e| SharedError::Serialization(e.to_string()))
            }
            #[cfg(not(feature = "msgpack"))]
            {
                Err(SharedError::UnsupportedFormat(
                    "MessagePack requires the `msgpack` Cargo feature".into(),
                ))
            }
        }
    }
}

/// Decode bytes into `T` using the specified wire format.
///
/// Returns [`SharedError::Deserialization`] on decode failure.
/// Returns [`SharedError::UnsupportedFormat`] when [`WireFormat::MsgPack`] is
/// requested but the `msgpack` Cargo feature is not enabled.
pub fn decode<T: DeserializeOwned>(bytes: &[u8], format: WireFormat) -> Result<T, SharedError> {
    match format {
        WireFormat::Json => {
            serde_json::from_slice(bytes).map_err(|e| SharedError::Deserialization(e.to_string()))
        }
        WireFormat::MsgPack => {
            #[cfg(feature = "msgpack")]
            {
                rmp_serde::from_slice(bytes)
                    .map_err(|e| SharedError::Deserialization(e.to_string()))
            }
            #[cfg(not(feature = "msgpack"))]
            {
                Err(SharedError::UnsupportedFormat(
                    "MessagePack requires the `msgpack` Cargo feature".into(),
                ))
            }
        }
    }
}
