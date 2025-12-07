use std::path::Path;

use crate::features::instance::{self, PackwizSettings};

pub fn get_from_instance(instance_id: &str) -> crate::Result<PackwizSettings> {
    let instance_dir = instance::instance_plugin_get_dir(instance_id.to_string())?;
    PackwizSettings::load_from_path(&Path::new(&instance_dir).join("packwiz.toml"))
}

pub fn get_from_path(path: &Path) -> crate::Result<PackwizSettings> {
    PackwizSettings::load_from_path(path)
}

pub fn save_to_instance(instance_id: &str, settings: &PackwizSettings) -> crate::Result<()> {
    let instance_dir = instance::instance_plugin_get_dir(instance_id.to_string())?;
    settings.save_to_path(&Path::new(&instance_dir).join("packwiz.toml"))
}
