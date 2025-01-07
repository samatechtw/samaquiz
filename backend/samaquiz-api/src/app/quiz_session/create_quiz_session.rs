use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::quiz_session::create_quiz_session_dto::{
    CreateQuizSessionDto, CreateQuizSessionResponse,
};
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::helpers::verify_admin_or_user;
use crate::app::quiz::helpers::verify_quiz_exist;
use crate::db::quiz_session_repo::QuizSessionCreateProps;

use super::helpers::verify_quiz_session_unique;

#[debug_handler]
pub async fn create_quiz_session(
    Path(quiz_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<CreateQuizSessionDto>,
) -> Result<(StatusCode, Json<CreateQuizSessionResponse>), ApiError> {
    check_bad_form(dto.validate())?;
    verify_quiz_session_unique(&context, dto.code.clone()).await?;
    let quiz = verify_quiz_exist(&context, quiz_id).await?;

    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    let props = QuizSessionCreateProps {
        quiz_id,
        user_id: quiz.user_id,
        code: dto.code.to_lowercase(),
        host_name: dto.host_name,
        host_avatar: dto.host_avatar,
    };

    let result = context
        .repo
        .quiz_session
        .create_quiz_session(props)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create quiz_session: {}", e))
        })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateQuizSessionResponse { id: result }),
    ))
}
