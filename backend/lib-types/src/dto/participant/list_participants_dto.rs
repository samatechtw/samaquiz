use crate::type_util::REGEX_UUID;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

use crate::dto::sort_direction::SortDirection;

use super::participant_view_model::ParticipantViewModel;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantSortColumn {
    CreatedAt,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ListParticipantsQuery {
    #[serde(default = "default_from")]
    #[validate(range(min = 1))]
    pub from: i32,
    #[serde(default = "default_to")]
    #[validate(range(min = 1))]
    pub to: i32,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub user_id: Option<String>,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub session_id: Option<String>,
    pub column: Option<ParticipantSortColumn>,
    pub direction: Option<SortDirection>,
}

impl ListParticipantsQuery {
    pub fn session(id: String) -> ListParticipantsQuery {
        Self {
            from: 1,
            to: 1000,
            user_id: None,
            session_id: Some(id),
            column: Some(ParticipantSortColumn::CreatedAt),
            direction: Some(SortDirection::Desc),
        }
    }
}

fn default_from() -> i32 {
    1
}

fn default_to() -> i32 {
    1000
}

#[derive(Serialize)]
pub struct ListParticipantsResponse {
    pub total: i64,
    pub results: Vec<ParticipantViewModel>,
}
