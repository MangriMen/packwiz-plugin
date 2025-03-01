use std::{collections::BTreeMap, path::PathBuf};

use url::Url;

use crate::models::PackwizPack;

pub fn get_pack_from_path_or_url(path_or_url: &str) -> crate::Result<PackwizPack> {
    let path = PathBuf::from(path_or_url);

    let data = if path.exists() && path.metadata().is_ok() {
        &std::fs::read(&path).map_err(|_| format!("Failed to read pack from {:?}", &path))?
    } else {
        let url = Url::parse(path_or_url).unwrap();

        let response = extism_pdk::http::request::<()>(
            &extism_pdk::HttpRequest {
                url: url.to_string(),
                headers: BTreeMap::default(),
                method: Some("GET".to_string()),
            },
            None,
        )
        .map_err(|_| format!("Failed to download pack from {}", url))?;

        &response.body()
    };

    let string_data = std::str::from_utf8(data).map_err(|_| "Failed to parse pack as UTF-8")?;
    let pack = toml::from_str(string_data).map_err(|_| "Failed to parse pack as TOML")?;

    Ok(pack)
}
