use std::sync::LazyLock;
use tracing_subscriber::{fmt, EnvFilter};

static INIT_TRACING_ONCE: LazyLock<()> = LazyLock::new(init_tracing);

fn init_tracing() {
    // Initialize logger with local time formatting and env-based filtering
    tracing_subscriber::fmt()
        .with_timer(fmt::time::LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub fn init_tracing_once() {
    // &INIT_TRACING_ONCE does not take effect
    // * After dereferencing, the initialization logic of LazyLock will be triggered, and the init_tracing function will be called.
    *INIT_TRACING_ONCE;
}
