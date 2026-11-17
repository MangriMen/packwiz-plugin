use aether_core_plugin_pdk::{Wire, api::v0::PluginInternalEventDto};
use extism_pdk::{FnResult, plugin_fn};

#[plugin_fn]
pub fn handle_event(Wire(event): Wire<PluginInternalEventDto>) -> FnResult<()> {
    Ok(crate::api::handle_plugin_event(&event)?)
}
