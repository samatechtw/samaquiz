use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateQuestionDto {
    #[validate(length(min = 3, max = 1000))]
    pub text: Option<String>,
    pub answers_order: Option<Vec<String>>,
}
