use serde::Deserialize;
use validator::Validate;

use crate::shared::asset::AssetState;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateQuizAssetDto {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub state: Option<AssetState>,
}
