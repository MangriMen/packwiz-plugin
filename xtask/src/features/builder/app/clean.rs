use std::fs;

use crate::features::builder::{BuildProfile, TARGET, get_target_dir};

pub fn clean(profile: BuildProfile) {
    let target_dir = get_target_dir(TARGET, profile);
    let _ = fs::remove_dir_all(target_dir);
    println!("Build artifacts are deleted");
}
