use std::{path::Path, process::Command};

use crate::features::{
    host::{LogLevel, log},
    instance::{self, PackwizSettings},
    java,
};

use crate::features::host::HostResult;

pub fn get_command_to_update_pack(
    instance_id: &str,
    settings: &PackwizSettings,
    bootstrap: &Path,
) -> crate::Result<Command> {
    const JAVA_VERSION: u32 = 8;
    log(LogLevel::Debug, "Try to get java path".to_string());

    let java = match java::get_java(JAVA_VERSION) {
        HostResult::Ok(java) => java,
        HostResult::Err(_) => java::install_java(JAVA_VERSION).into_result()?,
    };

    log(LogLevel::Debug, format!("Java path: {}", &java.path));

    let instance_folder = instance::instance_get_dir(instance_id.to_string()).into_result()?;
    let instance_folder = Path::new(&instance_folder);

    let mut cmd = Command::new(&java.path);
    cmd.current_dir(format!("#{}", instance_folder.to_string_lossy()))
        .arg("-jar")
        .arg(format!("#{}", bootstrap.to_string_lossy()))
        .arg("--bootstrap-no-update")
        .arg(settings.pack_path.clone());
    Ok(cmd)
}
