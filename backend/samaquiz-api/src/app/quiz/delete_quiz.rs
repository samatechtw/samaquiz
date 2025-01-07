use axum::{
    extract::{Path, State},
    Extension,
};

use lib_api::error::api_error::ApiError;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    app::{helpers::verify_admin_or_user, quiz::helpers::verify_quiz_exist},
};

pub async fn delete_quiz(
    Path(quiz_id): Path<Uuid>,
    Extension(request_user): Extension<RequestUser>,
    State(context): State<ApiContext>,
) -> Result<(), ApiError> {
    // Check if the quiz exists in the database
    let quiz = verify_quiz_exist(&context, quiz_id).await?;

    // Check if the requester is the owner of the quiz or an admin
    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    // Remove the entry from the database
    context
        .repo
        .quiz
        .delete_quiz_by_id(quiz_id)
        .await
        .map_err(|e| ApiError::internal_error().message(format!("Failed to delete quiz: {}", e)))?;

    Ok(())
}
