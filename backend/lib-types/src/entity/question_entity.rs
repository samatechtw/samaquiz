use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::question::question_view_model::QuestionAssetViewModelRelation,
    shared::{asset::AssetContentType, question::QuestionType},
};

use super::answer_entity::AnswerEntity;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuestionEntity {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
    pub answers_order: Vec<String>,
    pub asset: Option<QuestionAssetEntityRelation>,
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
    pub asset: Option<QuestionAssetEntityRelation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct QuestionCreateResult {
    pub id: Uuid,
}

pub struct QuestionUpdateParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub question_type: Option<QuestionType>,
    pub questions_order: Option<Vec<String>>,
}

impl QuestionUpdateParams {
    pub fn answers_order(order: Vec<String>) -> Self {
        Self {
            title: None,
            description: None,
            question_type: None,
            questions_order: Some(order),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuestionAssetEntityRelation {
    pub id: Uuid,
    pub size: i64,
    pub content_type: AssetContentType,
    pub question_id: Uuid,
}

impl QuestionAssetEntityRelation {
    pub fn to_api_response(&self) -> QuestionAssetViewModelRelation {
        return QuestionAssetViewModelRelation {
            id: self.id,
            size: self.size,
            content_type: self.content_type,
            question_id: self.question_id,
        };
    }
}
