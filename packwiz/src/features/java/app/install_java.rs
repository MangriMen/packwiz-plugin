use aether_core_plugin_pdk::{
    Wire,
    api::v0::{JavaDto, JavaVersionParamsDto},
};

use crate::features::java::host;

/// Have the launcher install the Java of `version`.
pub fn install_java(version: u32) -> crate::Result<JavaDto> {
    unsafe { host::install_java(Wire(JavaVersionParamsDto { version })) }?.into_result()
}
