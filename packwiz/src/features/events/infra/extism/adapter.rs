use extism_pdk::{FnResult, plugin_fn};

use crate::features::events::PluginEvent;

#[plugin_fn]
pub fn handle_event(event: PluginEvent) -> FnResult<()> {
    Ok(crate::api::handle_plugin_event(&event)?)
}
