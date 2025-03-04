use extism_pdk::FnResult;

use crate::models::PluginEvent;

pub fn handle_event(event: &PluginEvent) -> FnResult<()> {
    match event {
        PluginEvent::BeforeInstanceLaunch { instance_id } => {
            let plugin_settings = crate::api::settings::get_from_instance(instance_id)?;

            if plugin_settings.update_on_launch {
                return crate::api::update(instance_id);
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
