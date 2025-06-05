use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    dto::question::question_view_model::{self, QuestionViewModel},
    entity::{
        participant_entity::ParticipantEntity, quiz_entity::QuizEntityRelations,
        quiz_session_entity::QuizSessionEntityRelations,
    },
    shared::{quiz::QuizType, quiz_session::QuizSessionStatus},
};

#[derive(Serialize)]
pub struct GetQuizSessionResponse {
    pub id: Uuid,
    pub quiz: GetSessionQuizRelation,
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
    pub participants: Option<Vec<GetSessionParticipantRelation>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct GetSessionQuizRelation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub quiz_type: QuizType,
    pub questions: Vec<QuestionViewModel>,
    pub questions_order: Vec<String>,
    pub intro_background_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct GetSessionParticipantRelation {
    pub id: Uuid,
    pub name: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
}

pub fn to_api_response(
    entity: QuizSessionEntityRelations,
    quiz_entity: QuizEntityRelations,
    participants: Option<Vec<ParticipantEntity>>,
) -> GetQuizSessionResponse {
    let parts = participants.map(|ps| {
        ps.into_iter()
            .map(|p| GetSessionParticipantRelation {
                id: p.id,
                name: p.name,
                avatar: p.avatar,
                created_at: p.created_at,
            })
            .collect::<Vec<GetSessionParticipantRelation>>()
    });
    return GetQuizSessionResponse {
        id: entity.id,
        quiz: GetSessionQuizRelation {
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
            intro_background_url: quiz_entity.intro_background_url,
            created_at: quiz_entity.created_at,
            updated_at: quiz_entity.updated_at,
        },
        user_id: entity.user_id,
        code: entity.code,
        host_name: entity.host_name,
        host_avatar: entity.host_avatar,
        start_time: entity.start_time,
        end_time: entity.end_time,
        question_end_time: entity.question_end_time,
        question_index: entity.question_index,
        question_duration: entity.question_duration,
        status: entity.status,
        participants: parts,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    };
}
