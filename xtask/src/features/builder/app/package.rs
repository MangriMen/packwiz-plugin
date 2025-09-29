use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use zip::write::{ExtendedFileOptions, FileOptions};

use crate::features::builder::{bundle, get_package_dir, BuildProfile, TARGET};
use crate::features::cargo::CargoMetadata;

pub fn package(
    plugin_name: &str,
    working_dir: &str,
    profile: BuildProfile,
    metadata: &CargoMetadata,
) -> PathBuf {
    let package_dir = get_package_dir(TARGET, profile);

    bundle(plugin_name, working_dir, profile, metadata);

    let zip_path = package_dir.join(plugin_name).with_extension("zip");

    pack_to_zip(&package_dir, &get_files_to_pack(plugin_name), &zip_path);

    zip_path
}

fn get_files_to_pack(plugin_name: &str) -> Vec<String> {
    [
        format!("{}.wasm", &plugin_name),
        "plugin.toml".to_owned(),
        "icon.png".to_owned(),
    ]
    .to_vec()
}

fn pack_to_zip(input_path: &Path, files: &Vec<String>, out_path: &Path) {
    let zip_file = File::create(out_path).unwrap();

    let writer = BufWriter::new(zip_file);
    let mut zip = zip::ZipWriter::new(writer);

    let options: FileOptions<ExtendedFileOptions> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for file in files {
        let file_path = input_path.join(file);

        if file_path.exists() {
            zip.start_file(file, options.clone()).unwrap();
            let data = fs::read(&file_path).unwrap();
            zip.write_all(&data).unwrap();
        }
    }

    zip.finish().unwrap();
}
