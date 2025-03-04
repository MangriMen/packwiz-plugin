use extism_pdk::{encoding, Json, ToBytes};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, ToBytes)]
#[encoding(Json)]
#[serde(rename_all = "camelCase")]
pub struct PackInfo {
    pub pack_type: String,
    pub pack_version: String,
    pub can_update: bool,
}
