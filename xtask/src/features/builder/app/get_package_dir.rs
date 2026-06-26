use std::path::PathBuf;

use crate::features::builder::{BuildProfile, get_target_dir};

pub fn get_package_dir(target: &str, profile: BuildProfile) -> PathBuf {
    let target_dir = get_target_dir(target, profile);
    let package_dir = target_dir.join("package");
    std::fs::create_dir_all(&package_dir).unwrap();
    package_dir
}
