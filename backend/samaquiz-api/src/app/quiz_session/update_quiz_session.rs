use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;
use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::quiz_session::update_quiz_session_dto::UpdateQuizSessionDto;
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::quiz_session::QuizSessionStatus;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;

use crate::app::helpers::verify_admin_or_user;
use crate::app::quiz::helpers::verify_quiz_exist;
use crate::app::websocket::ws_helpers::{
    broadcast_question_end_update, broadcast_question_start, broadcast_quiz_cancel,
    broadcast_quiz_countdown, broadcast_quiz_end, broadcast_quiz_start,
};
use crate::db::quiz_session_repo::QuizSessionUpdateProps;

use super::helpers::verify_quiz_session_exist_relations;

pub async fn update_quiz_session(
    Path(quiz_session_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<UpdateQuizSessionDto>,
) -> Result<(StatusCode, ()), ApiError> {
    check_bad_form(dto.validate())?;

    // Get quiz_session to be updated
    let quiz_session = verify_quiz_session_exist_relations(&context, quiz_session_id).await?;
    let quiz = verify_quiz_exist(&context, quiz_session.quiz_id).await?;

    // Verify request
    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    // Verify status
    if let Some(status) = dto.status {
        let status_err = Err(ApiError::bad_request()
            .code(ApiErrorCode::InvalidStatus)
            .message("Invalid status update"));
        match status {
            QuizSessionStatus::Ready => return status_err,
            QuizSessionStatus::Active => {
                if quiz.questions_order.len() == 0 {
                    return Err(ApiError::bad_request()
                        .code(ApiErrorCode::NoQuestions)
                        .message("Quiz must have questions"));
                }
                if quiz_session.status == QuizSessionStatus::Canceled
                    || quiz_session.status == QuizSessionStatus::Complete
                {
                    return status_err;
                }
            }
            QuizSessionStatus::Canceled => {
                if quiz_session.status == QuizSessionStatus::Complete {
                    return status_err;
                }
            }
            QuizSessionStatus::Complete => {
                if quiz_session.status == QuizSessionStatus::Canceled
                    || quiz_session.status == QuizSessionStatus::Ready
                {
                    return status_err;
                }
            }
        }
    }

    let props = QuizSessionUpdateProps {
        code: dto.code.map(|c| c.to_lowercase()),
        host_name: dto.host_name,
        host_avatar: dto.host_avatar,
        status: dto.status,
        start_time: dto.start_time.clone(),
        question_end_time: dto.question_end_time,
        question_index: dto.question_index,
        question_duration: dto.question_duration,
        end_time: dto.end_time,
    };

    // Update quiz_session
    context
        .repo
        .quiz_session
        .update_quiz_session(quiz_session_id, props)
        .await
        .map_err(|e| match e {
            _ => {
                ApiError::internal_error().message(format!("Failed to update quiz_session: {}", e))
            }
        })?;

    if let Some(start_time) = dto.start_time {
        let _ = broadcast_quiz_countdown(&context, quiz_session_id.to_string(), start_time);
    }
    if let Some(status) = dto.status {
        // Avoid sending duplicate broadcasts
        if quiz_session.status != status {
            let q_end = dto.question_end_time;
            if status == QuizSessionStatus::Active && q_end.is_some() {
                let _ = broadcast_quiz_start(&context, quiz_session_id.to_string(), q_end.unwrap());
            }
            if status == QuizSessionStatus::Complete {
                let _ = broadcast_quiz_end(&context, quiz_session_id.to_string());
            }
            if status == QuizSessionStatus::Canceled {
                let _ = broadcast_quiz_cancel(&context, quiz_session_id.to_string());
            }
        }
    }
    if let (Some(question_index), Some(question_end_time)) =
        (dto.question_index, dto.question_end_time)
    {
        let prev_index = quiz_session.question_index.unwrap_or(0);
        if prev_index != question_index {
            let _ = broadcast_question_start(
                &context,
                quiz_session_id.to_string(),
                question_index,
                question_end_time,
            );
        }
    }
    if dto.status.is_none() && dto.question_index.is_none() {
        if let Some(q_end) = dto.question_end_time {
            let _ = broadcast_question_end_update(&context, quiz_session_id.to_string(), q_end);
        }
    }

    // Return response
    Ok((StatusCode::OK, ()))
}
