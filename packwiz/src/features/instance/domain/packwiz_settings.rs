use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackwizSettings {
    pub pack_path: String,
    pub update_on_launch: bool,
}

impl PackwizSettings {
    pub fn load_from_path(path: &Path) -> crate::Result<PackwizSettings> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read pack.toml. {}", e))?;
        Ok(toml::from_str::<PackwizSettings>(&data)
            .map_err(|e| format!("Failed to parse pack as TOML. {}", e))?)
    }

    /// Serialize for `write_instance_file` (T-1.5.A.1).
    ///
    /// Writing is deliberately not done here: this plugin's per-instance directory sits inside
    /// the instance (`.metadata/plugins/<plugin_id>/`), so it belongs to the host — see F-30,
    /// and T-1.5.B mounts the instances directory read-only.
    pub fn to_toml(&self) -> crate::Result<String> {
        Ok(toml::to_string(self).map_err(|_| "Failed to serialize pack as TOML")?)
    }
}
