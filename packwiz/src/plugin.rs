use extism_pdk::{plugin_fn, FnResult};

use crate::helpers::{log, LogLevel};

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

#[plugin_fn]
pub fn import(path: String) -> FnResult<()> {
    crate::api::import_pack(&path)
}
