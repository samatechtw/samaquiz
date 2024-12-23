use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::quiz::quiz_view_model::QuizAssetViewModelRelation,
    shared::{asset::AssetContentType, quiz::QuizType},
};

use super::question_entity::QuestionEntity;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub quiz_type: QuizType,
    pub questions_order: Vec<String>,
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
    pub asset: Option<QuizAssetEntityRelation>,
    pub questions: Vec<QuestionEntity>,
    pub questions_order: Vec<String>,
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
}

impl QuizUpdateParams {
    pub fn questions_order(order: Vec<String>) -> Self {
        Self {
            title: None,
            description: None,
            quiz_type: None,
            questions_order: Some(order),
        }
    }
}

#[derive(Debug)]
pub struct QuizListResults {
    pub total: i64,
    pub results: Vec<QuizEntity>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizAssetEntityRelation {
    pub id: Uuid,
    pub size: i64,
    pub content_type: AssetContentType,
    pub quiz_id: Uuid,
}

impl QuizAssetEntityRelation {
    pub fn to_api_response(&self) -> QuizAssetViewModelRelation {
        return QuizAssetViewModelRelation {
            id: self.id,
            size: self.size,
            content_type: self.content_type,
            quiz_id: self.quiz_id,
        };
    }
}
