use axum::{
    extract::{Path, State},
    Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::dto::question::get_question_dto::{to_api_response, GetQuestionResponse};
use uuid::Uuid;

use crate::api_context::ApiContext;

use super::helpers::verify_question_exist_relations;

pub async fn get_question(
    Path(question_id): Path<Uuid>,
    State(context): State<ApiContext>,
) -> Result<Json<GetQuestionResponse>, ApiError> {
    // Get question from DB
    let question = verify_question_exist_relations(&context, question_id).await?;

    Ok(Json(to_api_response(question)))
}
