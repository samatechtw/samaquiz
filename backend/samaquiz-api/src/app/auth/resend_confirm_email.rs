use axum::extract::State;
use axum::Extension;
use lib_types::shared::api_error::ApiErrorCode;

use super::helpers;
use crate::api_context::ApiContext;
use lib_api::error::api_error::ApiError;
use lib_types::shared::user::RequestUser;

pub async fn resend_confirm_email(
    State(context): State<ApiContext>,
    Extension(user): Extension<RequestUser>,
) -> Result<(), ApiError> {
    let user_id = user
        .user_id
        .ok_or(ApiError::internal_error().message("Missing user ID"))?;

    // Get User
    let user = context
        .repo
        .user
        .get_user_by_id(user_id)
        .await
        .map_err(|_| {
            ApiError::not_found().message(format!("User with ID {} not found", user_id))
        })?;

    if user.email_confirmed {
        return Err(ApiError::bad_request()
            .code(ApiErrorCode::AlreadyConfirmed)
            .message("Resend fail"));
    }

    helpers::resend_confirm_email(&context, user_id, &user.email).await?;

    Ok(())
}
