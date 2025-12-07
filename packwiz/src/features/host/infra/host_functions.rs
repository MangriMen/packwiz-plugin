pub mod host {
    use extism_pdk::host_fn;

    use crate::{
        features::{host::HostResult, java::Java},
        shared::serializable_command::{SerializableCommand, SerializableOutput},
    };

    #[host_fn]
    extern "ExtismHost" {
        pub fn log(level: u32, msg: String);
        pub fn get_id() -> String;

        pub fn get_java(version: u32) -> HostResult<Java>;
        pub fn install_java(version: u32) -> HostResult<Java>;

        pub fn run_command(command: SerializableCommand) -> HostResult<SerializableOutput>;

    }
}
