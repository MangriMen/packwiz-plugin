use serde::Deserialize;

use crate::features::builder::PathMapping;

#[derive(Deserialize)]
pub struct CargoMetadata {
    pub package: Package,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    pub version: semver::Version,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub metadata: PackageMetadata,
}

#[derive(Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub api_version: semver::VersionReq,
    pub allowed_hosts: Option<Vec<String>>,
    pub allowed_paths: Option<Vec<PathMapping>>,
}
