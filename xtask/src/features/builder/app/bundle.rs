use std::fs::{self};
use std::path::{Path, PathBuf};

use crate::features::builder::{build, generate_plugin_manifest, BuildProfile};
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

    let wasm_src = target_dir.join(plugin_name).with_extension("wasm");
    let wasm_dst = bundle_dir.join(wasm_src.file_name().unwrap());
    fs::copy(&wasm_src, wasm_dst).unwrap();

    let plugin_manifest = generate_plugin_manifest(metadata);
    let manifest_dst = bundle_dir.join("manifest.json");
    fs::write(
        manifest_dst,
        serde_json::to_string_pretty(&plugin_manifest).unwrap(),
    )
    .unwrap();

    let project_dir = Path::new(plugin_name);

    let plugin_capabilities_src = project_dir.join("capabilities.json");
    let plugin_capabilities_dst = bundle_dir.join("capabilities.json");
    fs::copy(plugin_capabilities_src, plugin_capabilities_dst).unwrap();

    let icon_src = project_dir.join("icon.png");
    if icon_src.exists() {
        let icon_dst = bundle_dir.join("icon.png");
        fs::copy(icon_src, icon_dst).unwrap();
    }

    bundle_dir
}
