//! Error types for avatara.

use serde::{Deserialize, Serialize};

/// Errors that can occur in avatara operations.
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AvataraError {
    /// Invalid parameter value.
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    /// Unknown archetype name.
    #[error("unknown archetype: {0}")]
    UnknownArchetype(String),

    /// Incompatible archetype combination.
    #[error("incompatible archetypes: {0}")]
    Incompatible(String),
}

/// Convenience Result type.
pub type Result<T> = std::result::Result<T, AvataraError>;
