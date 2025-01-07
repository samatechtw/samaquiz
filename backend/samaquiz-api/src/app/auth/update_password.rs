use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;

use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::auth::update_password_dto::UpdatePasswordDto;
use lib_types::entity::user_entity::UserUpdateParams;
use lib_types::shared::user::RequestUser;
use validator::Validate;

use crate::api_context::ApiContext;

pub async fn update_password(
    State(context): State<ApiContext>,
    Extension(user): Extension<RequestUser>,
    QJson(dto): QJson<UpdatePasswordDto>,
) -> Result<StatusCode, ApiError> {
    check_bad_form(dto.validate())?;
    let user_id = user
        .user_id
        .ok_or(ApiError::internal_error().message("Missing user ID"))?;

    context
        .repo
        .user
        .update_user(user_id, UserUpdateParams::password(dto.password))
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to update password: {}", e))
        })?;

    Ok(StatusCode::NO_CONTENT)
}
