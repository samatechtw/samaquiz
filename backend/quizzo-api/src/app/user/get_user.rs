use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::user::get_user_dto::{to_api_response, GetUserResponse},
    shared::user::{RequestUser, UserType},
};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn get_user(
    Path(id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<GetUserResponse>, ApiError> {
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

    let user = context
        .repo
        .user
        .get_user_by_id(id)
        .await
        .map_err(not_found_or_internal)?;

    Ok(Json(to_api_response(user)))
}
