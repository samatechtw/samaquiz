use axum::extract::State;

use lib_api::auth::verify_jwt::verify_confirm_token;
use lib_api::db::db_error::DbError;
use lib_api::error::api_error::ApiError;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::auth::confirm_email_dto::ConfirmEmailDto;
use lib_types::entity::user_entity::UserUpdateParams;

use crate::api_context::ApiContext;

pub async fn confirm_email(
    State(context): State<ApiContext>,
    QJson(dto): QJson<ConfirmEmailDto>,
) -> Result<(), ApiError> {
    let secret = &context.config.confirm_shared_secret;

    let user_id = verify_confirm_token(secret, &dto.code)?;

    context
        .repo
        .user
        .update_user(user_id, UserUpdateParams::email_confirmed(true))
        .await
        .map_err(|e| match e {
            DbError::EntityNotFound() => ApiError::bad_request().message("User not found"),
            _ => ApiError::internal_error().message(format!(
                "Failed to update email confirmed: {}",
                e.to_string()
            )),
        })?;

    Ok(())
}
