use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::entity::participant_entity::ParticipantEntityRelations;

#[derive(Serialize)]
pub struct ParticipantViewModel {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
}

pub fn to_api_response(participant_entity: ParticipantEntityRelations) -> ParticipantViewModel {
    return ParticipantViewModel {
        id: participant_entity.id,
        session_id: participant_entity.session_id,
        user_id: participant_entity.user_id,
        name: participant_entity.name,
        avatar: participant_entity.avatar,
        created_at: participant_entity.created_at,
    };
}
