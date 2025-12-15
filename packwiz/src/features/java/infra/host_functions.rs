pub mod host {
    use aether_core_plugin_api::v0::JavaDto;
    use extism_pdk::host_fn;

    use crate::features::host::HostResult;

    #[host_fn]
    extern "ExtismHost" {
        pub fn get_java(version: u32) -> HostResult<JavaDto>;
        pub fn install_java(version: u32) -> HostResult<JavaDto>;
    }
}
