use crate::error::generic_error::{ErrorBody, GenericError};

// body
fn general_error_body(err: String) -> ErrorBody {
    ErrorBody::new(
        format!("General error occurred: '{}'", err),
        "appServiceError001",
    )
}

fn general_error_for_param_body(param: String) -> ErrorBody {
    ErrorBody::new(
        format!("General error occurred for param: '{}'", param),
        "appServiceError002",
    )
}

fn general_error_for_param_value_body(param: String, value: String) -> ErrorBody {
    ErrorBody::new(
        format!(
            "General error occurred for param: '{}' value: '{}'",
            param, value
        ),
        "appServiceError003",
    )
}

fn entity_does_not_exists_for_param_value(param: String, value: String) -> ErrorBody {
    ErrorBody::new(
        format!(
            "Entity does not exist for param: '{}' value: '{}'",
            param, value
        ),
        "appServiceError004",
    )
}

fn parameter_must_be_provided(param: String) -> ErrorBody {
    ErrorBody::new(
        format!("Parameter must be provided: '{}'", param),
        "appServiceError005",
    )
}
fn relation_between_2_parameters_does_not_exist(param1: String, param2: String) -> ErrorBody {
    ErrorBody::new(
        format!(
            "Relation between '{}' and '{}' does not exist.",
            param1, param2
        ),
        "appServiceError005",
    )
}

pub struct ServiceError;

//TODO remove functions that are not needed

impl ServiceError {
    pub fn general_error(err: String) -> GenericError {
        GenericError::Service(general_error_body(err))
    }

    pub fn general_error_for_param(param: String) -> GenericError {
        GenericError::Service(general_error_for_param_body(param))
    }

    pub fn general_error_for_param_value(param: String, value: String) -> GenericError {
        GenericError::Service(general_error_for_param_value_body(param, value))
    }
    pub fn entity_does_not_exists_for_param_value(param: String, value: String) -> GenericError {
        GenericError::Service(entity_does_not_exists_for_param_value(param, value))
    }
    pub fn parameter_must_be_provided(param: String) -> GenericError {
        GenericError::Service(parameter_must_be_provided(param))
    }
    pub fn relation_between_2_parameters_does_not_exist(
        param1: String,
        param2: String,
    ) -> GenericError {
        GenericError::Service(relation_between_2_parameters_does_not_exist(param1, param2))
    }
}
