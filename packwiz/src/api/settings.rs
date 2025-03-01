use std::path::Path;

use crate::{config::host, models::PackwizSettings};

pub fn get_from_instance(instance_id: &str) -> crate::Result<PackwizSettings> {
    let instance_dir = unsafe { host::instance_plugin_get_dir(instance_id.to_string()) }
        .map_err(|_| "Failed to get instance directory")?;

    PackwizSettings::load_from_path(&Path::new(&instance_dir).join("packwiz.toml"))
}

pub fn get_from_path(path: &Path) -> crate::Result<PackwizSettings> {
    PackwizSettings::load_from_path(path)
}

pub fn save_to_instance(instance_id: &str, settings: &PackwizSettings) -> crate::Result<()> {
    let instance_dir = unsafe { host::instance_plugin_get_dir(instance_id.to_string()) }
        .map_err(|_| "Failed to get instance directory")?;

    settings.save_to_path(&Path::new(&instance_dir).join("packwiz.toml"))
}
