use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use zip::write::{ExtendedFileOptions, FileOptions};

use crate::build::BuildProfile;

#[derive(Deserialize)]
struct CargoMetadata {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    description: Option<String>,
    version: semver::Version,
    authors: Option<Vec<String>>,
    license: Option<String>,
    metadata: PackageMetadata,
}

#[derive(Deserialize)]
struct PackageMetadata {
    name: String,
    api_version: semver::VersionReq,
    allowed_hosts: Option<Vec<String>>,
    allowed_paths: Option<Vec<PathMapping>>,
}

pub fn bundle() -> (String, PathBuf) {
    let plugin_name = "packwiz";
    let build_profile = BuildProfile::Release;

    let target_dir = crate::build::build(plugin_name, &build_profile).unwrap();

    let bundle_dir = target_dir.join("bundle");
    std::fs::create_dir_all(&bundle_dir).unwrap();

    let wasm_path = target_dir.join(plugin_name).with_extension("wasm");
    fs::copy(&wasm_path, bundle_dir.join(wasm_path.file_name().unwrap())).unwrap();

    let plugin_cargo_toml = Path::new(plugin_name).join("Cargo.toml");
    let plugin_toml_content = generate_plugin_metadata(&plugin_cargo_toml);
    fs::write(
        bundle_dir.join("plugin.toml"),
        toml::to_string_pretty(&plugin_toml_content).unwrap(),
    )
    .unwrap();

    if Path::new("icon.png").exists() {
        fs::copy("icon.png", bundle_dir.join("icon.png")).unwrap();
    }

    println!("Plugin bundled in {:?}", bundle_dir);

    (plugin_name.to_string(), bundle_dir)
}

pub fn package() {
    let (plugin_name, bundle_dir) = bundle();

    let zip_path = bundle_dir.join(&plugin_name).with_extension("zip");
    let zip_file = File::create(&zip_path).unwrap();
    let writer = BufWriter::new(zip_file);
    let mut zip = zip::ZipWriter::new(writer);

    let options: FileOptions<ExtendedFileOptions> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let zip_files = [&format!("{}.wasm", &plugin_name), "plugin.toml", "icon.png"];

    for file in zip_files {
        let file_path = bundle_dir.join(file);

        if file_path.exists() {
            zip.start_file(file, options.clone()).unwrap();
            let data = fs::read(&file_path).unwrap();
            zip.write_all(&data).unwrap();
        }
    }
    zip.finish().unwrap();

    println!("Plugin packaged in {:?}", zip_path);
}

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

fn generate_plugin_metadata(cargo_toml_path: &Path) -> PluginManifest {
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).expect("Can't read Cargo.toml");

    let cargo: CargoMetadata =
        toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    PluginManifest {
        metadata: PluginMetadata {
            id: cargo.package.name.clone(),
            name: cargo.package.metadata.name,
            version: cargo.package.version,
            description: cargo.package.description,
            authors: cargo.package.authors.unwrap_or_default(),
            license: cargo.package.license,
        },
        runtime: RuntimeConfig {
            allowed_hosts: cargo.package.metadata.allowed_hosts.unwrap_or_default(),
            allowed_paths: cargo.package.metadata.allowed_paths.unwrap_or_default(),
        },
        load: LoadConfig::Extism {
            file: PathBuf::from(format!("{}.wasm", cargo.package.name)),
            memory_limit: None,
        },
        api: ApiConfig {
            version: cargo.package.metadata.api_version,
            features: Vec::default(),
        },
    }
}
