use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::{
        participant::list_participants_dto::ListParticipantsQuery,
        quiz_session::get_quiz_session_dto::{to_api_response, GetQuizSessionResponse},
    },
    shared::user::RequestUser,
};

use crate::{
    api_context::ApiContext,
    app::{helpers::not_found_or_internal, quiz::helpers::verify_quiz_exist_relations},
};

pub async fn get_quiz_session(
    Path(code): Path<String>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<GetQuizSessionResponse>, ApiError> {
    // Get quiz_session from DB
    let quiz_session = context
        .repo
        .quiz_session
        .get_quiz_session_by_code(code.to_lowercase())
        .await
        .map_err(not_found_or_internal)?;
    let quiz = verify_quiz_exist_relations(&context, quiz_session.quiz_id).await?;

    let participants = if Some(quiz.user_id) == request_user.user_id {
        Some(
            context
                .repo
                .participant
                .list_participants(ListParticipantsQuery::session(quiz_session.id.to_string()))
                .await
                .map_err(not_found_or_internal)?
                .results,
        )
    } else {
        None
    };

    Ok(Json(to_api_response(quiz_session, quiz, participants)))
}
