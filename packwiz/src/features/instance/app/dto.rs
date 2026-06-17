use extism_pdk::FromBytes;
use extism_pdk::Msgpack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes)]
#[encoding(Msgpack)]
#[serde(rename_all = "camelCase")]
pub struct PluginImportInstance {
    pub importer_id: String,
    pub path: String,
}
