use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateParticipantDto {
    #[validate(length(min = 2, max = 16))]
    pub name: String,
    #[validate(length(max = 100))]
    pub avatar: String,
}

#[derive(Serialize)]
pub struct CreateParticipantResponse {
    pub id: Uuid,
    pub count: i64,
}
