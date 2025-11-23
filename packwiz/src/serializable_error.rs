use std::borrow::Cow;

use extism_pdk::{encoding, FromBytes, Msgpack};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableError {
    pub code: Cow<'static, str>,
    pub fields: Option<serde_json::Value>,
    pub message: String,
}

#[derive(Serialize, Deserialize, FromBytes, Debug)]
#[encoding(Msgpack)]
pub enum SerializableResult<T> {
    Ok(T),
    Err(SerializableError),
}

pub type SResult<T> = core::result::Result<T, SerializableError>;

impl<T> From<SerializableResult<T>> for SResult<T> {
    fn from(value: SerializableResult<T>) -> Self {
        match value {
            SerializableResult::Ok(t) => Ok(t),
            SerializableResult::Err(e) => Err(e),
        }
    }
}

impl<T> SerializableResult<T> {
    pub fn to_result(self) -> SResult<T> {
        self.into()
    }
}
