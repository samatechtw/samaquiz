use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateQuestionDto {
    #[validate(length(min = 3, max = 1000))]
    pub text: String,
}

#[derive(Serialize)]
pub struct CreateQuestionResponse {
    pub id: Uuid,
}
