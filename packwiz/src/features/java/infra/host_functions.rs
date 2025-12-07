pub mod host {
    use extism_pdk::host_fn;

    use crate::features::{host::HostResult, java::Java};

    #[host_fn]
    extern "ExtismHost" {
        pub fn get_java(version: u32) -> HostResult<Java>;
        pub fn install_java(version: u32) -> HostResult<Java>;
    }
}
