use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::quiz_session::QuizSessionStatus;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizSessionEntity {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub question_end_time: Option<i64>,
    pub question_index: Option<i64>,
    pub question_duration: i64,
    pub status: QuizSessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizSessionEntityRelations {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub host_name: String,
    pub host_avatar: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub question_end_time: Option<i64>,
    pub question_index: Option<i64>,
    pub question_duration: i64,
    pub status: QuizSessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct QuizSessionCreateResult {
    pub id: Uuid,
}
