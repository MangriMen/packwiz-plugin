use std::fs::{self};
use std::path::{Path, PathBuf};

use crate::features::builder::{build, generate_plugin_metadata, BuildProfile};
use crate::features::cargo::CargoMetadata;

pub fn bundle(
    plugin_name: &str,
    working_dir: &str,
    profile: BuildProfile,
    metadata: &CargoMetadata,
) -> PathBuf {
    let target_dir = build(working_dir, profile).unwrap();

    let bundle_dir = target_dir.join("bundle");
    std::fs::create_dir_all(&bundle_dir).unwrap();

    let wasm_path = target_dir.join(plugin_name).with_extension("wasm");
    fs::copy(&wasm_path, bundle_dir.join(wasm_path.file_name().unwrap())).unwrap();

    let plugin_toml_content = generate_plugin_metadata(metadata);
    fs::write(
        bundle_dir.join("plugin.toml"),
        toml::to_string_pretty(&plugin_toml_content).unwrap(),
    )
    .unwrap();

    if Path::new("icon.png").exists() {
        fs::copy("icon.png", bundle_dir.join("icon.png")).unwrap();
    }

    bundle_dir
}
