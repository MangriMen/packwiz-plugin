use serr::SerializedError;

use crate::features::java::{host, Java};

pub fn get_java(version: u32) -> Result<Java, SerializedError> {
    unsafe { host::get_java(version) }
        .expect("Can't get java")
        .to_result()
}
