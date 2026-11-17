use aether_core_plugin_pdk::api::v0::PluginImportInstanceDto;

pub fn import(import_instance: PluginImportInstanceDto) -> crate::Result<()> {
    crate::features::instance::import(import_instance)
}
