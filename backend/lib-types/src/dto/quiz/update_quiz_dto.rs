use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateQuizDto {
    #[validate(length(min = 3, max = 80))]
    pub title: Option<String>,
    #[validate(length(min = 10, max = 10000))]
    pub description: Option<String>,
    pub questions_order: Option<Vec<String>>,
}
