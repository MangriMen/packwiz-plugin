use crate::features::{
    host::{log, LogLevel},
    instance::{
        extract_mod_loader, instance_create, LoaderVersionPreference, NewInstance, PackInfo,
        PackwizSettings,
    },
    packwiz::PackwizPack,
};

pub fn create_instance_from_pack(pack: &PackwizPack, pack_path: &str) -> crate::Result<String> {
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

    let instance_id = instance_create(new_instance);

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
