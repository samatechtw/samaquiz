use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

use crate::{
    dto::sort_direction::SortDirection,
    shared::asset::{AssetContentType, AssetState},
    type_util::REGEX_UUID,
};

use super::quiz_asset_viewmodel::QuizAssetViewModel;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "snake_case")]
pub enum QuizAssetSortColumn {
    Size,
    CreatedAt,
    UpdatedAt,
}

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ListQuizAssetsQuery {
    #[validate(regex(path = "*REGEX_UUID"))]
    pub user_id: Option<String>,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub quiz_id: Option<String>,
    pub content_type: Option<AssetContentType>,
    pub state: Option<AssetState>,
    #[serde(default = "default_from")]
    #[validate(range(min = 1))]
    pub from: i32,
    #[serde(default = "default_to")]
    #[validate(range(min = 1))]
    pub to: i32,
    pub column: Option<QuizAssetSortColumn>,
    pub direction: Option<SortDirection>,
}

impl ListQuizAssetsQuery {
    pub fn new() -> ListQuizAssetsQuery {
        Self {
            user_id: None,
            quiz_id: None,
            content_type: None,
            state: None,
            column: None,
            from: default_from(),
            to: default_to(),
            direction: None,
        }
    }
    pub fn quiz_id(mut self, quiz_id: String) -> ListQuizAssetsQuery {
        self.quiz_id = Some(quiz_id);
        self
    }
}

fn default_from() -> i32 {
    1
}

fn default_to() -> i32 {
    25
}

#[derive(Serialize)]
pub struct ListQuizAssetsResponse {
    pub total: i64,
    pub total_usage: u64,
    pub results: Vec<QuizAssetViewModel>,
}
