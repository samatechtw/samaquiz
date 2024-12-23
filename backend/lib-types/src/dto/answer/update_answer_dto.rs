use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateAnswerDto {
    #[validate(length(min = 3, max = 1000))]
    pub text: Option<String>,
    pub is_correct: Option<bool>,
    pub points: Option<i32>,
}
