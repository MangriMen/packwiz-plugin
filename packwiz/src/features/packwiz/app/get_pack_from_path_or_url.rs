use std::path::PathBuf;

use extism_pdk::{http, HttpRequest};
use path_slash::PathBufExt;
use url::Url;

use crate::features::packwiz::PackwizPack;

pub fn get_pack_from_path_or_url(path_or_url: &str) -> crate::Result<PackwizPack> {
    let data = match get_path_type(path_or_url) {
        PathType::Path(path) => {
            let path = path.to_slash_lossy().to_string();
            &std::fs::read(&path).map_err(|_| format!("Failed to read pack from {:?}", &path))?
        }
        PathType::Url(url) => {
            let response =
                http::request::<()>(&HttpRequest::new(url.clone()).with_method("GET"), None)
                    .map_err(|_| format!("Failed to download pack from {}", url))?;

            &response.body()
        }
        _ => return Err(format!("Invalid path or URL: {:?}", path_or_url).into()),
    };

    let string_data = std::str::from_utf8(data).map_err(|_| "Failed to parse pack as UTF-8")?;
    let pack = toml::from_str(string_data).map_err(|_| "Failed to parse pack as TOML")?;

    Ok(pack)
}

enum PathType {
    Unknown,
    Path(PathBuf),
    Url(Url),
}

fn get_path_type(path_or_url: &str) -> PathType {
    let path = PathBuf::from(path_or_url);
    if path.exists() && path.metadata().is_ok() {
        return PathType::Path(path);
    }

    match Url::parse(path_or_url) {
        Ok(url) => PathType::Url(url),
        Err(_) => PathType::Unknown,
    }
}
