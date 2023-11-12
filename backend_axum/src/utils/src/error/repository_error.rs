use crate::error::generic_error::{ErrorBody, GenericError};

// body
fn general_error_body(err: String) -> ErrorBody {
    ErrorBody::new(
        format!("General error occurred: '{}'", err),
        "appRepositoryError001",
    )
}

fn general_error_for_param_body(param: String) -> ErrorBody {
    ErrorBody::new(
        format!("General error occurred for param: '{}'", param),
        "appRepositoryError002",
    )
}

fn general_error_for_param_value_body(param: String, value: String) -> ErrorBody {
    ErrorBody::new(
        format!(
            "General error occurred for param: '{}' value: '{}'",
            param, value
        ),
        "appRepositoryError003",
    )
}

pub struct RepositoryError;

impl RepositoryError {
    pub fn general_error(err: String) -> GenericError {
        GenericError::Repository(general_error_body(err))
    }

    pub fn general_error_for_param(param: String) -> GenericError {
        GenericError::Repository(general_error_for_param_body(param))
    }

    pub fn general_error_for_param_value(param: String, value: String) -> GenericError {
        GenericError::Repository(general_error_for_param_value_body(param, value))
    }
}
