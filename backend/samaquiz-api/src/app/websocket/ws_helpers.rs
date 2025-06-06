use lib_api::error::api_error::ApiError;
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    app::{
        helpers::verify_admin_or_user, quiz_session::helpers::verify_quiz_session_exist,
        websocket::ws_state::add_broadcaster,
    },
    util::auth::user_from_token,
};

use super::ws_state::{get_broadcaster, WsServerMessage};

pub async fn is_session_owner(
    context: &ApiContext,
    session_id: Uuid,
    token_opt: Option<String>,
) -> bool {
    if let Some(token) = token_opt {
        let Ok(session) = verify_quiz_session_exist(context, session_id).await else {
            return false;
        };
        let Ok(user) = user_from_token(context, token.clone()).await else {
            return false;
        };
        if verify_admin_or_user(&user, session.user_id.to_string()).is_ok() {
            return true;
        }
    }
    false
}

// Tries to send a message and resets the channel once if
fn send_retry(
    context: &ApiContext,
    session_id: String,
    msg: WsServerMessage,
    retry: bool,
) -> Result<(), ApiError> {
    let broadcaster = get_broadcaster(&context.ws_state, session_id.clone())?;
    if let Err(e) = broadcaster.send(msg.clone()) {
        if retry {
            println!("Retry add participant: {}", e);
            drop(broadcaster);
            let _ = add_broadcaster(&context.ws_state, session_id.clone());
            return send_retry(context, session_id, msg, false);
        } else {
            println!("Failed to add participant: {}", e);
            return Err(ApiError::internal_error());
        }
    }
    Ok(())
}

pub fn notify_add_participant(
    context: &ApiContext,
    session_id: String,
    count: i64,
) -> Result<(), ApiError> {
    send_retry(context, session_id, WsServerMessage::joined(count), true)
}

pub fn notify_add_response(
    context: &ApiContext,
    session_id: String,
    count: i64,
) -> Result<(), ApiError> {
    send_retry(
        context,
        session_id,
        WsServerMessage::quiz_response(count),
        true,
    )
}

pub fn broadcast_quiz_countdown(
    context: &ApiContext,
    session_id: String,
    start_time: i64,
) -> Result<(), ApiError> {
    send_retry(
        context,
        session_id,
        WsServerMessage::quiz_countdown(start_time),
        true,
    )
}

pub fn broadcast_question_end_update(
    context: &ApiContext,
    session_id: String,
    question_end_time: i64,
) -> Result<(), ApiError> {
    send_retry(
        context,
        session_id,
        WsServerMessage::question_end_update(question_end_time),
        true,
    )
}

pub fn broadcast_quiz_start(
    context: &ApiContext,
    session_id: String,
    question_end_time: i64,
) -> Result<(), ApiError> {
    send_retry(
        context,
        session_id,
        WsServerMessage::quiz_start(question_end_time),
        true,
    )
}

pub fn broadcast_quiz_end(context: &ApiContext, session_id: String) -> Result<(), ApiError> {
    send_retry(context, session_id, WsServerMessage::quiz_end(), true)
}

pub fn broadcast_quiz_cancel(context: &ApiContext, session_id: String) -> Result<(), ApiError> {
    send_retry(context, session_id, WsServerMessage::quiz_cancel(), true)
}

pub fn broadcast_question_start(
    context: &ApiContext,
    session_id: String,
    question_index: i64,
    question_end_time: i64,
) -> Result<(), ApiError> {
    send_retry(
        context,
        session_id,
        WsServerMessage::question_start(question_index, question_end_time),
        true,
    )
}
