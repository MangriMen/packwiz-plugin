use aether_core_plugin_pdk::api::v0::PluginInternalEventDto;

pub fn handle_plugin_event(event: &PluginInternalEventDto) -> crate::Result<()> {
    match event {
        PluginInternalEventDto::BeforeInstanceLaunch { instance_id } => {
            let plugin_settings = crate::api::settings::get_from_instance(instance_id)?;

            if plugin_settings.update_on_launch {
                return crate::api::update(instance_id.to_string());
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
