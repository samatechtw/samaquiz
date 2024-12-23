use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::user::{UserStatus, UserType};

use super::user_view_model::UserViewModel;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ListUsersQuery {
    #[serde(default = "default_from")]
    #[validate(range(min = 1))]
    pub from: i32,
    #[serde(default = "default_to")]
    #[validate(range(min = 1))]
    pub to: i32,
    #[serde(rename = "type")]
    pub user_type: Option<UserType>,
    pub status: Option<UserStatus>,
}

fn default_from() -> i32 {
    1
}

fn default_to() -> i32 {
    10
}

#[derive(Serialize)]
pub struct ListUsersResponse {
    pub total: i64,
    pub results: Vec<UserViewModel>,
}
