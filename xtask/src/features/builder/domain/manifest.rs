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
    /// A SemVer requirement the launcher checks against the plugin API version it implements.
    pub version: semver::VersionReq,
    /// Host functions the plugin imports; the core links only these.
    pub host_functions: Vec<String>,
}
