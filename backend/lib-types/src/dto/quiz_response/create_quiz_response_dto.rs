use crate::type_util::REGEX_UUID;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateQuizResponseDto {
    #[validate(regex(path = "*REGEX_UUID"))]
    pub participant_id: String,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub question_id: String,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub answer_id: String,
}

#[derive(Serialize)]
pub struct CreateQuizResponseResponse {
    pub id: Uuid,
    pub is_correct: bool,
    pub question_response_count: i64,
}
