use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    dto::answer::answer_view_model::{self, AnswerViewModel},
    entity::question_entity::QuestionEntityRelations,
    shared::question::QuestionType,
};

use super::question_view_model::QuestionAssetViewModelRelation;

#[derive(Serialize)]
pub struct GetQuestionResponse {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
    pub answers_order: Vec<String>,
    pub answers: Vec<AnswerViewModel>,
    pub asset: Option<QuestionAssetViewModelRelation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(question_entity: QuestionEntityRelations) -> GetQuestionResponse {
    return GetQuestionResponse {
        id: question_entity.id,
        quiz_id: question_entity.quiz_id,
        text: question_entity.text,
        question_type: question_entity.question_type,
        answers: question_entity
            .answers
            .into_iter()
            .map(answer_view_model::to_api_response)
            .collect(),
        answers_order: question_entity.answers_order,
        asset: question_entity.asset.map(|a| a.to_api_response()),
        created_at: question_entity.created_at,
        updated_at: question_entity.updated_at,
    };
}
