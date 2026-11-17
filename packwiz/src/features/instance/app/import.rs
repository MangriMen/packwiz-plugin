use aether_core_plugin_pdk::api::v0::{ImportSourceDto, PluginImportInstanceDto};

use crate::features::{
    host::{LogLevel, log},
    packwiz,
};

use super::create_instance_from_pack;

pub fn import(import_instance: PluginImportInstanceDto) -> crate::Result<()> {
    match import_instance.importer_id.as_ref() {
        "packwiz" => import_by_packwiz_pack(&import_instance.source)?,
        _ => {
            let error_msg = format!("Unsupported importer: {}", import_instance.importer_id);
            log(LogLevel::Error, error_msg.clone());
            return Err(error_msg.into());
        }
    }

    Ok(())
}

fn import_by_packwiz_pack(source: &ImportSourceDto) -> crate::Result<()> {
    // What `packwiz-installer` is given on every update: the URL as it is, or the plugin-side
    // path, which the host turns back into the host path (`/mnt/d/…` → `D:\…`) when it runs it.
    let pack_path = match source {
        ImportSourceDto::Url { url } => url,
        ImportSourceDto::File { path } => path,
    };

    let pack = packwiz::get_pack(source)?;
    log(LogLevel::Debug, format!("Pack loaded: {}", pack.name));
    log(LogLevel::Trace, format!("Pack: {:?}", pack));

    let instance_id = create_instance_from_pack(&pack, pack_path)?;

    log(
        LogLevel::Debug,
        format!("Instance created: {}", instance_id),
    );

    crate::api::update(instance_id)?;
    log(LogLevel::Debug, "Import completed successfully".to_owned());

    Ok(())
}
