use crate::features::events::PluginEvent;

pub fn handle_plugin_event(event: &PluginEvent) -> crate::Result<()> {
    match event {
        PluginEvent::BeforeInstanceLaunch { instance_id } => {
            let plugin_settings = crate::api::settings::get_from_instance(instance_id)?;

            if plugin_settings.update_on_launch {
                return crate::api::update(instance_id.to_string());
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
