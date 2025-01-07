use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::type_util::REGEX_CODE;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateQuizSessionDto {
    #[validate(regex(path = "*REGEX_CODE"))]
    pub code: String,
    #[validate(length(min = 2, max = 20))]
    pub host_name: String,
    #[validate(length(max = 100))]
    pub host_avatar: Option<String>,
}

#[derive(Serialize)]
pub struct CreateQuizSessionResponse {
    pub id: Uuid,
}
