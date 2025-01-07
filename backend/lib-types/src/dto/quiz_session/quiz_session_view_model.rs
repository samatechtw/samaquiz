use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::quiz_session_entity::QuizSessionEntity, shared::quiz_session::QuizSessionStatus,
};

#[derive(Serialize)]
pub struct QuizSessionViewModel {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub status: QuizSessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(entity: QuizSessionEntity) -> QuizSessionViewModel {
    return QuizSessionViewModel {
        id: entity.id,
        quiz_id: entity.quiz_id,
        user_id: entity.user_id,
        code: entity.code,
        start_time: entity.start_time,
        end_time: entity.end_time,
        status: entity.status,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    };
}
