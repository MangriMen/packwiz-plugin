use serr::SerializedError;

use crate::features::java::{host, Java};

pub fn install_java(version: u32) -> Result<Java, SerializedError> {
    unsafe { host::install_java(version) }
        .expect("Can't install java")
        .to_result()
}
