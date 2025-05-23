use core::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib_types::shared::api_error::{ApiErrorCode, ErrorResponse};

pub type Result<T> = core::result::Result<T, ApiError>;

// region:    --- ApiError Boilerplate
impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
// end region: --- ApiError Boilerplate

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut response = self.to_error_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
    pub status: StatusCode,
}

pub fn status_from_u16(num: u16) -> StatusCode {
    StatusCode::from_u16(num).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

impl ApiError {
    pub fn not_found() -> ApiError {
        Self {
            code: ApiErrorCode::None,
            message: "Not found".to_string(),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn bad_request() -> ApiError {
        Self {
            code: ApiErrorCode::None,
            message: "Failed to validate request".to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn internal_error() -> ApiError {
        Self {
            code: ApiErrorCode::None,
            message: "Internal Server Error".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn unauthorized() -> ApiError {
        Self {
            code: ApiErrorCode::Unauthorized,
            message: "Unauthorized".to_string(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn forbidden() -> ApiError {
        Self {
            code: ApiErrorCode::None,
            message: "Forbidden".to_string(),
            status: StatusCode::FORBIDDEN,
        }
    }

    // builder
    pub fn code(mut self, code: ApiErrorCode) -> Self {
        self.code = code;
        self
    }

    pub fn message<T: fmt::Display>(mut self, message: T) -> Self {
        self.message = message.to_string();
        self
    }

    // to response
    pub fn to_error_response(&self) -> Response {
        let status_code = self.status;

        let error_response = ErrorResponse {
            code: self.code.to_string(),
            message: self.message.clone(),
            status: status_code.into(),
        };

        (status_code, axum::Json(error_response)).into_response()
    }
}
