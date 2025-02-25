use tracing_subscriber::{fmt, EnvFilter};

/// Centralized logger initialization
pub fn init_tracing_once() {
    // Initialize logger with local time formatting and env-based filtering
    tracing_subscriber::fmt()
        .with_timer(fmt::time::LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
