use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::participant::create_participant_dto::{
    CreateParticipantDto, CreateParticipantResponse,
};
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::quiz_session::QuizSessionStatus;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::quiz::helpers::verify_quiz_exist;
use crate::app::quiz_session::helpers::verify_quiz_session_exist_relations;
use crate::app::websocket::ws_helpers::notify_add_participant;
use crate::db::participant_repo::ParticipantCreateProps;

#[debug_handler]
pub async fn create_participant(
    Path(session_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<CreateParticipantDto>,
) -> Result<(StatusCode, Json<CreateParticipantResponse>), ApiError> {
    check_bad_form(dto.validate())?;
    let session = verify_quiz_session_exist_relations(&context, session_id).await?;
    let _ = verify_quiz_exist(&context, session.quiz_id).await?;

    if session.status == QuizSessionStatus::Complete {
        return Err(ApiError::bad_request()
            .code(ApiErrorCode::QuizSessionComplete)
            .message("Cannot create participant"));
    } else if session.status == QuizSessionStatus::Canceled {
        return Err(ApiError::bad_request()
            .code(ApiErrorCode::QuizSessionCanceled)
            .message("Cannot create participant"));
    }

    let props = ParticipantCreateProps {
        session_id,
        user_id: request_user.user_id,
        name: dto.name,
        avatar: dto.avatar,
    };

    let result = context
        .repo
        .participant
        .create_participant(props)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create participant: {}", e))
        })?;

    let _ = notify_add_participant(&context, session_id.to_string(), result.count);

    Ok((StatusCode::CREATED, Json(result)))
}
