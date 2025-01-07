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
        answer::helpers::verify_answer_exist, helpers::verify_admin_or_user,
        question::helpers::verify_question_exist_relations, quiz::helpers::verify_quiz_exist,
    },
    db::question_repo::QuestionUpdateProps,
};

pub async fn delete_answer(
    Path(answer_id): Path<Uuid>,
    Extension(request_user): Extension<RequestUser>,
    State(context): State<ApiContext>,
) -> Result<(), ApiError> {
    // Check if the answer exists in the database
    let answer = verify_answer_exist(&context, answer_id).await?;
    let question = verify_question_exist_relations(&context, answer.question_id).await?;
    let quiz = verify_quiz_exist(&context, question.quiz_id).await?;

    // Check if the requester is the owner of the answer or an admin
    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    let mut tx = context.repo.start_transaction().await?;

    // Remove the entry from the database
    context
        .repo
        .answer
        .delete_answer_by_id_tx(&mut tx, answer_id)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to delete answer: {}", e))
        })?;

    let new_order = question
        .answers_order
        .into_iter()
        .filter(|a| a != &answer_id.to_string())
        .collect();

    context
        .repo
        .question
        .update_question_tx(
            &mut tx,
            question.id,
            QuestionUpdateProps::answers_order(new_order),
        )
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to delete answer: {}", e))
        })?;

    commit_or_rollback(tx, Ok(())).await?;

    Ok(())
}
