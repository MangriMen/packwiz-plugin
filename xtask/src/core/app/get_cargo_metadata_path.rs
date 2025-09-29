use std::path::{Path, PathBuf};

pub fn get_cargo_metadata_path(plugin_name: &str) -> PathBuf {
    Path::new(plugin_name).join("Cargo.toml")
}
