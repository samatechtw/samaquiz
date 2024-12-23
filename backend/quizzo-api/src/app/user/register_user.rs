use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use lib_api::db::db_error::DbError;
use lib_api::db::util::commit_or_rollback;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::user::register_user_dto::{RegisterUserDto, RegisterUserResponse};
use lib_types::entity::user_entity::UserCreateResult;
use lib_types::shared::api_error::ApiErrorCode;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::auth::helpers::send_welcome_email;

fn to_api_response(result: UserCreateResult) -> Json<RegisterUserResponse> {
    return Json(RegisterUserResponse { id: result.id });
}

pub async fn register_user(
    State(context): State<ApiContext>,
    QJson(dto): QJson<RegisterUserDto>,
) -> Result<(StatusCode, Json<RegisterUserResponse>), ApiError> {
    let email = dto.email.clone();
    check_bad_form(dto.validate())?;

    // Start an sqlx transaction
    let mut tx = context.repo.start_transaction().await?;

    let user_result = context
        .repo
        .user
        .create_user(&mut tx, dto)
        .await
        .map_err(|e| match e {
            DbError::Unique(_) => ApiError::bad_request()
                .code(ApiErrorCode::UserExists)
                .message("User already exists"),
            _ => ApiError::internal_error().message(format!("Failed to create user: {}", e)),
        })?;

    commit_or_rollback(tx, Ok(())).await?;

    send_welcome_email(&context, user_result.id, email).await?;

    Ok((StatusCode::CREATED, to_api_response(user_result)))
}
