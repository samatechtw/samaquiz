use std::sync::Arc;

use dashmap::{mapref::one::Ref, DashMap};
use lib_api::error::api_error::ApiError;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

pub type Broadcaster = broadcast::Sender<WsServerMessage>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsReceiverType {
    Host,
    Participant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsReadyData {
    pub value: WsReceiverType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsJoinedData {
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsQuizCountdownData {
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsQuestionStartData {
    pub question_index: i64,
    pub question_end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsQuizResponseData {
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsQuizEndData {
    // TODO -- placeholder
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsServerMessage {
    Ready(WsReadyData),
    Joined(WsJoinedData),
    QuizCountdown(WsQuizCountdownData),
    QuizStart(WsQuestionStartData),
    QuestionStart(WsQuestionStartData),
    QuizResponse(WsQuizResponseData),
    QuizEnd(WsQuizEndData),
    QuizCancel(WsQuizEndData),
}

impl WsServerMessage {
    pub fn ready_participant() -> WsServerMessage {
        WsServerMessage::Ready(WsReadyData {
            value: WsReceiverType::Participant,
        })
    }

    pub fn ready_host() -> WsServerMessage {
        WsServerMessage::Ready(WsReadyData {
            value: WsReceiverType::Host,
        })
    }

    pub fn joined(count: i64) -> WsServerMessage {
        WsServerMessage::Joined(WsJoinedData { value: count })
    }

    pub fn quiz_countdown(start_time: i64) -> WsServerMessage {
        WsServerMessage::QuizCountdown(WsQuizCountdownData { value: start_time })
    }

    pub fn quiz_start(question_end_time: i64) -> WsServerMessage {
        WsServerMessage::QuizStart(WsQuestionStartData {
            question_index: 0,
            question_end_time,
        })
    }

    pub fn quiz_end() -> WsServerMessage {
        WsServerMessage::QuizEnd(WsQuizEndData { value: 0 })
    }

    pub fn quiz_cancel() -> WsServerMessage {
        WsServerMessage::QuizCancel(WsQuizEndData { value: 0 })
    }

    pub fn quiz_response(count: i64) -> WsServerMessage {
        WsServerMessage::QuizResponse(WsQuizResponseData { value: count })
    }

    pub fn question_start(question_index: i64, question_end_time: i64) -> WsServerMessage {
        WsServerMessage::QuestionStart(WsQuestionStartData {
            question_index,
            question_end_time,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsAuthData {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsClientMessage {
    Auth(WsAuthData),
}

#[derive(Clone)]
pub struct WsState {
    broadcasters: Arc<DashMap<String, Broadcaster>>,
}

pub fn init_ws_state() -> WsState {
    WsState {
        broadcasters: Arc::new(DashMap::new()),
    }
}

pub fn add_broadcaster(state: &WsState, session_id: String) -> Result<(), ApiError> {
    let broadcaster = Broadcaster::new(1000);
    let _ = state
        .broadcasters
        .insert(session_id, broadcaster)
        .ok_or(ApiError::internal_error().message("Add broadcaster"))?;
    Ok(())
}

pub fn get_broadcaster<'a>(
    state: &'a WsState,
    session_id: String,
) -> Result<Ref<'a, String, Broadcaster>, ApiError> {
    if let Some(broadcaster) = state.broadcasters.get(&session_id) {
        return Ok(broadcaster);
    }
    let broadcaster = Broadcaster::new(1000);
    state.broadcasters.insert(session_id.clone(), broadcaster);
    Ok(state
        .broadcasters
        .get(&session_id)
        .ok_or(ApiError::internal_error().message("Get broadcaster"))?)
}
