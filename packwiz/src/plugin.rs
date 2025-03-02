use extism_pdk::{plugin_fn, FnResult};

use crate::{
    helpers::{log, LogLevel},
    models::ImportConfig,
};

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
pub fn get_import_config() -> FnResult<ImportConfig> {
    Ok(crate::api::get_import_config())
}

#[plugin_fn]
pub fn import(path: String) -> FnResult<()> {
    crate::api::import(&path)
}
