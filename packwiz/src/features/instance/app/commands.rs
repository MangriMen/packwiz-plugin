use std::collections::HashMap;

use aether_core_plugin_pdk::{
    Wire,
    api::v0::{
        ChangeContentsParamsDto, ContentFileDto, InstanceIdParamsDto, NewInstanceDto,
        WriteInstanceFileParamsDto,
    },
};

use crate::features::instance::infra::host;

fn instance_id_params(instance_id: &str) -> Wire<InstanceIdParamsDto> {
    Wire(InstanceIdParamsDto {
        instance_id: instance_id.to_owned(),
    })
}

/// The sandbox path of the instance, `/instances/<id>`.
pub fn instance_get_dir(instance_id: &str) -> crate::Result<String> {
    Ok(
        unsafe { host::instance_get_dir(instance_id_params(instance_id)) }?
            .into_result()?
            .path,
    )
}

/// The sandbox path of this plugin's folder in the instance,
/// `/instances/<id>/.metadata/plugins/<plugin id>`.
pub fn instance_plugin_get_dir(instance_id: &str) -> crate::Result<String> {
    Ok(
        unsafe { host::instance_plugin_get_dir(instance_id_params(instance_id)) }?
            .into_result()?
            .path,
    )
}

/// Create an instance and return its id.
pub fn instance_create(new_instance: NewInstanceDto) -> crate::Result<String> {
    Ok(unsafe { host::instance_create(Wire(new_instance)) }?
        .into_result()?
        .instance_id)
}

/// The content files of the instance, by their path in it.
pub fn list_content(instance_id: &str) -> crate::Result<HashMap<String, ContentFileDto>> {
    Ok(
        unsafe { host::list_content(instance_id_params(instance_id)) }?
            .into_result()?
            .files,
    )
}

pub fn enable_contents(instance_id: &str, paths: Vec<String>) -> crate::Result<()> {
    unsafe {
        host::enable_contents(Wire(ChangeContentsParamsDto {
            instance_id: instance_id.to_owned(),
            paths,
        }))
    }?
    .into_result()
}

pub fn disable_contents(instance_id: &str, paths: Vec<String>) -> crate::Result<()> {
    unsafe {
        host::disable_contents(Wire(ChangeContentsParamsDto {
            instance_id: instance_id.to_owned(),
            paths,
        }))
    }?
    .into_result()
}

/// Write a file into an instance directory through the host (T-1.3).
///
/// `params.relative_path` is relative to the instance root and `/`-separated; the plugin never
/// touches the instance directory through `std::fs` itself.
pub fn write_instance_file(params: WriteInstanceFileParamsDto) -> crate::Result<()> {
    unsafe { host::write_instance_file(Wire(params)) }?.into_result()
}
