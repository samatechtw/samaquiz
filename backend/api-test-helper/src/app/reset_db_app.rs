use axum::{extract::State, http::StatusCode};
use db_app::seeds::seed_all;
use lib_api::error::api_error::ApiError;

use crate::{api_context::ApiContext, db::api_helper_repo::ApiHelperRepo};

pub async fn reset_app(repo: ApiHelperRepo) -> Result<(), ApiError> {
    repo.app
        .reset()
        .await
        .map_err(|e| ApiError::internal_error().message(e))?;

    seed_all(repo.app.get_db())
        .await
        .map_err(|e| ApiError::internal_error().message(e))
}

pub async fn reset(State(context): State<ApiContext>) -> Result<StatusCode, ApiError> {
    reset_app(context.repo).await?;
    Ok(StatusCode::NO_CONTENT)
}
