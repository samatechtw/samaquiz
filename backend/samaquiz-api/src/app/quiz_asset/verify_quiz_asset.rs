use axum::{
    extract::{Path, State},
    Extension, Json,
};
use chrono::Utc;
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::quiz_asset::verify_quiz_asset_dto::VerifyQuizAssetResponse,
    shared::{asset::AssetState, user::RequestUser},
};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::verify_admin_or_user};

use super::helpers::verify_quiz_asset_exist;

pub async fn verify_quiz_asset(
    Path(asset_id): Path<Uuid>,
    Extension(request_user): Extension<RequestUser>,
    State(context): State<ApiContext>,
) -> Result<Json<VerifyQuizAssetResponse>, ApiError> {
    let quiz_asset = verify_quiz_asset_exist(&context, asset_id).await?;

    // Check if the asset belongs to the current user
    verify_admin_or_user(&request_user, quiz_asset.user_id.to_string())?;

    // Perform a HEAD request to check if the object exists
    let exists = context
        .s3_client
        .verify_quiz_asset(&quiz_asset.relative_url())
        .await?;

    // Determine the state based on the verification result and expiration time
    let now = Utc::now();
    let state = if exists {
        AssetState::Uploaded
    } else if now > quiz_asset.upload_expires_at {
        AssetState::Expired
    } else {
        AssetState::Created
    };

    // Update the state of the site asset in the repository
    context
        .repo
        .quiz_asset
        .update_quiz_asset_state(asset_id, state)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to update site asset: {}", e))
        })?;

    Ok(Json(VerifyQuizAssetResponse { verified: exists }))
}
