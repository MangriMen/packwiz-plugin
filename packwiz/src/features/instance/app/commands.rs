use std::collections::HashMap;

use extism_pdk::Msgpack;
use serr::SerializedError;

use crate::features::instance::{host, ContentFile, NewInstance};

pub fn instance_get_dir(instance_id: String) -> Result<String, SerializedError> {
    unsafe { host::instance_get_dir(instance_id) }
        .expect("Can't get instance directory")
        .to_result()
}

pub fn instance_plugin_get_dir(instance_id: String) -> Result<String, SerializedError> {
    unsafe { host::instance_plugin_get_dir(instance_id) }
        .expect("Can't get instance directory")
        .to_result()
}

pub fn instance_create(new_instance: NewInstance) -> Result<String, SerializedError> {
    unsafe { host::instance_create(new_instance) }
        .expect("Can't create instance")
        .to_result()
}

pub fn list_content(instance_id: String) -> Result<HashMap<String, ContentFile>, SerializedError> {
    unsafe { host::list_content(instance_id) }
        .expect("Can't list content")
        .to_result()
}

pub fn enable_contents(
    instance_id: String,
    content_paths: Msgpack<Vec<String>>,
) -> Result<(), SerializedError> {
    unsafe { host::enable_contents(instance_id, content_paths) }
        .expect("Can't enable contents")
        .to_result()
}

pub fn disable_contents(
    instance_id: String,
    content_paths: Msgpack<Vec<String>>,
) -> Result<(), SerializedError> {
    unsafe { host::disable_contents(instance_id, content_paths) }
        .expect("Can't disable contents")
        .to_result()
}
