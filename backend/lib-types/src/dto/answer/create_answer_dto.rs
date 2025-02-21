use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateAnswerDto {
    #[validate(length(min = 1, max = 200))]
    pub text: String,
    pub is_correct: bool,
    pub points: i32,
}

#[derive(Serialize)]
pub struct CreateAnswerResponse {
    pub id: Uuid,
}
