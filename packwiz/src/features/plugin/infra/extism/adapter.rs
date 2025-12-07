use extism_pdk::{plugin_fn, FnResult};

use crate::features::host::{log, LogLevel};

#[plugin_fn]
pub fn on_load() -> FnResult<()> {
    log(LogLevel::Debug, "Load packwiz plugin".to_string());
    Ok(())
}

#[plugin_fn]
pub fn on_unload() -> FnResult<()> {
    log(LogLevel::Debug, "Unload packwiz plugin".to_string());
    Ok(())
}
