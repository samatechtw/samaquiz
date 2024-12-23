use crate::{shared::quiz::QuizType, type_util::REGEX_UUID};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

use crate::dto::sort_direction::SortDirection;

use super::quiz_view_model::QuizViewModel;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "snake_case")]
pub enum QuizSortColumn {
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ListQuizzesQuery {
    #[serde(default = "default_from")]
    #[validate(range(min = 1))]
    pub from: i32,
    #[serde(default = "default_to")]
    #[validate(range(min = 1))]
    pub to: i32,
    pub types: Option<Vec<QuizType>>,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub user_id: Option<String>,
    pub column: Option<QuizSortColumn>,
    pub direction: Option<SortDirection>,
}

fn default_from() -> i32 {
    1
}

fn default_to() -> i32 {
    20
}

#[derive(Serialize)]
pub struct ListQuizzesResponse {
    pub total: i64,
    pub results: Vec<QuizViewModel>,
}
