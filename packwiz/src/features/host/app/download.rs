use std::path::PathBuf;

use aether_core_plugin_pdk::{
    Wire,
    api::v0::{DownloadToCacheParamsDto, FileHashDto},
};

use crate::features::host::host;

/// Have the host download `url` into the plugin's `/cache` and return the path of the file.
///
/// The host checks the file against `sha256` before it makes it visible, and the body never
/// passes through the plugin's memory, so this is the way to fetch anything of size.
pub fn download_to_cache_sha256(url: &str, sha256: &str) -> crate::Result<PathBuf> {
    let cached = unsafe {
        host::download_to_cache(Wire(DownloadToCacheParamsDto {
            url: url.to_string(),
            hash: FileHashDto::sha256(sha256),
        }))
    }?
    .into_result()?;

    Ok(PathBuf::from(cached.path))
}
