use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utils::error::generic_error::GenericError;
use utils::error::generic_error::GenericErrorTrait;

pub struct ResponseError {
    pub status_code: StatusCode,
    pub message: String,
    pub code: String,
}

impl ResponseError {
    pub fn new(status_code: StatusCode, code: String, message: String) -> Self {
        ResponseError {
            status_code,
            code,
            message,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ResponseErrorBody {
    pub code: String,
    pub message: String,
}

impl ResponseErrorBody {
    pub fn from_response_error(err: ResponseError) -> ResponseErrorBody {
        ResponseErrorBody {
            message: err.message,
            code: err.code,
        }
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(ResponseErrorBody::from_response_error(self)),
        )
            .into_response() as Response
    }
}

pub fn prepare_response<T>(result: Result<T, GenericError>) -> Result<Json<T>, ResponseError> {
    match result {
        Ok(data) => Ok(Json(data)),
        Err(generic_error) => match generic_error {
            GenericError::Repository(err) => Err(ResponseError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                err.get_code(),
                err.get_message(),
            )),
            GenericError::Service(err) => Err(ResponseError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                err.get_code(),
                err.get_message(),
            )),
            GenericError::Web(err) => Err(ResponseError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                err.get_code(),
                err.get_message(),
            )),
        },
    }
}
