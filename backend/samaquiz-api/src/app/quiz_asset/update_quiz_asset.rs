use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::{
    db::db_error::DbError,
    error::{api_error::ApiError, helpers::check_bad_form},
    util::json_extractor::QJson,
};
use lib_types::{
    dto::quiz_asset::{
        quiz_asset_viewmodel::{to_api_response, QuizAssetViewModel},
        update_quiz_asset_dto::UpdateQuizAssetDto,
    },
    shared::user::{RequestUser, UserType},
};
use uuid::Uuid;
use validator::Validate;

use crate::{api_context::ApiContext, app::helpers::verify_admin_or_user};

use super::helpers::verify_quiz_asset_exist;

pub async fn update_quiz_asset(
    Path(asset_id): Path<Uuid>,
    Extension(request_user): Extension<RequestUser>,
    State(context): State<ApiContext>,
    QJson(dto): QJson<UpdateQuizAssetDto>,
) -> Result<Json<QuizAssetViewModel>, ApiError> {
    check_bad_form(dto.validate())?;
    let user_type = request_user.user_type;

    // Get Asset
    let quiz_asset = verify_quiz_asset_exist(&context, asset_id).await?;

    // Check if the asset belongs to the current user
    verify_admin_or_user(&request_user, quiz_asset.user_id.to_string())?;

    if user_type == UserType::User {
        if dto.state.is_some() {
            return Err(ApiError::bad_request().message("Unauthorized field update"));
        }
    }

    let result = context
        .repo
        .quiz_asset
        .update_quiz_asset(asset_id, dto)
        .await
        .map_err(|e| match e {
            DbError::NoUpdate => ApiError::bad_request().message("No changes were made"),
            DbError::EntityNotFound() => ApiError::not_found(),
            DbError::Query(e) => ApiError::bad_request()
                .message("Error occurred during the database query: ".to_owned() + &e),
            _ => ApiError::internal_error().message(format!("Failed to update site asset: {}", e)),
        })?;

    Ok(Json(to_api_response(result)))
}
