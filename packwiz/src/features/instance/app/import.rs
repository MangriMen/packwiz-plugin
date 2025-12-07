use crate::features::{
    host::{log, LogLevel},
    instance::PluginImportInstance,
    packwiz::{self},
};

use super::create_instance_from_pack;

pub fn import(import_instance: PluginImportInstance) -> crate::Result<()> {
    match import_instance.importer_id.as_ref() {
        "packwiz" => import_by_packwiz_pack(import_instance)?,
        _ => {
            let error_msg = format!("Unsupported importer: {}", import_instance.importer_id);
            log(LogLevel::Error, error_msg.clone());
            return Err(crate::Error(error_msg));
        }
    }

    Ok(())
}

fn import_by_packwiz_pack(import_instance: PluginImportInstance) -> crate::Result<()> {
    let path = &import_instance.path;

    let pack = packwiz::get_pack_from_path_or_url(path)?;
    log(LogLevel::Debug, format!("Pack loaded: {}", pack.name));
    log(LogLevel::Trace, format!("Pack: {:?}", pack));

    let instance_id = create_instance_from_pack(&pack, path)?;

    log(
        LogLevel::Debug,
        format!("Instance created: {}", instance_id),
    );

    crate::api::update(instance_id)?;
    log(LogLevel::Debug, "Import completed successfully".to_owned());

    Ok(())
}
