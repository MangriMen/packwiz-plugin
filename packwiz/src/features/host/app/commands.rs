use aether_core_plugin_pdk::{
    Wire,
    api::v0::{CommandDto, OutputDto},
};

use crate::features::host::host;

/// Run `command` through the host and return its output, whatever its exit code: a non-zero exit
/// is the program's answer, which the caller judges (`OutputDto::success`).
pub fn run_command(command: CommandDto) -> crate::Result<OutputDto> {
    unsafe { host::run_command(Wire(command)) }?.into_result()
}
