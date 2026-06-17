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
    /// Convert to a `Result`, mapping `HostError` to its debug representation.
    pub fn to_result(self) -> Result<T, String> {
        match self {
            HostResult::Ok(t) => Ok(t),
            HostResult::Err(e) => Err(format!("{e:?}")),
        }
    }
}
