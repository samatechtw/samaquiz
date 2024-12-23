use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;
use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::question::update_question_dto::UpdateQuestionDto;
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;

use crate::app::helpers::verify_admin_or_user;
use crate::app::quiz::helpers::verify_quiz_exist;
use crate::db::question_repo::QuestionUpdateProps;

use super::helpers::verify_question_exist_relations;

pub async fn update_question(
    Path((quiz_id, question_id)): Path<(Uuid, Uuid)>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<UpdateQuestionDto>,
) -> Result<(StatusCode, ()), ApiError> {
    check_bad_form(dto.validate())?;

    let quiz = verify_quiz_exist(&context, quiz_id).await?;
    // Get question to be updated
    let question = verify_question_exist_relations(&context, question_id).await?;

    if quiz_id != question.quiz_id {
        return Err(ApiError::not_found().message("Question not found in quiz"));
    }

    // Verify request
    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    if let Some(answers_order) = &dto.answers_order {
        if question.answers.len() != answers_order.len() {
            return Err(ApiError::bad_request().code(ApiErrorCode::OrderLength));
        }
        let a_ids: Vec<String> = question.answers.iter().map(|q| q.id.to_string()).collect();
        for new_id in answers_order.iter() {
            if !a_ids.iter().any(|id| id == new_id) {
                return Err(ApiError::bad_request().code(ApiErrorCode::OrderValue));
            }
        }
    }

    let props = QuestionUpdateProps {
        text: dto.text,
        question_type: None,
        answers_order: dto.answers_order,
    };

    // Update question
    context
        .repo
        .question
        .update_question(question_id, props)
        .await
        .map_err(|e| match e {
            _ => ApiError::internal_error().message(format!("Failed to update question: {}", e)),
        })?;

    // Return response
    Ok((StatusCode::OK, ()))
}
