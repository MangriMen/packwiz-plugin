use extism_pdk::FnResult;

use crate::{
    config::host,
    helpers::{log, packwiz::get_pack_from_path_or_url, LogLevel},
    models::{ImportConfig, ModLoader, PackVersions, PackwizPack, PackwizSettings},
};

lazy_static::lazy_static! {
    static ref DEFAULT_IMPORT_CONFIG: ImportConfig = ImportConfig {
        pack_type: "packwiz".to_string(),
        title: "Packwiz".to_string(),
        field_label: "Packwiz pack URL or file".to_string(),
        file_name: "Packwiz modpack".to_string(),
        file_extensions: vec!["toml".to_string()],
    };
}

pub fn get_import_config() -> ImportConfig {
    DEFAULT_IMPORT_CONFIG.clone()
}

fn extract_mod_loader(version: &PackVersions) -> crate::Result<(ModLoader, Option<String>)> {
    match (
        version.fabric.as_ref(),
        version.forge.as_ref(),
        version.liteloader.as_ref(),
        version.quilt.as_ref(),
    ) {
        (Some(fabric), _, _, _) => Ok((ModLoader::Fabric, Some(fabric.clone()))),
        (_, Some(forge), _, _) => Ok((ModLoader::Forge, Some(forge.clone()))),
        (_, _, Some(liteloader), _) => Err(crate::Error(format!(
            "Unsupported mod loader: \"liteloader: {}\"",
            liteloader
        ))),
        (_, _, _, Some(quilt)) => Ok((ModLoader::Quilt, Some(quilt.clone()))),
        (_, _, _, _) => Ok((ModLoader::Vanilla, None)),
    }
}

fn create_instance_from_pack(pack: &PackwizPack, pack_path: &str) -> crate::Result<String> {
    let (mod_loader, mod_loader_version) = extract_mod_loader(&pack.versions)?;

    let instance_id = unsafe {
        host::instance_create(
            pack.name.to_string(),
            pack.versions.minecraft.to_string(),
            mod_loader.as_str().to_string(),
            mod_loader_version,
            None,
            Some(0),
        )
    }
    .map_err(|_| "Failed to create instance")?;

    crate::api::settings::save_to_instance(
        &instance_id,
        &PackwizSettings {
            pack_path: pack_path.to_string(),
        },
    )?;

    Ok(instance_id)
}

pub fn import(path_or_url: &str) -> FnResult<()> {
    let pack = get_pack_from_path_or_url(path_or_url)?;

    log(LogLevel::Debug, format!("{:?}", pack));

    let instance_id = create_instance_from_pack(&pack, path_or_url)?;

    crate::api::update_pack(&instance_id)?;

    Ok(())
}
