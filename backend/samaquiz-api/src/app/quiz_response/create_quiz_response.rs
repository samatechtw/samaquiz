use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use chrono::Utc;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::conversion::str_to_uuid;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::quiz_response::create_quiz_response_dto::{
    CreateQuizResponseDto, CreateQuizResponseResponse,
};
use lib_types::entity::participant_entity::ParticipantEntityRelations;
use lib_types::shared::api_error::ApiErrorCode;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::participant::helpers::verify_participant_exist_relations;
use crate::app::question::helpers::verify_question_exist_relations;
use crate::app::quiz::helpers::verify_quiz_exist_relations;
use crate::app::quiz_session::helpers::verify_quiz_session_exist_relations;
use crate::app::websocket::ws_helpers::notify_add_response;
use crate::db::participant_repo::ParticipantUpdateProps;
use crate::db::quiz_response_repo::QuizResponseCreateProps;

#[debug_handler]
pub async fn create_quiz_response(
    State(context): State<ApiContext>,
    QJson(dto): QJson<CreateQuizResponseDto>,
) -> Result<(StatusCode, Json<CreateQuizResponseResponse>), ApiError> {
    check_bad_form(dto.validate())?;
    let participant_id = str_to_uuid(&dto.participant_id)?;
    let question_id = str_to_uuid(&dto.question_id)?;
    let answer_id = str_to_uuid(&dto.answer_id)?;
    let participant: ParticipantEntityRelations =
        verify_participant_exist_relations(&context, participant_id).await?;
    let question = verify_question_exist_relations(&context, question_id.clone()).await?;
    let session = verify_quiz_session_exist_relations(&context, participant.session_id).await?;
    let quiz = verify_quiz_exist_relations(&context, session.quiz_id).await?;
    let question_index = session
        .question_index
        .ok_or(ApiError::bad_request().code(ApiErrorCode::InvalidQuestion))?;
    let active_question_id = quiz
        .questions_order
        .get(question_index as usize)
        .ok_or(ApiError::bad_request().code(ApiErrorCode::InvalidQuestion))?;
    if active_question_id != &dto.question_id {
        return Err(ApiError::bad_request().code(ApiErrorCode::InvalidQuestion));
    }
    if let Some(question_end_time) = session.question_end_time {
        if Utc::now().timestamp_millis() > question_end_time {
            return Err(ApiError::bad_request().code(ApiErrorCode::QuestionOver));
        }
    }
    let answer = question
        .answers
        .into_iter()
        .find(|q| q.id == answer_id)
        .ok_or(ApiError::bad_request().code(ApiErrorCode::InvalidAnswer))?;

    let props = QuizResponseCreateProps {
        participant_id,
        question_id,
        answer_id,
        is_correct: answer.is_correct,
    };

    let result = context
        .repo
        .quiz_response
        .create_quiz_response(props)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create quiz_response: {}", e))
        })?;

    if answer.is_correct {
        context
            .repo
            .participant
            .update_participant(
                participant_id,
                ParticipantUpdateProps::points(participant.points + answer.points),
            )
            .await
            .map_err(|e| {
                ApiError::internal_error().message(format!("Failed to add points: {}", e))
            })?;
    }

    let _ = notify_add_response(
        &context,
        session.id.to_string(),
        result.question_response_count,
    );

    Ok((StatusCode::CREATED, Json(result)))
}
