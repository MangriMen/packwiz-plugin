use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use extism_pdk::HttpRequest;

use crate::config::{
    host, PACKWIZ_INSTALLER_FILE_NAME, REDISTRIBUTABLE_FILES, REDISTRIBUTABLE_LINKS,
};

use super::{log, LogLevel};

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
    for (key, path) in REDISTRIBUTABLE_FILES.into_iter() {
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

        if let Some(url) = REDISTRIBUTABLE_LINKS.get(key) {
            if let Err(e) = download_redistributable(url, &path_to_redistributable, key) {
                log(LogLevel::Debug, e.0);
                continue;
            }
        } else {
            log(LogLevel::Debug, format!("Url for {} not found", key));
            continue;
        }
    }

    Ok(())
}

pub fn ensure_resource_in_instance_directory(
    instance_folder: &Path,
    resource_name: &str,
) -> crate::Result<()> {
    let file = instance_folder.join(resource_name);

    if !file.exists() {
        std::fs::copy(Path::new("/cache").join(resource_name), &file)
            .map_err(|_| "Failed to copy packwiz installer")?;
    }

    Ok(())
}

pub fn preload_resources(instance_id: &str) -> crate::Result<()> {
    ensure_download_all_redistributable()?;

    let instance_plugin_folder = unsafe { host::instance_get_dir(instance_id.to_string()) }?
        .to_result()
        .map_err(|_| "Failed to get instance directory".to_owned())?;
    let instance_folder = Path::new(&instance_plugin_folder);

    ensure_resource_in_instance_directory(instance_folder, PACKWIZ_INSTALLER_FILE_NAME)?;

    Ok(())
}
