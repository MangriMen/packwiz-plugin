use std::path::PathBuf;

use crate::features::builder::BuildProfile;

pub fn get_target_dir(target: &str, profile: BuildProfile) -> PathBuf {
    PathBuf::from("target").join(target).join(profile.as_str())
}
