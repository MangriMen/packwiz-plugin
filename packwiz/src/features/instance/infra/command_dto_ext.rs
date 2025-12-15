use aether_core_plugin_api::v0::CommandDto;

pub trait CommandDtoExt {
    fn from_command(command: &std::process::Command) -> Self;
}

impl CommandDtoExt for CommandDto {
    fn from_command(command: &std::process::Command) -> Self {
        Self {
            program: command.get_program().to_string_lossy().to_string(),
            args: command
                .get_args()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
            current_dir: command.get_current_dir().map(|dir| dir.to_path_buf()),
        }
    }
}
