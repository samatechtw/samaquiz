use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;
use lib_api::db::util::commit_or_rollback;
use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::answer::update_answer_dto::UpdateAnswerDto;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;

use crate::app::helpers::verify_admin_or_user;
use crate::app::question::helpers::verify_question_exist;
use crate::app::quiz::helpers::verify_quiz_exist;
use crate::db::answer_repo::AnswerUpdateProps;

use super::helpers::verify_answer_exist;

pub async fn update_answer(
    Path((quiz_id, question_id, answer_id)): Path<(Uuid, Uuid, Uuid)>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<UpdateAnswerDto>,
) -> Result<(StatusCode, ()), ApiError> {
    check_bad_form(dto.validate())?;

    let quiz = verify_quiz_exist(&context, quiz_id).await?;
    let question = verify_question_exist(&context, question_id).await?;
    // Get answer to be updated
    let answer = verify_answer_exist(&context, answer_id).await?;
    println!("ANSWR {:?}", answer);

    if question_id != answer.question_id {
        return Err(ApiError::not_found().message("Answer not found in question"));
    } else if question.quiz_id != quiz_id {
        return Err(ApiError::not_found().message("Question not found in quiz"));
    }

    // Verify request
    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    let props = AnswerUpdateProps {
        text: dto.text,
        points: dto.points,
    };

    let mut tx = context.repo.start_transaction().await?;

    // Update answer
    if props.points.is_some() || props.text.is_some() {
        context
            .repo
            .answer
            .update_answer_tx(&mut tx, answer_id, props)
            .await
            .map_err(|e| match e {
                _ => ApiError::internal_error().message(format!("Failed to update answer: {}", e)),
            })?;
    }

    if let Some(is_correct) = dto.is_correct {
        context
            .repo
            .answer
            .update_answer_correct_tx(&mut tx, question_id, answer_id, is_correct)
            .await
            .map_err(|e| match e {
                _ => ApiError::internal_error().message(format!("Failed to update answer: {}", e)),
            })?;
    }

    commit_or_rollback(tx, Ok(())).await?;

    // Return response
    Ok((StatusCode::OK, ()))
}
