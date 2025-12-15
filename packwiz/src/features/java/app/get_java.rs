use aether_core_plugin_api::v0::JavaDto;
use serr::SerializedError;

use crate::features::java::host;

pub fn get_java(version: u32) -> Result<JavaDto, SerializedError> {
    unsafe { host::get_java(version) }
        .expect("Can't get java")
        .to_result()
}
