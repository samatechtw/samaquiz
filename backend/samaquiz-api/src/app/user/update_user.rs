use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use lib_api::db::password::verify;
use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::user::update_user_dto::UpdateUserDto;
use lib_types::dto::user::user_view_model::{to_api_response, UserViewModel};
use lib_types::entity::user_entity::UserUpdateParams;
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::user::{RequestUser, UserType};
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;

use crate::app::auth::helpers::resend_confirm_email;
use crate::app::helpers::verify_admin_or_user;

pub async fn update_user(
    Path(user_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<UpdateUserDto>,
) -> Result<(StatusCode, Json<UserViewModel>), ApiError> {
    check_bad_form(dto.validate())?;
    let has_new_email = dto.email.is_some();

    // Get user to be updated
    let user_to_be_updated = context
        .repo
        .user
        .get_user_by_id(user_id)
        .await
        .map_err(|_| {
            ApiError::not_found().message(format!("User with ID {} not found", user_id))
        })?;

    // Verify user type
    verify_admin_or_user(&request_user, user_to_be_updated.id.to_string())?;
    if request_user.user_type == UserType::User {
        // User are not allowed to update user_type and user_status
        if dto.user_type.is_some() || dto.user_status.is_some() {
            return Err(ApiError::bad_request().message("Unauthorized field update"));
        }
    }

    // Verify request
    // Verify new email is unique
    if let Some(new_email) = &dto.email {
        if context
            .repo
            .user
            .find_user_by_email(new_email.to_string())
            .await
            .is_ok()
        {
            return Err(ApiError::bad_request()
                .code(ApiErrorCode::UserExists)
                .message(format!("Email {} already exists", new_email)));
        }
    }
    // Verify old password matches current password
    let old_password = dto.old_password.clone();
    if let Some(pwd) = old_password {
        let password_match = match verify(pwd, &user_to_be_updated.password_hash) {
            Ok(matched) => matched,
            Err(_) => {
                return Err(ApiError::internal_error().message("Password verification failed"))
            }
        };

        if !password_match {
            return Err(ApiError::bad_request()
                .code(ApiErrorCode::InvalidOldPassword)
                .message("Invalid old password".to_string()));
        }
    }

    // Update user
    let user_result = context
        .repo
        .user
        .update_user(user_id, to_api_params(dto, has_new_email))
        .await
        .map_err(|e| ApiError::internal_error().message(format!("Failed to update user: {}", e)))?;

    // Resend email confirmation if needed
    if has_new_email {
        resend_confirm_email(&context, user_result.id.clone(), &user_result.email).await?;
    }

    // Return response
    Ok((StatusCode::OK, Json(to_api_response(user_result))))
}

fn to_api_params(dto: UpdateUserDto, has_new_email: bool) -> UserUpdateParams {
    return UserUpdateParams {
        email: dto.email,
        name: dto.name,
        description: dto.description,
        link: dto.link,
        location: dto.location,
        password: dto.new_password,
        user_type: dto.user_type,
        user_status: dto.user_status,
        email_confirmed: if has_new_email { Some(false) } else { None },
    };
}
