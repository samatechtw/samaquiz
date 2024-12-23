use lib_api::error::api_error::ApiError;
use lib_types::entity::answer_entity::AnswerEntity;
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn verify_answer_exist(context: &ApiContext, id: Uuid) -> Result<AnswerEntity, ApiError> {
    let answer = context
        .repo
        .answer
        .get_answer_by_id(id)
        .await
        .map_err(not_found_or_internal)?;
    Ok(answer)
}
