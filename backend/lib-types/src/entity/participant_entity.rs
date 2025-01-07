use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ParticipantEntity {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub avatar: String,
    pub points: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ParticipantEntityRelations {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub avatar: String,
    pub points: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ParticipantListResults {
    pub total: i64,
    pub results: Vec<ParticipantEntity>,
}

pub struct ParticipantCreateResult {
    pub id: Uuid,
}

pub struct ParticipantUpdateParams {
    pub name: Option<String>,
    pub avatar: Option<String>,
}
