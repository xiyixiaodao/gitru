mod tracing_util;
mod windows_color_util;
pub use tracing_util::init_tracing_once;
#[cfg(windows)]
pub use windows_color_util::enable_old_windows_color_support;
