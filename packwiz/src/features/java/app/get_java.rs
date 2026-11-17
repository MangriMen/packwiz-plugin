use aether_core_plugin_pdk::{
    Wire,
    api::v0::{JavaDto, JavaVersionParamsDto},
};

use crate::features::java::host;

/// The Java of `version` the launcher already has.
pub fn get_java(version: u32) -> crate::Result<JavaDto> {
    unsafe { host::get_java(Wire(JavaVersionParamsDto { version })) }?.into_result()
}
