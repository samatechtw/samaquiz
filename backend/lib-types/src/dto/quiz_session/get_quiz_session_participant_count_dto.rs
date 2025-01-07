use serde::Serialize;

#[derive(Serialize)]
pub struct GetQuizSessionParticipantCountResponse {
    pub count: i64,
}
