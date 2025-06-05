use axum::{extract::State, http::StatusCode, Extension, Json};

use chrono::{Duration, Utc};
use lib_api::{
    error::{api_error::ApiError, helpers::check_bad_form},
    util::{conversion::str_to_uuid, json_extractor::QJson},
};
use lib_types::{
    dto::quiz_asset::create_quiz_asset_dto::{
        to_api_response, CreateQuizAssetDto, CreateQuizAssetResponse,
    },
    shared::{
        asset::AssetState,
        user::{RequestUser, UserType},
    },
};
use validator::Validate;

use crate::{api_context::ApiContext, db::quiz_asset_repo::QuizAssetEntityProps};

use super::helpers::check_quiz_asset_exceeded;

pub async fn create_quiz_asset(
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<CreateQuizAssetDto>,
) -> Result<(StatusCode, Json<CreateQuizAssetResponse>), ApiError> {
    check_bad_form(dto.validate())?;

    let user_id = request_user.user_id.ok_or(ApiError::forbidden())?;

    let quiz_id = str_to_uuid(&dto.quiz_id)?;

    let quiz = context
        .repo
        .quiz
        .get_quiz_by_id(quiz_id)
        .await
        .map_err(|_| ApiError::bad_request().message("Quiz does not exist"))?;

    if request_user.user_type != UserType::Admin && quiz.user_id != user_id {
        return Err(ApiError::forbidden().message("User does not own quiz"));
    }

    check_quiz_asset_exceeded(&context, user_id, dto.content_size).await?;

    let content_type = dto.content_type.to_string();

    let expires_seconds: i64 = 600;
    // Remove query/signature for storage
    let entity_props = QuizAssetEntityProps {
        size: dto.content_size,
        content_type: content_type.clone(),
        state: AssetState::Created,
        user_id,
        quiz_id,
        upload_expires_at: Utc::now() + Duration::seconds(expires_seconds),
    };

    let quiz_asset = context
        .repo
        .quiz_asset
        .create_quiz_asset(entity_props)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create quiz asset: {}", e))
        })?;

    let signed_url = context.s3_client.presign_put_quiz_asset(
        &quiz_asset.relative_url(),
        expires_seconds as u64,
        &content_type,
        dto.content_size,
    )?;

    let response = Json(to_api_response(quiz_asset, signed_url.to_string()));
    Ok((StatusCode::CREATED, response))
}
