use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PluginManifest {
    pub metadata: PluginMetadata,
    pub runtime: RuntimeConfig,
    pub load: LoadConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: semver::Version,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuntimeConfig {
    #[serde(default)]
    pub allowed_hosts: Vec<String>,

    #[serde(default)]
    pub allowed_paths: Vec<PathMapping>,
}

pub type PathMapping = (String, PathBuf);

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum LoadConfigType {
    Extism,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LoadConfig {
    Extism {
        file: PathBuf,
        #[serde(default)]
        memory_limit: Option<usize>,
    },
    Native {
        lib_path: PathBuf,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiConfig {
    pub version: semver::VersionReq,
    #[serde(default)]
    pub features: Vec<String>,
}
