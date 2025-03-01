use extism_pdk::FnResult;

use crate::{
    config::host,
    helpers::{log, packwiz::get_pack_from_path_or_url, LogLevel},
    models::{PackwizPack, PackwizSettings},
};

fn create_instance_from_pack(pack: &PackwizPack, pack_path: &str) -> crate::Result<String> {
    // let (mod_loader, mod_loader_version) = PackwizPlugin::extract_mod_loader(&pack.versions)
    //     .map_err(|e| crate::ErrorKind::PluginError(self.get_id(), e.to_string()))?;

    let instance_id = unsafe {
        host::instance_create(
            pack.name.to_string(),
            pack.versions.minecraft.to_string(),
            "vanilla".to_owned(),
            None,
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

pub fn import_pack(path_or_url: &str) -> FnResult<()> {
    let pack = get_pack_from_path_or_url(path_or_url)?;

    log(LogLevel::Debug, format!("{:?}", pack));

    let instance_id = create_instance_from_pack(&pack, path_or_url)?;

    crate::api::update_pack(&instance_id)?;

    Ok(())
}
