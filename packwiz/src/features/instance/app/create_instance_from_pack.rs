use aether_core_plugin_api::v0::{
    LoaderVersionPreferenceDto, NewInstanceDto, PackInfoDto, ProviderIdDto,
};

use crate::features::{
    host::{self, log, LogLevel},
    instance::{extract_mod_loader, instance_create, PackwizSettings},
    packwiz::PackwizPack,
};

pub fn create_instance_from_pack(pack: &PackwizPack, pack_path: &str) -> crate::Result<String> {
    let plugin_id = host::get_id()?;

    let (mod_loader, mod_loader_version) = extract_mod_loader(&pack.versions)?;

    log(
        LogLevel::Debug,
        format!(
            "Creating instance with mod_loader: {:?} version: {:?}",
            mod_loader, mod_loader_version
        ),
    );

    let new_instance = NewInstanceDto {
        name: pack.name.to_owned(),
        game_version: pack.versions.minecraft.to_owned(),
        mod_loader,
        loader_version: mod_loader_version.map(LoaderVersionPreferenceDto::Exact),
        icon_path: None,
        skip_install_instance: None,
        pack_info: Some(PackInfoDto {
            provider_id: ProviderIdDto {
                plugin_id,
                capability_id: "packwiz".to_owned(),
            },
            modpack_id: "packwiz".to_owned(),
            version_id: pack.version.clone(),
        }),
    };

    log(LogLevel::Trace, format!("New instance: {:?}", new_instance));

    let instance_id = instance_create(new_instance)
        .into_result()
        .inspect_err(|e| {
            log(LogLevel::Error, format!("{e:?}"));
        })?;

    let pack_settings = PackwizSettings {
        pack_path: pack_path.to_string(),
        update_on_launch: true,
    };
    crate::api::settings::save_to_instance(&instance_id, &pack_settings)?;
    log(
        LogLevel::Debug,
        format!("Pack settings saved: {:?}", pack_settings),
    );

    Ok(instance_id)
}
