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

    /// Trait weight out of valid range (0.0–1.0).
    #[error("trait weight out of range in {context}: {value}")]
    OutOfRange { context: String, value: f64 },
}

/// Convenience Result type.
pub type Result<T> = std::result::Result<T, AvataraError>;

/// Validate that a trait weight is in the 0.0–1.0 range.
#[inline]
pub fn require_unit_range(value: f64, context: &str) -> Result<f64> {
    if (0.0..=1.0).contains(&value) {
        Ok(value)
    } else {
        Err(AvataraError::OutOfRange {
            context: context.to_string(),
            value,
        })
    }
}

/// Validate that all values in a slice are in the 0.0–1.0 range.
#[inline]
pub fn require_all_unit_range(values: &[f64], context: &str) -> Result<()> {
    for &v in values {
        require_unit_range(v, context)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_unit_range() {
        assert!(require_unit_range(0.0, "test").is_ok());
        assert!(require_unit_range(0.5, "test").is_ok());
        assert!(require_unit_range(1.0, "test").is_ok());
    }

    #[test]
    fn invalid_unit_range() {
        assert!(require_unit_range(-0.1, "test").is_err());
        assert!(require_unit_range(1.1, "test").is_err());
        assert!(require_unit_range(f64::NAN, "test").is_err());
    }

    #[test]
    fn all_unit_range() {
        assert!(require_all_unit_range(&[0.0, 0.5, 1.0], "test").is_ok());
        assert!(require_all_unit_range(&[0.5, 1.5], "test").is_err());
    }
}
