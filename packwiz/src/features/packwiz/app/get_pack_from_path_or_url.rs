use std::path::PathBuf;

use extism_pdk::{HttpRequest, http};
use path_slash::PathBufExt;

use crate::features::packwiz::PackwizPack;

pub fn get_pack_from_path_or_url(path_or_url: &str) -> crate::Result<PackwizPack> {
    // Heuristic: if the string contains "://" it's a URL, otherwise it's a local path.
    // We cannot use Url::parse() because Windows drive letters (e.g. "D:\")
    // are incorrectly parsed as URL schemes inside WASM.
    let data: Vec<u8> = if path_or_url.contains("://") {
        let response = http::request::<()>(&HttpRequest::new(path_or_url).with_method("GET"), None)
            .map_err(|_| format!("Failed to download pack from {path_or_url}"))?;

        response.body().to_vec()
    } else {
        let path = PathBuf::from(path_or_url).to_slash_lossy().to_string();

        std::fs::read(&path).map_err(|_| format!("Failed to read pack from {path_or_url}"))?
    };

    let string_data = std::str::from_utf8(&data).map_err(|_| "Failed to parse pack as UTF-8")?;
    let pack = toml::from_str(string_data).map_err(|_| "Failed to parse pack as TOML")?;

    Ok(pack)
}
