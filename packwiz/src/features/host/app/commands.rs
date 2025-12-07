use serr::SerializedError;

use crate::{
    features::host::host,
    shared::serializable_command::{SerializableCommand, SerializableOutput},
};

pub fn get_id() -> crate::Result<String> {
    Ok(unsafe { host::get_id() }.expect("Can't get id"))
}

pub fn run_command(command: SerializableCommand) -> Result<SerializableOutput, SerializedError> {
    unsafe { host::run_command(command) }
        .expect("Can't run command")
        .to_result()
}
