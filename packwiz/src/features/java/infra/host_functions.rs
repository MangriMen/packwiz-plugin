pub mod host {
    use aether_core_plugin_pdk::{
        HostReply, Wire,
        api::v0::{JavaDto, JavaVersionParamsDto},
    };
    use extism_pdk::host_fn;

    #[host_fn]
    extern "ExtismHost" {
        pub fn get_java(params: Wire<JavaVersionParamsDto>) -> HostReply<JavaDto>;
        pub fn install_java(params: Wire<JavaVersionParamsDto>) -> HostReply<JavaDto>;
    }
}
