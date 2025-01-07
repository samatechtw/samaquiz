use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::quiz_session::get_quiz_session_leaders::GetQuizSessionLeadersResponse,
    shared::user::RequestUser,
};
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    app::helpers::{not_found_or_internal, verify_admin_or_user},
};

use super::helpers::verify_quiz_session_exist;

pub async fn get_quiz_session_leaders(
    Path(session_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<GetQuizSessionLeadersResponse>, ApiError> {
    let session = verify_quiz_session_exist(&context, session_id).await?;
    verify_admin_or_user(&request_user, session.user_id.to_string())?;

    // Get participant count from DB
    let response = context
        .repo
        .participant
        .get_session_leaders(session_id)
        .await
        .map_err(not_found_or_internal)?;

    Ok(Json(response))
}
