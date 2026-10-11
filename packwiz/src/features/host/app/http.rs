use std::collections::HashMap;

use aether_core_plugin_api::v0::HttpRequestDto;
use extism_pdk::Msgpack;

use crate::features::host::host;

/// GET `url` through the host's `http_get` and return the response body.
///
/// The host answers any HTTP status with a response, so a non-2xx status is turned into an
/// error here: a 404 page must never be parsed as a pack or saved as a jar.
pub fn http_get_bytes(url: &str) -> crate::Result<Vec<u8>> {
    let response = unsafe {
        host::http_get(Msgpack(HttpRequestDto {
            url: url.to_string(),
            headers: HashMap::new(),
        }))
    }?
    .into_result()?;

    if !(200..300).contains(&response.status) {
        return Err(format!("{url} responded with HTTP {}", response.status).into());
    }

    Ok(response.body)
}
