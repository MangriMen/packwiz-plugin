use aether_core_plugin_pdk::{Wire, api::v0::LogParamsDto};

use crate::features::host::host;

pub use aether_core_plugin_pdk::api::v0::LogLevelDto as LogLevel;

/// Log through the host. A failed log is dropped: there is nowhere else to report it.
pub fn log(level: LogLevel, message: String) {
    let _ = unsafe { host::log(Wire(LogParamsDto { level, message })) };
}
