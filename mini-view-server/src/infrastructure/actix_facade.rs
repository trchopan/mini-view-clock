use crate::domain::MyAppError;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

impl error::ResponseError for MyAppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(self).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyAppError::InternalError { msg: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            MyAppError::BadClientData { msg: _ } => StatusCode::BAD_REQUEST,
            MyAppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
