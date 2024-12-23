use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use lib_api::db::util::commit_or_rollback;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::question::create_question_dto::{CreateQuestionDto, CreateQuestionResponse};
use lib_types::shared::question::QuestionType;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::helpers::verify_admin_or_user;
use crate::app::quiz::helpers::verify_quiz_exist;
use crate::db::question_repo::QuestionCreateProps;
use crate::db::quiz_repo::QuizUpdateProps;

#[debug_handler]
pub async fn create_question(
    Path(quiz_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<CreateQuestionDto>,
) -> Result<(StatusCode, Json<CreateQuestionResponse>), ApiError> {
    check_bad_form(dto.validate())?;
    let quiz = verify_quiz_exist(&context, quiz_id).await?;

    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    let props = QuestionCreateProps {
        quiz_id,
        text: dto.text,
        question_type: QuestionType::MultipleChoice,
    };

    let mut tx = context.repo.start_transaction().await?;

    let result = context
        .repo
        .question
        .create_question_tx(&mut tx, props)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create question: {}", e))
        })?;

    let mut new_order = quiz.questions_order.clone();
    new_order.push(result.to_string());

    context
        .repo
        .quiz
        .update_quiz_tx(
            &mut tx,
            quiz.id,
            QuizUpdateProps::questions_order(new_order),
        )
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create question: {}", e))
        })?;

    commit_or_rollback(tx, Ok(())).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateQuestionResponse { id: result }),
    ))
}
