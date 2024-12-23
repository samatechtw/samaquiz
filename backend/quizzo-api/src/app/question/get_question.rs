use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::question::get_question_dto::{to_api_response, GetQuestionResponse},
    shared::user::RequestUser,
};
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    app::{helpers::verify_admin_or_user, quiz::helpers::verify_quiz_exist},
};

use super::helpers::verify_question_exist_relations;

pub async fn get_question(
    Path((quiz_id, question_id)): Path<(Uuid, Uuid)>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<GetQuestionResponse>, ApiError> {
    // Get question from DB
    let question = verify_question_exist_relations(&context, question_id).await?;
    let quiz = verify_quiz_exist(&context, question.quiz_id).await?;

    if quiz_id != question.quiz_id {
        return Err(ApiError::not_found().message("Question not found in quiz"));
    }

    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    Ok(Json(to_api_response(question)))
}
