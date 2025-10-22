use extism_pdk::{Json, ToBytes};
use serde::{Deserialize, Serialize};

use crate::models::{LoaderVersionPreference, ModLoader, PackInfo};

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
