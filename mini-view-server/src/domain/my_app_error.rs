use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Serialize, Debug, Display, Error)]
#[serde(tag = "code")]
pub enum MyAppError {
    InternalError { msg: String },
    BadClientData { msg: String },
    NotFound,
}
