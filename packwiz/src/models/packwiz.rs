#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizPack {
    pub name: String,
    pub author: String,
    pub version: String,
    pub pack_format: Option<String>,
    pub description: Option<String>,

    pub index: PackIndex,
    pub versions: PackVersions,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PackIndex {
    file: String,
    hash_format: String,
    hash: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PackVersions {
    pub minecraft: String,
    pub fabric: Option<String>,
    pub forge: Option<String>,
    pub liteloader: Option<String>,
    pub quilt: Option<String>,
}
