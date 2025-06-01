use crate::type_util::REGEX_UUID;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

use crate::dto::sort_direction::SortDirection;

use super::quiz_session_view_model::QuizSessionViewModel;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "snake_case")]
pub enum QuizSessionSortColumn {
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ListQuizSessionsQuery {
    #[serde(default = "default_from")]
    #[validate(range(min = 1))]
    pub from: i32,
    #[serde(default = "default_to")]
    #[validate(range(min = 1))]
    pub to: i32,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub user_id: Option<String>,
    pub column: Option<QuizSessionSortColumn>,
    pub direction: Option<SortDirection>,
}

fn default_from() -> i32 {
    1
}

fn default_to() -> i32 {
    20
}

#[derive(Serialize)]
pub struct ListQuizSessionsResponse {
    pub total: i64,
    pub results: Vec<QuizSessionViewModel>,
}
