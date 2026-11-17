use aether_core_plugin_pdk::api::v0::PluginInternalEventDto;

pub fn handle_plugin_event(event: &PluginInternalEventDto) -> crate::Result<()> {
    crate::features::events::handle_plugin_event(event)
}
