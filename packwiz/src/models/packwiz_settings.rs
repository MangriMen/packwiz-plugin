use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
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

    pub fn save_to_path(&self, path: &Path) -> crate::Result<()> {
        let data = toml::to_string(self).map_err(|_| "Failed to serialize pack as TOML")?;
        let parent_dir = path.parent().ok_or(crate::Error(format!(
            "Failed to get parent directory {:?}",
            path
        )))?;

        std::fs::create_dir_all(parent_dir)
            .map_err(|e| format!("Failed to create directory. {}", e))?;
        std::fs::write(path, data).map_err(|e| format!("Failed to write packwiz.toml. {}", e))?;
        Ok(())
    }
}
