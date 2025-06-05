use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub status: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApiErrorCode {
    AssetUsageExceeded,
    AlreadyConfirmed,
    ConfirmExpired,
    InvalidAuth,
    InvalidFormData,
    InvalidOldPassword,
    InvalidNumber,
    InvalidStatus,
    InvalidQuestion,
    InvalidAnswer,
    OrderLength,
    OrderValue,
    NoQuestions,
    QuestionOver,
    QuizSessionCode,
    QuizSessionComplete,
    QuizSessionCanceled,
    RestrictedStatus,
    UserExists,
    NoUpdates,
    Unauthorized,
    None,
}

impl std::fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for ApiErrorCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "InvalidAuth" => ApiErrorCode::InvalidAuth,
            "InvalidFormData" => ApiErrorCode::InvalidFormData,
            "InvalidOldPassword" => ApiErrorCode::InvalidOldPassword,
            "UserExists" => ApiErrorCode::UserExists,
            "NoUpdates" => ApiErrorCode::NoUpdates,
            _ => ApiErrorCode::None,
        })
    }
}
