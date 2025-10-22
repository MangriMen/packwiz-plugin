use extism_pdk::FnResult;

use crate::{
    config::host,
    helpers::{log, packwiz::get_pack_from_path_or_url, LogLevel},
    models::{
        LoaderVersionPreference, ModLoader, NewInstance, PackInfo, PackVersions, PackwizPack,
        PackwizSettings, PluginImportInstance,
    },
};

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
        host::instance_create(NewInstance {
            name: pack.name.to_owned(),
            game_version: pack.versions.minecraft.to_owned(),
            mod_loader,
            loader_version: mod_loader_version.map(LoaderVersionPreference::Exact),
            icon_path: None,
            skip_install_instance: None,
            pack_info: Some(PackInfo {
                pack_type: "packwiz".to_owned(),
                pack_version: pack.version.clone(),
                can_update: true,
            }),
        })
    }
    .map_err(|_| "Failed to create instance")?;

    crate::api::settings::save_to_instance(
        &instance_id,
        &PackwizSettings {
            pack_path: pack_path.to_string(),
            update_on_launch: true,
        },
    )?;

    Ok(instance_id)
}

pub fn import(import_instance: &PluginImportInstance) -> FnResult<()> {
    match import_instance.importer_id.as_ref() {
        "packwiz" => {
            let path = &import_instance.path;

            let pack = get_pack_from_path_or_url(path)?;

            log(LogLevel::Debug, format!("{:?}", pack));

            let instance_id = create_instance_from_pack(&pack, path)?;

            crate::api::update(&instance_id)?;
        }
        _ => {
            return Err(crate::Error(format!(
                "Unsupported importer: {}",
                import_instance.importer_id
            ))
            .into())
        }
    }

    Ok(())
}
