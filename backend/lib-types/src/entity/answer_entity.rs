use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct AnswerEntity {
    pub id: Uuid,
    pub question_id: Uuid,
    pub text: String,
    pub is_correct: bool,
    pub points: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct AnswerCreateResult {
    pub id: Uuid,
}

pub struct AnswerUpdateParams {
    pub text: Option<String>,
    pub is_correct: Option<String>,
}
