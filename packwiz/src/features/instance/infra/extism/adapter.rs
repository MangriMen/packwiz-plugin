use aether_core_plugin_pdk::{
    Wire,
    api::v0::{PluginImportInstanceDto, UpdateInstanceParamsDto},
};
use extism_pdk::{FnResult, plugin_fn};

#[plugin_fn]
pub fn import(Wire(import_instance): Wire<PluginImportInstanceDto>) -> FnResult<()> {
    Ok(crate::api::import(import_instance)?)
}

#[plugin_fn]
pub fn update(Wire(params): Wire<UpdateInstanceParamsDto>) -> FnResult<()> {
    Ok(crate::api::update(params.instance_id)?)
}
