use aether_core_plugin_api::v0::JavaDto;
use serr::SerializedError;

use crate::features::java::host;

pub fn install_java(version: u32) -> Result<JavaDto, SerializedError> {
    unsafe { host::install_java(version) }
        .expect("Can't install java")
        .to_result()
}
