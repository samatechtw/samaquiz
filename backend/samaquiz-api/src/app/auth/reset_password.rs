use axum::extract::State;
use axum::http::StatusCode;

use lib_api::error::api_error::ApiError;
use lib_api::{auth::generate_jwt::generate_jwt, db::db_error::DbError};

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::auth::reset_password_dto::ResetPasswordDto;
use tracing::{error, info};
use validator::Validate;

use crate::api_context::ApiContext;

pub async fn reset_password(
    State(context): State<ApiContext>,
    QJson(dto): QJson<ResetPasswordDto>,
) -> Result<StatusCode, ApiError> {
    let email = dto.email.clone();
    check_bad_form(dto.validate())?;

    let user_result = context.repo.user.find_user_by_email(dto.email).await;

    let user = match user_result {
        Ok(user) => Some(user),
        Err(DbError::EntityNotFound()) => None,
        Err(e) => return Err(ApiError::internal_error().message(format!("Internal Error: {}", e))),
    };

    if let Some(user) = user {
        let user_token = generate_jwt(
            user.id,
            user.user_type,
            1440,
            &context.config.app_auth_secret,
        )?;

        let reset_token = match user_token.token {
            Some(token) => token,
            None => String::new(),
        };

        /*
        match send_password_reset::password_reset(
            &email,
            &context.config.sendgrid_api_key,
            &context.config.app_web_url,
            &reset_token,
        )
        .await
        {
            Ok(_) => info!("Sent reset password mail to: {}", email),
            Err(e) => error!(err = e.to_string(), "Failed to send mail"),
        }
        */
    }

    Ok(StatusCode::ACCEPTED)
}
