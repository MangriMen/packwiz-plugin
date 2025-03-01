use std::{path::PathBuf, process::Command};

pub enum BuildProfile {
    Release,
    Debug,
}

impl BuildProfile {
    pub fn as_str(&self) -> &str {
        match self {
            BuildProfile::Release => "release",
            BuildProfile::Debug => "debug",
        }
    }
}

pub fn build(dir_name: &str, build_profile: &BuildProfile) -> Result<PathBuf, String> {
    let profile = format!("--{}", build_profile.as_str());
    let target = "wasm32-wasip1";

    Command::new("cargo")
        .current_dir(dir_name)
        .args(["build", "--target", target, &profile])
        .status()
        .map_err(|_| "Failed to build packwiz")?;

    let target_dir = PathBuf::from("target")
        .join(target)
        .join(build_profile.as_str());

    Ok(target_dir)
}
