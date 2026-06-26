use std::{path::PathBuf, process::Command};

use crate::features::builder::{BuildProfile, TARGET, get_target_dir};

pub fn build(working_dir: &str, profile: BuildProfile) -> Result<PathBuf, String> {
    let profile_arg = format!("--{}", profile.as_str());

    Command::new("cargo")
        .current_dir(working_dir)
        .args(["build", "--target", TARGET, &profile_arg])
        .status()
        .map_err(|_| "Failed to build packwiz")?;

    Ok(get_target_dir(TARGET, profile))
}
