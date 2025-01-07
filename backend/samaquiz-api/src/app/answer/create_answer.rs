use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use lib_api::db::util::commit_or_rollback;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::answer::create_answer_dto::{CreateAnswerDto, CreateAnswerResponse};
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::helpers::verify_admin_or_user;
use crate::app::question::helpers::verify_question_exist_relations;
use crate::app::quiz::helpers::verify_quiz_exist;
use crate::db::answer_repo::AnswerCreateProps;
use crate::db::question_repo::QuestionUpdateProps;

#[debug_handler]
pub async fn create_answer(
    Path(question_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<CreateAnswerDto>,
) -> Result<(StatusCode, Json<CreateAnswerResponse>), ApiError> {
    check_bad_form(dto.validate())?;
    let question = verify_question_exist_relations(&context, question_id).await?;
    let quiz = verify_quiz_exist(&context, question.quiz_id).await?;

    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    let props = AnswerCreateProps {
        question_id,
        text: dto.text,
        points: dto.points,
    };

    let mut tx = context.repo.start_transaction().await?;

    let result = context
        .repo
        .answer
        .create_answer_tx(&mut tx, props)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create answer: {}", e))
        })?;

    let mut new_order = question.answers_order.clone();
    new_order.push(result.to_string());

    context
        .repo
        .question
        .update_question_tx(
            &mut tx,
            question_id,
            QuestionUpdateProps::answers_order(new_order),
        )
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create answer: {}", e))
        })?;

    if dto.is_correct {
        context
            .repo
            .answer
            .update_answer_correct_tx(&mut tx, question_id, result, dto.is_correct)
            .await
            .map_err(|e| match e {
                _ => ApiError::internal_error().message(format!("Failed to update answer: {}", e)),
            })?;
    }

    commit_or_rollback(tx, Ok(())).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateAnswerResponse { id: result }),
    ))
}
