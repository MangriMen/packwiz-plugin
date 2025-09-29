use crate::{
    core::get_cargo_metadata_path,
    features::{
        builder::{self, BUILD_PROFILE, PLUGIN_NAME},
        cargo,
    },
};

pub fn build() {
    builder::build(PLUGIN_NAME, BUILD_PROFILE).expect("Failed to build plugin");

    println!("Plugin built");
}

pub fn bundle() {
    let meta = cargo::read_metadata(&get_cargo_metadata_path(PLUGIN_NAME));

    let bundle_dir = builder::bundle(PLUGIN_NAME, PLUGIN_NAME, BUILD_PROFILE, &meta);

    println!("Plugin bundled in {:?}", bundle_dir);
}

pub fn package() {
    let meta = cargo::read_metadata(&get_cargo_metadata_path(PLUGIN_NAME));

    let package_path = builder::package(PLUGIN_NAME, PLUGIN_NAME, BUILD_PROFILE, &meta);

    println!("Plugin packaged in {:?}", package_path);
}

pub fn clean() {
    builder::clean(BUILD_PROFILE);
}
