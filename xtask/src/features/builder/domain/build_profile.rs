#[derive(Debug, Clone, Copy)]
pub enum BuildProfile {
    Release,
    Debug,
}

impl BuildProfile {
    pub fn as_str(&self) -> &str {
        match self {
            BuildProfile::Release => "release",
            BuildProfile::Debug => "debug",
        }
    }
}
