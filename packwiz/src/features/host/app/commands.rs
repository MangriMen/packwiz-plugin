use aether_core_plugin_pdk::{
    self as pdk,
    api::v0::{CommandDto, OutputDto},
};

/// Run `command` through the host and return its output, whatever its exit code: a non-zero exit
/// is the program's answer, which the caller judges (`OutputDto::success`).
pub fn run_command(command: CommandDto) -> crate::Result<OutputDto> {
    pdk::host::run_command(command)
}
