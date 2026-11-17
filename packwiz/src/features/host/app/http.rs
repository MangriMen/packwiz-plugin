use std::collections::HashMap;

use aether_core_plugin_pdk::{self as pdk, api::v0::HttpRequestDto};

/// GET `url` through the host's `http_get` and return the response body.
///
/// The host answers any HTTP status with a response, so a non-2xx status is turned into an
/// error here: a 404 page must never be parsed as a pack or saved as a jar.
pub fn http_get_bytes(url: &str) -> crate::Result<Vec<u8>> {
    let response = pdk::host::http_get(HttpRequestDto {
        url: url.to_string(),
        headers: HashMap::new(),
    })?;

    if !(200..300).contains(&response.status) {
        return Err(format!("{url} responded with HTTP {}", response.status).into());
    }

    Ok(response.body)
}
