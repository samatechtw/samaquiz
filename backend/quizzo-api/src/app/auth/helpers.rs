use lib_api::{auth::generate_jwt::generate_confirm_token, error::api_error::ApiError};
use tracing::{error, info};
use uuid::Uuid;

use crate::api_context::ApiContext;

pub async fn send_welcome_email(
    context: &ApiContext,
    user_id: Uuid,
    email: String,
) -> Result<(), ApiError> {
    let secret = context.config.confirm_shared_secret.clone();
    let token = generate_confirm_token(user_id, 1440, secret).map_err(|e| {
        ApiError::internal_error().message(format!("Confirm token fail: {}", e.to_string()))
    })?;
    /*
    match send_welcome(
        &context.config.sendgrid_api_key,
        &email,
        &context.config.app_web_url,
        &token,
    )
    .await
    {
        Ok(_) => info!("Sent welcome mail to: {}", email),
        Err(e) => error!(err = e.to_string(), "Failed to send welcome mail"),
    }
    */
    Ok(())
}

pub async fn resend_confirm_email(
    context: &ApiContext,
    user_id: Uuid,
    email: &str,
) -> Result<(), ApiError> {
    let secret = context.config.confirm_shared_secret.clone();
    let token = generate_confirm_token(user_id, 1440, secret).map_err(|e| {
        ApiError::internal_error().message(format!("Confirm token fail: {}", e.to_string()))
    })?;
    /*
    match send_welcome(
        &context.config.sendgrid_api_key,
        email,
        &context.config.app_web_url,
        &token,
    )
    .await
    {
        Ok(_) => info!("Resent confirm mail to: {}", email),
        Err(e) => error!(err = e.to_string(), "Failed to resend confirm mail"),
    }
    */
    Ok(())
}
