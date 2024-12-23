use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::quiz_asset_entity::QuizAssetEntity,
    shared::{asset::AssetState, js_date::JsDate},
};

#[derive(Serialize)]
pub struct QuizAssetViewModel {
    pub id: Uuid,
    pub size: i64,
    pub content_type: String,
    pub state: AssetState,
    pub user_id: Uuid,
    pub quiz_id: Uuid,
    pub upload_expires_at: JsDate,
    pub created_at: JsDate,
    pub updated_at: JsDate,
}

pub fn to_api_response(entity: QuizAssetEntity) -> QuizAssetViewModel {
    return QuizAssetViewModel {
        id: entity.id,
        size: entity.size,
        content_type: entity.content_type.to_string(),
        state: entity.state,
        user_id: entity.user_id,
        quiz_id: entity.quiz_id,
        upload_expires_at: JsDate {
            timestamp: entity.upload_expires_at,
        },
        created_at: JsDate {
            timestamp: entity.created_at,
        },
        updated_at: JsDate {
            timestamp: entity.updated_at,
        },
    };
}
