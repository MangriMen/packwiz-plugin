use std::collections::HashMap;

use aether_core_plugin_api::v0::{ContentFileDto, NewInstanceDto};
use extism_pdk::Msgpack;

use crate::features::host::HostResult;
use crate::features::instance::infra::host;

pub fn instance_get_dir(instance_id: String) -> HostResult<String> {
    unsafe { host::instance_get_dir(instance_id) }.expect("Can't get instance directory")
}

pub fn instance_plugin_get_dir(instance_id: String) -> HostResult<String> {
    unsafe { host::instance_plugin_get_dir(instance_id) }.expect("Can't get instance directory")
}

pub fn instance_create(new_instance: NewInstanceDto) -> HostResult<String> {
    unsafe { host::instance_create(Msgpack(new_instance)) }.expect("Can't create instance")
}

pub fn list_content(instance_id: String) -> HostResult<HashMap<String, ContentFileDto>> {
    unsafe { host::list_content(instance_id) }.expect("Can't list content")
}

pub fn enable_contents(instance_id: String, content_paths: Vec<String>) -> HostResult<()> {
    unsafe { host::enable_contents(instance_id, Msgpack(content_paths)) }
        .expect("Can't enable contents")
}

pub fn disable_contents(instance_id: String, content_paths: Vec<String>) -> HostResult<()> {
    unsafe { host::disable_contents(instance_id, Msgpack(content_paths)) }
        .expect("Can't disable contents")
}
