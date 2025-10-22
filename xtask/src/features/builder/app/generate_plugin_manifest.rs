use std::path::PathBuf;

use crate::features::{
    builder::{ApiConfig, LoadConfig, PluginManifest, PluginMetadata, RuntimeConfig},
    cargo::CargoMetadata,
};

pub fn generate_plugin_manifest(meta: &CargoMetadata) -> PluginManifest {
    PluginManifest {
        metadata: PluginMetadata {
            id: meta.package.name.clone(),
            name: meta.package.metadata.name.clone(),
            version: meta.package.version.clone(),
            description: meta.package.description.clone(),
            authors: meta.package.authors.clone().unwrap_or_default(),
            license: meta.package.license.clone(),
        },
        runtime: RuntimeConfig {
            allowed_hosts: meta
                .package
                .metadata
                .allowed_hosts
                .clone()
                .unwrap_or_default(),
            allowed_paths: meta
                .package
                .metadata
                .allowed_paths
                .clone()
                .unwrap_or_default(),
        },
        load: LoadConfig::Extism {
            file: PathBuf::from(format!("{}.wasm", meta.package.name)),
            memory_limit: None,
        },
        api: ApiConfig {
            version: meta.package.metadata.api_version.clone(),
            features: Vec::default(),
        },
    }
}
