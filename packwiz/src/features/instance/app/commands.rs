use std::collections::HashMap;

use aether_core_plugin_pdk::{
    self as pdk,
    api::v0::{
        ChangeContentsParamsDto, ContentFileDto, NewInstanceDto, WriteInstanceFileParamsDto,
    },
};

/// The sandbox path of the instance, `/instances/<id>`.
pub fn instance_get_dir(instance_id: &str) -> crate::Result<String> {
    Ok(pdk::host::instance_get_dir(instance_id)?.path)
}

/// The sandbox path of this plugin's folder in the instance,
/// `/instances/<id>/.metadata/plugins/<plugin id>`.
pub fn instance_plugin_get_dir(instance_id: &str) -> crate::Result<String> {
    Ok(pdk::host::instance_plugin_get_dir(instance_id)?.path)
}

/// Create an instance and return its id.
pub fn instance_create(new_instance: NewInstanceDto) -> crate::Result<String> {
    Ok(pdk::host::instance_create(new_instance)?.instance_id)
}

/// The content files of the instance, by their path in it.
pub fn list_content(instance_id: &str) -> crate::Result<HashMap<String, ContentFileDto>> {
    Ok(pdk::host::list_content(instance_id)?.files)
}

pub fn enable_contents(instance_id: &str, paths: Vec<String>) -> crate::Result<()> {
    pdk::host::enable_contents(ChangeContentsParamsDto {
        instance_id: instance_id.to_owned(),
        paths,
    })
}

pub fn disable_contents(instance_id: &str, paths: Vec<String>) -> crate::Result<()> {
    pdk::host::disable_contents(ChangeContentsParamsDto {
        instance_id: instance_id.to_owned(),
        paths,
    })
}

/// Write a file into an instance directory through the host (T-1.3).
///
/// `params.relative_path` is relative to the instance root and `/`-separated; the plugin never
/// touches the instance directory through `std::fs` itself.
pub fn write_instance_file(params: WriteInstanceFileParamsDto) -> crate::Result<()> {
    pdk::host::write_instance_file(params)
}
