use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateQuizDto {
    #[validate(length(min = 3, max = 80))]
    pub title: String,
    #[validate(length(min = 10, max = 10000))]
    pub description: String,
}

#[derive(Serialize)]
pub struct CreateQuizResponse {
    pub id: Uuid,
}
