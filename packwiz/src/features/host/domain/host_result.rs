use extism_pdk::FromBytes;
use extism_pdk::Msgpack;
use serde::{Deserialize, Serialize};

/// Mirror of the host's `HostResult` — must match `#[serde(tag = "status", content = "data")]`
/// and Msgpack encoding used by `to_extism_res` in aether-core.
#[derive(Serialize, Deserialize, FromBytes, Debug)]
#[encoding(Msgpack)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub enum HostResult<T> {
    Ok(T),
    Err(aether_core_plugin_api::v0::HostError),
}

impl<T> HostResult<T> {
    /// Convert to `Result<T, HostError>`, for use with `?` and `From<HostError> for Error`.
    pub fn into_result(self) -> Result<T, aether_core_plugin_api::v0::HostError> {
        match self {
            HostResult::Ok(val) => Ok(val),
            HostResult::Err(e) => Err(e),
        }
    }
}
