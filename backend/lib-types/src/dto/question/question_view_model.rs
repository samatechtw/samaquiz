use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{entity::question_entity::QuestionEntity, shared::question::QuestionType};

#[derive(Serialize)]
pub struct QuestionViewModel {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
    pub answers_order: Vec<String>,
    pub asset_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(question_entity: QuestionEntity) -> QuestionViewModel {
    return QuestionViewModel {
        id: question_entity.id,
        quiz_id: question_entity.quiz_id,
        text: question_entity.text,
        question_type: question_entity.question_type,
        answers_order: question_entity.answers_order,
        asset_url: question_entity.asset_url,
        created_at: question_entity.created_at,
        updated_at: question_entity.updated_at,
    };
}
