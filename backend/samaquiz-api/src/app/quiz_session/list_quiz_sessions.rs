use axum::{extract::State, Extension, Json};
use lib_api::error::{api_error::ApiError, helpers::check_bad_form};
use lib_types::{
    dto::quiz_session::{
        list_quiz_sessions_dto::{ListQuizSessionsQuery, ListQuizSessionsResponse},
        quiz_session_view_model::{to_api_response, QuizSessionViewModel},
    },
    shared::user::{RequestUser, UserType},
};
use validator::Validate;

use crate::{api_context::ApiContext, app::Qs};

pub async fn list_quiz_sessions(
    State(context): State<ApiContext>,
    Qs(query): Qs<ListQuizSessionsQuery>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<ListQuizSessionsResponse>, ApiError> {
    check_bad_form(query.validate())?;

    let user_id = if request_user.user_type == UserType::Admin {
        query.user_id
    } else {
        let id = request_user.user_id.map(|u| u.to_string());
        // User required to provide user_id matching their own
        if query.user_id.is_none() || query.user_id != id {
            return Err(ApiError::forbidden());
        }
        id
    };

    let validated_query = ListQuizSessionsQuery {
        from: query.from,
        to: query.to,
        user_id,
        column: query.column,
        direction: query.direction,
    };

    let sessions = context
        .repo
        .quiz_session
        .list_quiz_sessions(validated_query)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to list sessions: {}", e))
        })?;

    let view_models: Vec<QuizSessionViewModel> = sessions
        .results
        .into_iter()
        .map(|sessions| to_api_response(sessions))
        .collect();

    Ok(Json(ListQuizSessionsResponse {
        total: sessions.total,
        results: view_models,
    }))
}
