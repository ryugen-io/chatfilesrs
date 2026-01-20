//! Logging module with hyprlog support and standard fallback.

#[cfg(feature = "hyprlog")]
use std::sync::OnceLock;

#[cfg(feature = "hyprlog")]
use hl_core::Logger;

#[cfg(feature = "hyprlog")]
static LOGGER: OnceLock<Logger> = OnceLock::new();

#[cfg(feature = "hyprlog")]
pub fn init() {
    LOGGER.get_or_init(|| Logger::from_config("chatfiles"));
}

#[cfg(not(feature = "hyprlog"))]
pub fn init() {}

#[cfg(feature = "hyprlog")]
fn get() -> &'static Logger {
    LOGGER
        .get()
        .expect("Logger not initialized - call log::init() first")
}

pub fn debug(scope: &str, msg: &str) {
    #[cfg(feature = "hyprlog")]
    get().debug(scope, msg);

    #[cfg(not(feature = "hyprlog"))]
    eprintln!("[DEBUG] [{scope}] {msg}");
}

pub fn info(scope: &str, msg: &str) {
    #[cfg(feature = "hyprlog")]
    get().info(scope, msg);

    #[cfg(not(feature = "hyprlog"))]
    eprintln!("[INFO] [{scope}] {msg}");
}

pub fn warn(scope: &str, msg: &str) {
    #[cfg(feature = "hyprlog")]
    get().warn(scope, msg);

    #[cfg(not(feature = "hyprlog"))]
    eprintln!("[WARN] [{scope}] {msg}");
}

pub fn error(scope: &str, msg: &str) {
    #[cfg(feature = "hyprlog")]
    get().error(scope, msg);

    #[cfg(not(feature = "hyprlog"))]
    eprintln!("[ERROR] [{scope}] {msg}");
}
