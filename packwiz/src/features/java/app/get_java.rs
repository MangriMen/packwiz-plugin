use aether_core_plugin_api::v0::JavaDto;

use crate::features::host::HostResult;
use crate::features::java::host;

pub fn get_java(version: u32) -> HostResult<JavaDto> {
    unsafe { host::get_java(version) }.expect("Can't get java")
}
