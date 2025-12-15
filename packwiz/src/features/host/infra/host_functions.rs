pub mod host {
    use aether_core_plugin_api::v0::{CommandDto, OutputDto};
    use extism_pdk::{host_fn, Msgpack};

    use crate::features::host::HostResult;

    #[host_fn]
    extern "ExtismHost" {
        pub fn log(level: u32, msg: String);
        pub fn get_id() -> String;

        pub fn run_command(command: Msgpack<CommandDto>) -> HostResult<OutputDto>;
    }
}
