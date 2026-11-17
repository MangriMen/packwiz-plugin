use std::path::Path;

use aether_core_plugin_pdk::api::v0::WriteInstanceFileParamsDto;

use crate::features::instance::{self, PackwizSettings};

/// Name of this plugin's settings file inside its per-instance directory.
pub const PACKWIZ_SETTINGS_FILE_NAME: &str = "packwiz.toml";

pub fn get_from_instance(instance_id: &str) -> crate::Result<PackwizSettings> {
    let instance_dir = instance::instance_plugin_get_dir(instance_id)?;
    PackwizSettings::load_from_path(&Path::new(&instance_dir).join(PACKWIZ_SETTINGS_FILE_NAME))
}

pub fn get_from_path(path: &Path) -> crate::Result<PackwizSettings> {
    PackwizSettings::load_from_path(path)
}

/// Save the settings through `write_instance_file` (T-1.5.A.1).
///
/// Reading still goes through the mounted instance directory, which stays readable under the
/// `ro:` mount of T-1.5.B; only writing has to go to the host, which also creates the parent
/// directories on the way.
pub fn save_to_instance(instance_id: &str, settings: &PackwizSettings) -> crate::Result<()> {
    let relative_path =
        instance::instance_plugin_relative_path(instance_id, PACKWIZ_SETTINGS_FILE_NAME)?;

    instance::write_instance_file(WriteInstanceFileParamsDto {
        instance_id: instance_id.to_owned(),
        relative_path,
        bytes: settings.to_toml()?.into_bytes(),
    })
}
