use crate::domain::MyAppError;
use tracing::debug;

impl From<reqwest::Error> for MyAppError {
    fn from(err: reqwest::Error) -> Self {
        debug!("reqwest error {:?}", err);
        Self::InternalError {
            msg: "api error".to_string(),
        }
    }
}

