use lib_api::{db::db_error::DbError, error::api_error::ApiError};
use lib_types::{
    entity::user_entity::UserEntity,
    shared::user::{RequestUser, UserType},
};

use crate::api_context::ApiContext;

// Returns 404 not found if a DB entry does not exist
pub fn not_found_or_internal(e: DbError) -> ApiError {
    match e {
        DbError::EntityNotFound() => ApiError::not_found(),
        _ => ApiError::internal_error().message(e),
    }
}

// Verifies the requester is Admin, or the User that owns the target resource
pub fn verify_admin_or_user(user: &RequestUser, user_id: String) -> Result<(), ApiError> {
    let requester_id = user.user_id.ok_or(ApiError::unauthorized())?.to_string();

    if user.user_type != UserType::Admin && user_id.to_string() != requester_id.to_string() {
        return Err(ApiError::forbidden());
    }
    return Ok(());
}

// Verifies the request user is Admin
pub fn verify_admin(user: &RequestUser) -> Result<(), ApiError> {
    if user.user_type != UserType::Admin {
        return Err(ApiError::forbidden());
    }
    return Ok(());
}

// Validates an User ID against the request user
// Used to ensure an user can only access their resources
// Returns User ID for use in queries
pub fn verify_user(user: &RequestUser, user_id: Option<String>) -> Result<String, ApiError> {
    let requester_id = user.user_id.ok_or(ApiError::unauthorized())?.to_string();

    Ok(if let Some(user_id) = user_id {
        if user_id.to_string() != requester_id.to_string() {
            return Err(ApiError::forbidden());
        } else {
            user_id
        }
    } else {
        requester_id
    })
}

pub async fn get_request_user(
    context: &ApiContext,
    user: &RequestUser,
) -> Result<UserEntity, ApiError> {
    let user_id = user
        .user_id
        .ok_or(ApiError::internal_error().message("Request user not found"))?;

    // Get the request user
    Ok(context
        .repo
        .user
        .get_user_by_id(user_id)
        .await
        .map_err(not_found_or_internal)?)
}
