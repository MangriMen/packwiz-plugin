use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use aether_core_plugin_api::v0::WriteInstanceFileParamsDto;
use extism_pdk::HttpRequest;

use crate::features::{
    host::{LogLevel, log},
    instance,
};

use super::constants::*;

pub fn download_redistributable(url: &str, path: &Path, name: &str) -> crate::Result<()> {
    let res = extism_pdk::http::request::<()>(
        &HttpRequest {
            url: url.to_string(),
            headers: BTreeMap::default(),
            method: Some("GET".to_string()),
        },
        None,
    )
    .map_err(|_| format!("Failed to download redistributable {}", name))?;

    Ok(std::fs::write(path, res.body())
        .map_err(|_| format!("Failed to save redistributable {} to {:?}", name, &path))?)
}

pub fn ensure_download_all_redistributable() -> crate::Result<()> {
    for (key, path) in REDISTRIBUTABLE_FILES.iter() {
        let path_to_redistributable = PathBuf::from("/cache").join(path);

        if path_to_redistributable.exists() {
            log(
                LogLevel::Debug,
                format!(
                    "Skip redistributable {} -> {:?}",
                    key, path_to_redistributable
                ),
            );
            continue;
        }

        log(
            LogLevel::Debug,
            format!(
                "Downloading redistributable {} -> {:?}",
                key, path_to_redistributable
            ),
        );

        if let Some(url) = get_redistributable_link(key) {
            if let Err(e) = download_redistributable(url, &path_to_redistributable, key) {
                log(LogLevel::Debug, e.to_string());
                continue;
            }
        } else {
            log(LogLevel::Debug, format!("Url for {} not found", key));
            continue;
        }
    }

    Ok(())
}

/// Put a cached redistributable into the instance directory.
///
/// The bytes go through `write_instance_file` (T-1.3), not `std::fs`: the instance directory
/// belongs to the host, which resolves `instance_id` + relative path itself and enforces the
/// path rules and the 64 MiB cap (F-25). `packwiz-installer.jar` is well under that cap;
/// anything large would have to go through `download_to_cache` (T-1.2) instead.
///
/// The "already there" check stays an ordinary read of the mounted instance directory — only
/// writes move to the host function (T-1.5.B makes that mount `ro:`, reads keep working).
///
/// Note (F-24): the host does not register this file in `PackStorage`, exactly as the previous
/// `std::fs::copy` did not. `packwiz-installer.jar` sits in the instance root, outside the
/// content folders the instance watcher tracks, so it stays invisible to the pack — which is
/// what we want for a tool jar.
pub fn ensure_resource_in_instance_directory(
    instance_id: &str,
    resource_name: &str,
) -> crate::Result<()> {
    let instance_folder = instance::instance_get_dir(instance_id.to_string()).into_result()?;

    if Path::new(&instance_folder).join(resource_name).exists() {
        return Ok(());
    }

    let bytes = std::fs::read(Path::new("/cache").join(resource_name))
        .map_err(|_| format!("Failed to read redistributable {} from cache", resource_name))?;

    instance::write_instance_file(WriteInstanceFileParamsDto {
        instance_id: instance_id.to_string(),
        relative_path: resource_name.to_string(),
        bytes,
    })
    .into_result()?;

    Ok(())
}

pub fn preload_resources(instance_id: &str) -> crate::Result<()> {
    ensure_download_all_redistributable()?;

    ensure_resource_in_instance_directory(instance_id, PACKWIZ_INSTALLER_FILE_NAME)?;

    Ok(())
}
