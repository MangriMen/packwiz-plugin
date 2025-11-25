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

    log(
        LogLevel::Debug,
        format!(
            "Creating instance with mod_loader: {:?} version: {:?}",
            mod_loader, mod_loader_version
        ),
    );

    let new_instance = NewInstance {
        name: pack.name.to_owned(),
        game_version: pack.versions.minecraft.to_owned(),
        mod_loader,
        loader_version: mod_loader_version.map(LoaderVersionPreference::Exact),
        icon_path: None,
        skip_install_instance: None,
        pack_info: Some(PackInfo {
            modpack_id: "packwiz".to_owned(),
            version: pack.version.clone(),
        }),
    };

    log(LogLevel::Trace, format!("New instance: {:?}", new_instance));

    let instance_id = unsafe { host::instance_create(new_instance) }?.to_result();

    match instance_id {
        Ok(instance_id) => {
            crate::api::settings::save_to_instance(
                &instance_id,
                &PackwizSettings {
                    pack_path: pack_path.to_string(),
                    update_on_launch: true,
                },
            )?;

            Ok(instance_id)
        }
        Err(e) => {
            let error_msg = e.message;
            log(LogLevel::Error, error_msg.clone());
            Err(crate::Error(error_msg))
        }
    }
}

pub fn import(import_instance: &PluginImportInstance) -> FnResult<()> {
    match import_instance.importer_id.as_ref() {
        "packwiz" => {
            let path = &import_instance.path;

            let pack = get_pack_from_path_or_url(path)?;
            log(LogLevel::Debug, format!("Pack loaded: {}", pack.name));
            log(LogLevel::Trace, format!("Pack: {:?}", pack));

            let instance_id = create_instance_from_pack(&pack, path)?;
            log(
                LogLevel::Debug,
                format!("Instance created: {}", instance_id),
            );

            crate::api::update(&instance_id)?;
            log(LogLevel::Debug, "Import completed successfully".to_owned());
        }
        _ => {
            let error_msg = format!("Unsupported importer: {}", import_instance.importer_id);
            log(LogLevel::Error, error_msg.clone());
            return Err(crate::Error(error_msg).into());
        }
    }

    Ok(())
}
