pub mod host {
    use std::collections::HashMap;

    use aether_core_plugin_api::v0::{ContentFileDto, NewInstanceDto, WriteInstanceFileParamsDto};
    use extism_pdk::{Msgpack, host_fn};

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

        /// The only way to put a file into an instance directory (T-1.3): the plugin names the
        /// instance by id and the file by a path relative to the instance root, the host decides
        /// where that lands on disk and enforces the segment rules and the 64 MiB cap.
        pub fn write_instance_file(params: Msgpack<WriteInstanceFileParamsDto>) -> HostResult<()>;
    }
}

use std::collections::HashMap;

use aether_core_plugin_api::v0::{ContentFileDto, NewInstanceDto};
use extism_pdk::Msgpack;

use crate::features::host::HostResult;

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
        .expect("Can't enable content")
}

pub fn disable_contents(instance_id: String, content_paths: Vec<String>) -> HostResult<()> {
    unsafe { host::disable_contents(instance_id, Msgpack(content_paths)) }
        .expect("Can't disable content")
}
