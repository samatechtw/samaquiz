use lib_api::error::api_error::ApiError;
use lib_types::{entity::quiz_asset_entity::QuizAssetEntity, shared::api_error::ApiErrorCode};
use uuid::Uuid;

use crate::api_context::ApiContext;

pub async fn verify_quiz_asset_exist(
    context: &ApiContext,
    id: Uuid,
) -> Result<QuizAssetEntity, ApiError> {
    let asset = context
        .repo
        .quiz_asset
        .get_quiz_asset_by_id(id)
        .await
        .map_err(|_| ApiError::not_found().message("Quiz asset not found"))?;
    Ok(asset)
}

pub async fn check_quiz_asset_exceeded(
    context: &ApiContext,
    user_id: Uuid,
    new_asset_size: i64,
) -> Result<(), ApiError> {
    // TODO -- update with real allocation
    let mut asset_allocation = 100_000_000.0;

    // Apply 5% margin
    asset_allocation = asset_allocation * 1.05;

    // Get current total asset usage
    let asset_usage = context
        .repo
        .quiz_asset
        .get_total_asset_usage_by_user(user_id)
        .await
        .map_err(|e| ApiError::internal_error().message(e))?;

    // Verify asset limit
    if asset_usage as f64 + new_asset_size as f64 > asset_allocation {
        return Err(ApiError::bad_request()
            .code(ApiErrorCode::AssetUsageExceeded)
            .message("Asset usage exceeded limit".to_string()));
    }

    Ok(())
}
