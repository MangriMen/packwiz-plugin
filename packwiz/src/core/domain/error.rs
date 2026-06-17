use aether_core_plugin_api::v0::HostError;
use serr::SerializedError;

#[derive(Debug)]
pub enum Error {
    HostError(HostError),
    Custom(String),
}

impl From<Error> for extism_pdk::Error {
    fn from(value: Error) -> Self {
        extism_pdk::Error::msg(value.to_string())
    }
}

impl From<extism_pdk::Error> for Error {
    fn from(value: extism_pdk::Error) -> Self {
        Error::Custom(value.to_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Custom(value)
    }
}

impl From<SerializedError> for Error {
    fn from(value: SerializedError) -> Self {
        Error::Custom(value.message)
    }
}

impl From<HostError> for Error {
    fn from(value: HostError) -> Self {
        Error::HostError(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HostError(e) => write!(fmt, "{e:?}"),
            Error::Custom(msg) => write!(fmt, "{msg}"),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
