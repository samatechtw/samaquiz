use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{question_entity::QuestionEntity, quiz_session_entity::QuizSessionEntity};
use crate::shared::quiz::QuizType;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizEntity {
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

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizEntityRelations {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub quiz_type: QuizType,
    pub questions: Vec<QuestionEntity>,
    pub questions_order: Vec<String>,
    pub intro_background_url: String,
    pub sessions: Vec<QuizSessionEntity>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct QuizCreateResult {
    pub id: Uuid,
}

pub struct QuizUpdateParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub quiz_type: Option<QuizType>,
    pub questions_order: Option<Vec<String>>,
    pub intro_background_url: Option<String>,
}

impl QuizUpdateParams {
    pub fn questions_order(order: Vec<String>) -> Self {
        Self {
            title: None,
            description: None,
            quiz_type: None,
            questions_order: Some(order),
            intro_background_url: None,
        }
    }
}

#[derive(Debug)]
pub struct QuizListResults {
    pub total: i64,
    pub results: Vec<QuizEntity>,
}
