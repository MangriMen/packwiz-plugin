use std::path::Path;

use extism_pdk::FnResult;

use crate::{
    config::host,
    helpers::{log, LogLevel},
    models::{PackwizSettings, SerializableCommand},
};

pub fn update_pack_base(instance_id: &str, settings: &PackwizSettings) -> crate::Result<()> {
    log(
        LogLevel::Info,
        format!("Updating pack in instance {}", &instance_id),
    );

    let command = crate::helpers::get_command_to_update_pack(instance_id, settings)?;

    let command = SerializableCommand::from_command(&command);

    unsafe { host::run_command(command.clone()) }
        .map_err(|_| format!("Failed to run command {:?}", &command))?;

    log(
        LogLevel::Info,
        format!("Pack in instance {} updated successfully!", &instance_id),
    );

    Ok(())
}

pub fn update(instance_id: &str) -> FnResult<()> {
    crate::helpers::preload_resources(instance_id)?;

    let instance_plugin_folder = unsafe { host::instance_plugin_get_dir(instance_id.to_string()) }?
        .to_result()
        .map_err(|_| crate::Error("Failed to get instance directory".to_owned()))?;

    let instance_packwiz_settings = crate::api::settings::get_from_path(
        &Path::new(&instance_plugin_folder).join("packwiz.toml"),
    )?;

    update_pack_base(instance_id, &instance_packwiz_settings)?;

    Ok(())
}
