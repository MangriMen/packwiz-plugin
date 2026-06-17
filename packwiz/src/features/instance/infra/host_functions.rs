pub mod host {
    use std::collections::HashMap;

    use aether_core_plugin_api::v0::{ContentFileDto, NewInstanceDto};
    use extism_pdk::{host_fn, Msgpack};

    use crate::features::host::HostResult;

    #[host_fn]
    extern "ExtismHost" {
        pub fn instance_get_dir(instance_id: String) -> HostResult<String>;
        pub fn instance_plugin_get_dir(instance_id: String) -> HostResult<String>;

        pub fn instance_create(new_instance: Msgpack<NewInstanceDto>) -> HostResult<String>;

        pub fn list_content(instance_id: String) -> HostResult<HashMap<String, ContentFileDto>>;
        pub fn enable_contents(
            instance_id: String,
            content_paths: Msgpack<Vec<String>>,
        ) -> HostResult<()>;
        pub fn disable_contents(
            instance_id: String,
            content_paths: Msgpack<Vec<String>>,
        ) -> HostResult<()>;
    }
}

use std::collections::HashMap;

use aether_core_plugin_api::v0::{ContentFileDto, NewInstanceDto};
use extism_pdk::Msgpack;

pub fn instance_get_dir(instance_id: String) -> Result<String, String> {
    unsafe { host::instance_get_dir(instance_id) }
        .expect("Can't get instance directory")
        .to_result()
}

pub fn instance_plugin_get_dir(instance_id: String) -> Result<String, String> {
    unsafe { host::instance_plugin_get_dir(instance_id) }
        .expect("Can't get instance directory")
        .to_result()
}

pub fn instance_create(new_instance: NewInstanceDto) -> Result<String, String> {
    unsafe { host::instance_create(Msgpack(new_instance)) }
        .expect("Can't create instance")
        .to_result()
}

pub fn list_content(instance_id: String) -> Result<HashMap<String, ContentFileDto>, String> {
    unsafe { host::list_content(instance_id) }
        .expect("Can't list content")
        .to_result()
}

pub fn enable_contents(instance_id: String, content_paths: Vec<String>) -> Result<(), String> {
    unsafe { host::enable_contents(instance_id, Msgpack(content_paths)) }
        .expect("Can't enable content")
        .to_result()
}

pub fn disable_contents(instance_id: String, content_paths: Vec<String>) -> Result<(), String> {
    unsafe { host::disable_contents(instance_id, Msgpack(content_paths)) }
        .expect("Can't disable content")
        .to_result()
}
