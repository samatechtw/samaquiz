use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    dto::{
        question::question_view_model::{self, QuestionViewModel},
        quiz_session::quiz_session_view_model::{self, QuizSessionViewModel},
    },
    entity::quiz_entity::QuizEntityRelations,
    shared::quiz::QuizType,
};

#[derive(Serialize)]
pub struct GetQuizResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub quiz_type: QuizType,
    pub questions: Vec<QuestionViewModel>,
    pub questions_order: Vec<String>,
    pub sessions: Vec<QuizSessionViewModel>,
    pub intro_background_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(quiz_entity: QuizEntityRelations) -> GetQuizResponse {
    return GetQuizResponse {
        id: quiz_entity.id,
        user_id: quiz_entity.user_id,
        title: quiz_entity.title,
        description: quiz_entity.description,
        quiz_type: quiz_entity.quiz_type,
        questions: quiz_entity
            .questions
            .into_iter()
            .map(question_view_model::to_api_response)
            .collect(),
        questions_order: quiz_entity.questions_order,
        sessions: quiz_entity
            .sessions
            .into_iter()
            .map(quiz_session_view_model::to_api_response)
            .collect(),
        intro_background_url: quiz_entity.intro_background_url,
        created_at: quiz_entity.created_at,
        updated_at: quiz_entity.updated_at,
    };
}
