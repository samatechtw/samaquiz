use super::api_error::ApiError;

use lib_types::shared::api_error::ApiErrorCode;
use validator::ValidationErrors;

pub fn check_bad_form<T>(result: Result<T, ValidationErrors>) -> Result<T, ApiError> {
    result.map_err(|_| ApiError::bad_request().code(ApiErrorCode::InvalidFormData))
}

/// Validates that a string contains only digits
pub fn validate_integer(str: &str) -> Result<(), ApiError> {
    if !str.chars().all(|c| c.is_digit(10)) {
        return Err(ApiError::bad_request().code(ApiErrorCode::InvalidFormData));
    }
    Ok(())
}
