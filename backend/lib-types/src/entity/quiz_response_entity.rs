use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizResponseEntity {
    pub id: Uuid,
    pub participant_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Uuid,
    pub is_correct: bool,
    pub created_at: DateTime<Utc>,
}

pub struct QuizResponseCreateResult {
    pub id: Uuid,
}
