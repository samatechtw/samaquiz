use lib_api::error::api_error::ApiError;
use lib_types::entity::question_entity::{QuestionEntity, QuestionEntityRelations};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn verify_question_exist(
    context: &ApiContext,
    id: Uuid,
) -> Result<QuestionEntity, ApiError> {
    let question = context
        .repo
        .question
        .get_question_by_id(id)
        .await
        .map_err(not_found_or_internal)?;
    Ok(question)
}

pub async fn verify_question_exist_relations(
    context: &ApiContext,
    id: Uuid,
) -> Result<QuestionEntityRelations, ApiError> {
    let question = context
        .repo
        .question
        .get_question_relations_by_id(id, false)
        .await
        .map_err(not_found_or_internal)?;
    Ok(question)
}
