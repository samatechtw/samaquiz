use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use lib_api::error::api_error::ApiError;
use lib_types::shared::user::{RequestUser, UserType};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<StatusCode, ApiError> {
    // Verify user
    if request_user.user_type == UserType::User {
        if let Some(request_user_id) = request_user.user_id {
            if request_user_id != id {
                return Err(ApiError::forbidden());
            }
        } else {
            return Err(ApiError::internal_error().message("Request missing user_id"));
        }
    }

    context
        .repo
        .user
        .delete_user(id)
        .await
        .map_err(not_found_or_internal)?;

    Ok(StatusCode::NO_CONTENT)
}
