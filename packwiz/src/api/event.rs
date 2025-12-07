use crate::features::events::PluginEvent;

pub fn handle_plugin_event(event: &PluginEvent) -> crate::Result<()> {
    crate::features::events::handle_plugin_event(event)
}
