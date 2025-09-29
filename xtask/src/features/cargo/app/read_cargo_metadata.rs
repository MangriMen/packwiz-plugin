use std::{fs, path::Path};

use crate::features::cargo::CargoMetadata;

pub fn read_metadata(path: &Path) -> CargoMetadata {
    let cargo_toml_content = fs::read_to_string(path).expect("Can't read Cargo.toml");

    let cargo: CargoMetadata =
        toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    cargo
}
