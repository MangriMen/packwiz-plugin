use extism_convert::{encoding, Json};
use extism_pdk::FromBytes;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromBytes)]
#[encoding(Json)]
pub enum PluginEvent {
    BeforeInstanceLaunch { instance_id: String },
    AfterInstanceLaunch { instance_id: String },
}
