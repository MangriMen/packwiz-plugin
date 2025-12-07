use extism_pdk::FromBytes;
use extism_pdk::{Json, ToBytes};
use serde::{Deserialize, Serialize};

use crate::features::instance::{LoaderVersionPreference, ModLoader, PackInfo};

#[derive(Debug, Serialize, Deserialize, ToBytes)]
#[encoding(Json)]
#[serde(rename_all = "camelCase")]
pub struct NewInstance {
    pub name: String,
    pub game_version: String,
    pub mod_loader: ModLoader,
    pub loader_version: Option<LoaderVersionPreference>,
    pub icon_path: Option<String>,
    pub skip_install_instance: Option<bool>,
    pub pack_info: Option<PackInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes)]
#[encoding(Json)]
#[serde(rename_all = "camelCase")]
pub struct PluginImportInstance {
    pub importer_id: String,
    pub path: String,
}
