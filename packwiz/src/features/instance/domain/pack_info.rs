use extism_pdk::{encoding, Json, ToBytes};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, ToBytes)]
#[encoding(Json)]
#[serde(rename_all = "camelCase")]
pub struct PackInfo {
    pub modpack_id: String,
    pub version: String,
}
