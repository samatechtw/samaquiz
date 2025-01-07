use axum::{
    extract::{Path, State},
    Extension,
};

use lib_api::{db::util::commit_or_rollback, error::api_error::ApiError};
use lib_types::shared::user::RequestUser;
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    app::{
        helpers::verify_admin_or_user, question::helpers::verify_question_exist,
        quiz::helpers::verify_quiz_exist,
    },
    db::quiz_repo::QuizUpdateProps,
};

pub async fn delete_question(
    Path(question_id): Path<Uuid>,
    Extension(request_user): Extension<RequestUser>,
    State(context): State<ApiContext>,
) -> Result<(), ApiError> {
    // Check if the question exists in the database
    let question = verify_question_exist(&context, question_id).await?;
    let quiz = verify_quiz_exist(&context, question.quiz_id).await?;

    // Check if the requester is the owner of the question or an admin
    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    let mut tx = context.repo.start_transaction().await?;

    // Remove the entry from the database
    context
        .repo
        .question
        .delete_question_by_id_tx(&mut tx, question_id)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to delete question: {}", e))
        })?;

    let new_order = quiz
        .questions_order
        .into_iter()
        .filter(|a| a != &question_id.to_string())
        .collect();

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
            ApiError::internal_error().message(format!("Failed to delete question: {}", e))
        })?;

    commit_or_rollback(tx, Ok(())).await?;

    Ok(())
}
