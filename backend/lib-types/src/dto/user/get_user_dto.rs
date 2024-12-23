use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::user_entity::UserEntity,
    shared::user::{UserStatus, UserType},
};

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub link: String,
    pub location: String,
    pub email: String,
    pub user_type: UserType,
    pub user_status: UserStatus,
    pub email_confirmed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn to_api_response(user_entity: UserEntity) -> GetUserResponse {
    return GetUserResponse {
        id: user_entity.id,
        name: user_entity.name,
        description: user_entity.description,
        link: user_entity.link,
        location: user_entity.location,
        email: user_entity.email,
        user_type: user_entity.user_type,
        user_status: user_entity.user_status,
        email_confirmed: user_entity.email_confirmed,
        created_at: user_entity.created_at,
        updated_at: user_entity.updated_at,
    };
}
