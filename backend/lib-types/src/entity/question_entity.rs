use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::question::QuestionType;

use super::answer_entity::AnswerEntity;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuestionEntity {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
    pub answers_order: Vec<String>,
    pub asset_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuestionEntityRelations {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
    pub answers: Vec<AnswerEntity>,
    pub answers_order: Vec<String>,
    pub asset_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct QuestionCreateResult {
    pub id: Uuid,
}
