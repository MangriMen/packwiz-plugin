use extism_pdk::{FromBytes, Msgpack};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromBytes)]
#[encoding(Msgpack)]
pub enum PluginEvent {
    Loaded,
    Unloaded,
    BeforeInstanceLaunch { instance_id: String },
    AfterInstanceLaunch { instance_id: String },
}
