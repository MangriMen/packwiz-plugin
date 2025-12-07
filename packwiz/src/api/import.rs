use crate::features::instance::PluginImportInstance;

pub fn import(import_instance: PluginImportInstance) -> crate::Result<()> {
    crate::features::instance::import(import_instance)
}
