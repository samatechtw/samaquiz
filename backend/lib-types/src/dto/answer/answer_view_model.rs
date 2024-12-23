use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::entity::answer_entity::AnswerEntity;

#[derive(Serialize)]
pub struct AnswerViewModel {
    pub id: Uuid,
    pub question_id: Uuid,
    pub text: String,
    pub is_correct: bool,
    pub points: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(entity: AnswerEntity) -> AnswerViewModel {
    return AnswerViewModel {
        id: entity.id,
        question_id: entity.question_id,
        text: entity.text,
        is_correct: entity.is_correct,
        points: entity.points,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    };
}
