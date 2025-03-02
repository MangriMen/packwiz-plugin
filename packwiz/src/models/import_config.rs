use extism_pdk::{encoding, Json, ToBytes};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, ToBytes)]
#[encoding(Json)]
#[serde(rename_all = "camelCase")]
pub struct ImportConfig {
    pub pack_type: String,
    pub title: String,
    pub field_label: String,
    pub file_name: String,
    pub file_extensions: Vec<String>,
}
