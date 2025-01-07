use axum::{extract::State, Extension, Json};
use lib_api::error::{api_error::ApiError, helpers::check_bad_form};
use lib_types::{
    dto::quiz::{
        list_quizzes_dto::{ListQuizzesQuery, ListQuizzesResponse},
        quiz_view_model::{to_api_response, QuizViewModel},
    },
    shared::user::{RequestUser, UserType},
};
use validator::Validate;

use crate::{api_context::ApiContext, app::Qs};

pub async fn list_quizzes(
    State(context): State<ApiContext>,
    Qs(query): Qs<ListQuizzesQuery>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<ListQuizzesResponse>, ApiError> {
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

    let validated_query = ListQuizzesQuery {
        from: query.from,
        to: query.to,
        types: query.types,
        user_id,
        column: query.column,
        direction: query.direction,
    };

    let quizzes = context
        .repo
        .quiz
        .list_quizzes(validated_query)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to list quizzes: {}", e))
        })?;

    let view_models: Vec<QuizViewModel> = quizzes
        .results
        .into_iter()
        .map(|quizzes| to_api_response(quizzes))
        .collect();

    Ok(Json(ListQuizzesResponse {
        total: quizzes.total,
        results: view_models,
    }))
}
