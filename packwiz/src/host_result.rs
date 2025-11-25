use extism_pdk::{encoding, FromBytes, Msgpack};
use serde::{Deserialize, Serialize};
use serr::SerializedError;

#[derive(Serialize, Deserialize, FromBytes, Debug)]
#[encoding(Msgpack)]
pub enum HostResult<T> {
    Ok(T),
    Err(SerializedError),
}

// impl<T> From<crate::Result<T>> for HostResult<T> {
//     fn from(res: crate::Result<T>) -> Self {
//         match res {
//             Ok(v) => Self::Ok(v),
//             Err(e) => Self::Err(e.raw.to_serialized()),
//         }
//     }
// }

impl<T> From<HostResult<T>> for Result<T, SerializedError> {
    fn from(value: HostResult<T>) -> Self {
        match value {
            HostResult::Ok(t) => Ok(t),
            HostResult::Err(e) => Err(e),
        }
    }
}

impl<T> HostResult<T> {
    pub fn to_result(self) -> Result<T, SerializedError> {
        self.into()
    }
}
