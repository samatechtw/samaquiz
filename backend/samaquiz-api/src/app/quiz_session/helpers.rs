use lib_api::error::api_error::ApiError;
use lib_types::{
    entity::quiz_session_entity::{QuizSessionEntity, QuizSessionEntityRelations},
    shared::api_error::ApiErrorCode,
};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn verify_quiz_session_unique(
    context: &ApiContext,
    code: String,
) -> Result<(), ApiError> {
    let quiz = context
        .repo
        .quiz_session
        .get_quiz_session_by_code(code)
        .await;
    if quiz.is_ok() {
        return Err(ApiError::bad_request().code(ApiErrorCode::QuizSessionCode));
    }
    Ok(())
}

pub async fn verify_quiz_session_exist(
    context: &ApiContext,
    id: Uuid,
) -> Result<QuizSessionEntity, ApiError> {
    let quiz_session = context
        .repo
        .quiz_session
        .get_quiz_session_by_id(id)
        .await
        .map_err(not_found_or_internal)?;
    Ok(quiz_session)
}

pub async fn verify_quiz_session_exist_relations(
    context: &ApiContext,
    id: Uuid,
) -> Result<QuizSessionEntityRelations, ApiError> {
    let quiz_session = context
        .repo
        .quiz_session
        .get_quiz_session_relations_by_id(id)
        .await
        .map_err(not_found_or_internal)?;
    Ok(quiz_session)
}
