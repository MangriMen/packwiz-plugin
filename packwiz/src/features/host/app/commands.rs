use aether_core_plugin_api::v0::{CommandDto, OutputDto};
use extism_pdk::Msgpack;

use crate::features::host::host;

pub fn get_id() -> crate::Result<String> {
    Ok(unsafe { host::get_id() }.expect("Can't get id"))
}

pub fn run_command(command: CommandDto) -> crate::Result<OutputDto> {
    Ok(unsafe { host::run_command(Msgpack(command)) }
        .expect("Can't run command")
        .into_result()?)
}
