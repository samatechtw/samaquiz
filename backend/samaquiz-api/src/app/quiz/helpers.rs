use lib_api::error::api_error::ApiError;
use lib_types::entity::quiz_entity::{QuizEntity, QuizEntityRelations};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn verify_quiz_exist(context: &ApiContext, id: Uuid) -> Result<QuizEntity, ApiError> {
    let quiz = context
        .repo
        .quiz
        .get_quiz_by_id(id)
        .await
        .map_err(not_found_or_internal)?;
    Ok(quiz)
}

pub async fn verify_quiz_exist_relations(
    context: &ApiContext,
    id: Uuid,
) -> Result<QuizEntityRelations, ApiError> {
    let quiz = context
        .repo
        .quiz
        .get_quiz_relations_by_id(id)
        .await
        .map_err(not_found_or_internal)?;
    Ok(quiz)
}
