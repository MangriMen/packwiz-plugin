use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub metadata: PluginMetadata,
    pub runtime: RuntimeConfig,
    pub load: LoadConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: semver::Version,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeConfig {
    #[serde(default)]
    pub allowed_hosts: Vec<String>,

    #[serde(default)]
    pub allowed_paths: Vec<PathMapping>,
}

pub type PathMapping = (String, PathBuf); // (path on disk, plugin path)

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LoadConfigType {
    Extism,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum LoadConfig {
    #[serde(rename_all = "camelCase")]
    Extism {
        file: PathBuf,
        #[serde(default)]
        memory_limit: Option<usize>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    pub version: semver::VersionReq,
    #[serde(default)]
    pub features: Vec<String>,
    /// Host functions the plugin imports; the core links only these. Left out of the manifest
    /// when `[package.metadata]` has no `host_functions`, which makes the core link all of them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_functions: Option<Vec<String>>,
}
