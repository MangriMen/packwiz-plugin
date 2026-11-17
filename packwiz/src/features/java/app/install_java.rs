use aether_core_plugin_pdk::{self as pdk, api::v0::JavaDto};

/// Have the launcher install the Java of `version`.
pub fn install_java(version: u32) -> crate::Result<JavaDto> {
    pdk::host::install_java(version)
}
