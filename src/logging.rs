//! Structured logging for avatara.
//!
//! Set `AVATARA_LOG=debug` (or `trace`, `info`, `warn`, `error`) to enable.
//! Requires the `logging` feature.

/// Initialize the tracing subscriber for development.
///
/// Set `AVATARA_LOG=debug` (or `trace`, `info`, `warn`, `error`) to control
/// the log level. Defaults to `warn` if the environment variable is unset.
///
/// This is a no-op if the subscriber has already been initialized.
/// Requires the `logging` feature.
#[cfg(feature = "logging")]
pub fn try_init() {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_env("AVATARA_LOG").unwrap_or_else(|_| EnvFilter::new("warn"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
