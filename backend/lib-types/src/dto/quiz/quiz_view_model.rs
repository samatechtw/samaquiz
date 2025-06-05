use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::quiz_entity::QuizEntity,
    shared::{asset::AssetContentType, quiz::QuizType},
};

#[derive(Serialize)]
pub struct QuizViewModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub quiz_type: QuizType,
    pub questions_order: Vec<String>,
    pub intro_background_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(quiz_entity: QuizEntity) -> QuizViewModel {
    return QuizViewModel {
        id: quiz_entity.id,
        user_id: quiz_entity.user_id,
        title: quiz_entity.title,
        description: quiz_entity.description,
        quiz_type: quiz_entity.quiz_type,
        questions_order: quiz_entity.questions_order,
        intro_background_url: quiz_entity.intro_background_url,
        created_at: quiz_entity.created_at,
        updated_at: quiz_entity.updated_at,
    };
}

#[derive(Serialize)]
pub struct QuizAssetViewModelRelation {
    pub id: Uuid,
    pub content_type: AssetContentType,
    pub size: i64,
    pub quiz_id: Uuid,
}
