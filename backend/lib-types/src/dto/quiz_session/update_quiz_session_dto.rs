use serde::Deserialize;
use validator::Validate;

use crate::shared::quiz_session::QuizSessionStatus;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateQuizSessionDto {
    #[validate(length(min = 1, max = 12))]
    pub code: Option<String>,
    #[validate(length(min = 2, max = 20))]
    pub host_name: Option<String>,
    #[validate(length(max = 100))]
    pub host_avatar: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub question_index: Option<i64>,
    pub question_end_time: Option<i64>,
    pub question_duration: Option<i64>,
    pub status: Option<QuizSessionStatus>,
}
