use std::fs;

use crate::features::builder::{get_target_dir, BuildProfile, TARGET};

pub fn clean(profile: BuildProfile) {
    let target_dir = get_target_dir(TARGET, profile);
    let _ = fs::remove_dir_all(target_dir);
    println!("Build artifacts are deleted");
}
