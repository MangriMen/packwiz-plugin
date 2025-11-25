use std::{path::Path, process::Command};

use crate::{
    config::{host, PACKWIZ_INSTALLER_BOOTSTRAP_FILE_NAME, PACKWIZ_INSTALLER_FILE_NAME},
    helpers::{log, LogLevel},
    models::PackwizSettings,
};

pub fn get_command_to_update_pack(
    instance_id: &str,
    settings: &PackwizSettings,
) -> crate::Result<Command> {
    const JAVA_VERSION: u32 = 8;
    log(LogLevel::Debug, "Try to get java path".to_string());

    let java = match unsafe { host::get_java(JAVA_VERSION) }?.to_result() {
        Ok(java) => Ok(java),
        Err(_) => unsafe { host::install_java(JAVA_VERSION) }?.to_result(),
    }?;

    log(LogLevel::Debug, format!("Java path: {}", &java.path));

    let cache_dir = Path::new("/cache");

    let packwiz_installer_paths = (
        cache_dir.join(PACKWIZ_INSTALLER_FILE_NAME),
        cache_dir.join(PACKWIZ_INSTALLER_BOOTSTRAP_FILE_NAME),
    );

    let instance_folder = unsafe { host::instance_get_dir(instance_id.to_string()) }?
        .to_result()
        .map_err(|_| "Failed to get instance directory")?;
    let instance_folder = Path::new(&instance_folder);

    let _packwiz_installer_instance_path = instance_folder.join(PACKWIZ_INSTALLER_FILE_NAME);

    // if !packwiz_installer_instance_path.exists() {
    //     // TODO: need admin rights
    //     // symlink_file(
    //     //     &packwiz_installer_paths.1,
    //     //     &packwiz_installer_paths.0,
    //     // )
    //     // .await?;
    //     hard_link(&packwiz_installer_paths.0, &packwiz_installer_instance_path).await?;
    // }

    let mut cmd = Command::new(&java.path);
    cmd.current_dir(format!("#{}", instance_folder.to_string_lossy()))
        .arg("-jar")
        .arg(format!(
            "#{}",
            &packwiz_installer_paths.1.to_string_lossy().to_string()
        ))
        .arg("--bootstrap-no-update")
        .arg(settings.pack_path.clone());
    Ok(cmd)
}
