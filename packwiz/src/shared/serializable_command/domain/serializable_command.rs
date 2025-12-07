use std::path::PathBuf;

use extism_pdk::{Json, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, ToBytes)]
#[encoding(Json)]
pub struct SerializableCommand {
    pub program: String,
    pub args: Vec<String>,
    pub current_dir: Option<PathBuf>,
}

impl SerializableCommand {
    pub fn from_command(command: &std::process::Command) -> Self {
        Self {
            program: command.get_program().to_string_lossy().to_string(),
            args: command
                .get_args()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
            current_dir: command.get_current_dir().map(|dir| dir.to_path_buf()),
        }
    }

    pub fn to_command(&self) -> std::process::Command {
        let mut command = std::process::Command::new(&self.program);
        if let Some(current_dir) = &self.current_dir {
            command.current_dir(current_dir);
        }
        command.args(&self.args);
        command
    }
}
