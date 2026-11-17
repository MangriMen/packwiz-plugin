use std::path::{Path, PathBuf};

use aether_core_plugin_pdk::api::v0::WriteInstanceFileParamsDto;

use crate::{
    ResultExt,
    features::{
        host::{LogLevel, download_to_cache_sha256, log},
        instance,
    },
};

use super::constants::*;

/// Where the jars the plugin runs are, in the plugin's `/cache`.
pub struct Redistributables {
    pub installer: PathBuf,
    pub bootstrap: PathBuf,
}

/// Have the host fetch the pinned jar into the cache (a no-op when it is already there) and
/// return its path.
fn ensure_redistributable(redistributable: &Redistributable) -> crate::Result<PathBuf> {
    log(
        LogLevel::Debug,
        format!(
            "Ensure redistributable {} ({})",
            redistributable.name, redistributable.url
        ),
    );

    download_to_cache_sha256(redistributable.url, redistributable.sha256).context(format!(
        "Failed to download redistributable {}",
        redistributable.name
    ))
}

pub fn ensure_download_all_redistributable() -> crate::Result<Redistributables> {
    Ok(Redistributables {
        installer: ensure_redistributable(&PACKWIZ_INSTALLER)?,
        bootstrap: ensure_redistributable(&PACKWIZ_INSTALLER_BOOTSTRAP)?,
    })
}

/// Put a cached redistributable into the instance directory under `resource_name`.
///
/// The bytes go through `write_instance_file` (T-1.3), not `std::fs`: the instance directory
/// belongs to the host, which resolves `instance_id` + relative path itself and enforces the
/// path rules and the 64 MiB cap (F-25). The jar is read from the cache with ordinary WASI
/// reads and handed over as a msgpack `bin`, so its size costs no per-byte work in the plugin.
///
/// The jar of another release replaces the one in the instance: the cache name is the digest, so
/// a differing size or content means the pinned release changed.
///
/// Note (F-24): the host does not register this file in `PackStorage`, exactly as the previous
/// `std::fs::copy` did not. `packwiz-installer.jar` sits in the instance root, outside the
/// content folders the instance watcher tracks, so it stays invisible to the pack — which is
/// what we want for a tool jar.
pub fn ensure_resource_in_instance_directory(
    instance_id: &str,
    resource_name: &str,
    cached: &Path,
) -> crate::Result<()> {
    let instance_folder = instance::instance_get_dir(instance_id)?;

    let bytes = std::fs::read(cached).map_err(|_| {
        format!(
            "Failed to read redistributable {} from cache",
            resource_name
        )
    })?;

    let in_instance = Path::new(&instance_folder).join(resource_name);
    if std::fs::read(&in_instance).is_ok_and(|existing| existing == bytes) {
        return Ok(());
    }

    instance::write_instance_file(WriteInstanceFileParamsDto {
        instance_id: instance_id.to_string(),
        relative_path: resource_name.to_string(),
        bytes,
    })
}

pub fn preload_resources(instance_id: &str) -> crate::Result<Redistributables> {
    let redistributables = ensure_download_all_redistributable()?;

    ensure_resource_in_instance_directory(
        instance_id,
        PACKWIZ_INSTALLER_FILE_NAME,
        &redistributables.installer,
    )?;

    Ok(redistributables)
}
