use extism_pdk::{FromBytes, Json};

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Clone, FromBytes)]
#[encoding(Json)]
pub struct Java {
    pub major_version: u32,
    pub version: String,
    pub architecture: String,
    pub path: String,
}
