use extism_pdk::host_fn;

use crate::{
    helpers::SerializableOutput,
    models::{Java, NewInstance, SerializableCommand},
};

#[host_fn]
extern "ExtismHost" {
    pub fn log(level: u32, msg: String);
    pub fn get_id() -> String;

    pub fn instance_get_dir(instance_id: String) -> String;
    pub fn instance_plugin_get_dir(instance_id: String) -> String;

    pub fn instance_create(new_instance: NewInstance) -> String;

    pub fn get_or_download_java(version: u32) -> Java;

    pub fn run_command(command: SerializableCommand) -> SerializableOutput;
}
