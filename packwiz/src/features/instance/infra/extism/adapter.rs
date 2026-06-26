use extism_pdk::{FnResult, plugin_fn};

use crate::features::instance::PluginImportInstance;

#[plugin_fn]
pub fn import(import_instance: PluginImportInstance) -> FnResult<()> {
    Ok(crate::api::import(import_instance)?)
}

#[plugin_fn]
pub fn update(instance_id: String) -> FnResult<()> {
    Ok(crate::api::update(instance_id)?)
}
