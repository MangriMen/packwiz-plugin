use extism_pdk::host_fn;

use crate::{
    helpers::SerializableOutput,
    models::{Java, NewInstance, SerializableCommand},
    serializable_error::SerializableResult,
};

#[host_fn]
extern "ExtismHost" {
    pub fn log(level: u32, msg: String);
    pub fn get_id() -> String;

    pub fn instance_get_dir(instance_id: String) -> String;
    pub fn instance_plugin_get_dir(instance_id: String) -> String;

    pub fn instance_create(new_instance: NewInstance) -> String;

    pub fn get_java(version: u32) -> SerializableResult<Java>;
    pub fn install_java(version: u32) -> SerializableResult<Java>;

    pub fn run_command(command: SerializableCommand) -> SerializableOutput;
}
