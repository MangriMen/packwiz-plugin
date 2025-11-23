use crate::serializable_error::SerializableError;

#[derive(Debug)]
pub struct Error(pub String);

impl From<Error> for extism_pdk::Error {
    fn from(value: Error) -> Self {
        extism_pdk::Error::msg(value.0)
    }
}

impl From<extism_pdk::Error> for Error {
    fn from(value: extism_pdk::Error) -> Self {
        Error(value.to_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error(value)
    }
}

impl From<SerializableError> for Error {
    fn from(value: SerializableError) -> Self {
        Error(value.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
