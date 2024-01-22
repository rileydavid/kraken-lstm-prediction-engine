use crate::error::generic_error::{ErrorBody, GenericError};

// body
fn general_error_body(err: String) -> ErrorBody {
    ErrorBody::new(
        format!("General error occurred: '{}'", err),
        "appWebError001",
    )
}

fn general_error_for_param_body(param: String) -> ErrorBody {
    ErrorBody::new(
        format!("General error occurred for param: '{}'", param),
        "appWebError002",
    )
}

fn general_error_for_param_value_body(param: String, value: String) -> ErrorBody {
    ErrorBody::new(
        format!(
            "General error occurred for param: '{}' value: '{}'",
            param, value
        ),
        "appWebError003",
    )
}
fn failed_getting_db_connection(err: String) -> ErrorBody {
    ErrorBody::new(
        format!("Failed getting db connection: {}", err),
        "appWebError004",
    )
}

pub struct WebError;

impl WebError {
    pub fn general_error(err: String) -> GenericError {
        GenericError::Web(general_error_body(err))
    }

    pub fn general_error_for_param(param: String) -> GenericError {
        GenericError::Web(general_error_for_param_body(param))
    }

    pub fn general_error_for_param_value(param: String, value: String) -> GenericError {
        GenericError::Web(general_error_for_param_value_body(param, value))
    }

    pub fn failed_getting_db_connection(err: String) -> GenericError {
        GenericError::Web(failed_getting_db_connection(err))
    }
}
