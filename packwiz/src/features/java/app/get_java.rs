use aether_core_plugin_pdk::{self as pdk, api::v0::JavaDto};

/// The Java of `version` the launcher already has.
pub fn get_java(version: u32) -> crate::Result<JavaDto> {
    pdk::host::get_java(version)
}
