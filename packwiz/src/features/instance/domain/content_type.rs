use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Mod,
    DataPack,
    ResourcePack,
    ShaderPack,
}
