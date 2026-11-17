use std::path::PathBuf;

use aether_core_plugin_pdk::api::v0::ImportSourceDto;
use path_slash::PathBufExt;

use crate::{
    ResultExt,
    features::{host::http_get_bytes, packwiz::PackwizPack},
};

/// Read the `pack.toml` the importer was given: download it through the host, or read it from a
/// folder the user granted the plugin.
pub fn get_pack(source: &ImportSourceDto) -> crate::Result<PackwizPack> {
    let data: Vec<u8> = match source {
        ImportSourceDto::Url { url } => {
            http_get_bytes(url).context(format!("Failed to download pack from {url}"))?
        }
        ImportSourceDto::File { path } => {
            let path = PathBuf::from(path).to_slash_lossy().to_string();

            std::fs::read(&path).map_err(|e| format!("Failed to read pack from {path}: {e}"))?
        }
    };

    let string_data = std::str::from_utf8(&data).map_err(|_| "Failed to parse pack as UTF-8")?;
    let pack = toml::from_str(string_data).map_err(|_| "Failed to parse pack as TOML")?;

    Ok(pack)
}
