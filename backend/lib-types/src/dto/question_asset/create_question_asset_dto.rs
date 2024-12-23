use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    entity::question_asset_entity::QuestionAssetEntity,
    shared::asset::{AssetContentType, AssetState},
    type_util::REGEX_UUID,
};

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateQuestionAssetDto {
    #[validate(range(min = 0, max = 20000000))]
    pub content_size: i64,
    pub content_type: AssetContentType,
    #[validate(regex(path = "*REGEX_UUID"))]
    pub question_id: String,
}

#[derive(Serialize)]
pub struct CreateQuestionAssetResponse {
    pub id: Uuid,
    pub signed_url: String,
    pub size: i64,
    pub content_type: String,
    pub state: AssetState,
    pub user_id: Uuid,
    pub question_id: Uuid,
    pub upload_expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(
    entity: QuestionAssetEntity,
    signed_url: String,
) -> CreateQuestionAssetResponse {
    return CreateQuestionAssetResponse {
        id: entity.id,
        signed_url,
        size: entity.size,
        content_type: entity.content_type.to_string(),
        state: entity.state,
        user_id: entity.user_id,
        question_id: entity.question_id,
        upload_expires_at: entity.upload_expires_at,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    };
}
