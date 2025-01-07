use axum::{
    extract::{Query, State},
    Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::dto::user::{
    list_users_dto::{ListUsersQuery, ListUsersResponse},
    user_view_model::{to_api_response, UserViewModel},
};

use crate::api_context::ApiContext;

pub async fn list_users(
    State(context): State<ApiContext>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<ListUsersResponse>, ApiError> {
    let users =
        context.repo.user.list_users(query).await.map_err(|e| {
            ApiError::internal_error().message(format!("Failed to list users: {}", e))
        })?;

    let view_models: Vec<UserViewModel> = users
        .results
        .into_iter()
        .map(|users| to_api_response(users))
        .collect();

    Ok(Json(ListUsersResponse {
        total: users.total,
        results: view_models,
    }))
}
