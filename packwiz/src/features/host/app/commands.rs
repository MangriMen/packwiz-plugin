use aether_core_plugin_api::v0::{CommandDto, OutputDto};
use extism_pdk::Msgpack;
use serr::SerializedError;

use crate::features::host::host;

pub fn get_id() -> crate::Result<String> {
    Ok(unsafe { host::get_id() }.expect("Can't get id"))
}

pub fn run_command(command: CommandDto) -> Result<OutputDto, SerializedError> {
    unsafe { host::run_command(Msgpack(command)) }
        .expect("Can't run command")
        .to_result()
}
