use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::user::{UserStatus, UserType};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub link: String,
    pub location: String,
    pub email: String,
    pub password_hash: String,
    pub user_type: UserType,
    pub user_status: UserStatus,
    pub email_confirmed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserCreateResult {
    pub id: Uuid,
}

pub struct UserUpdateParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub user_type: Option<UserType>,
    pub user_status: Option<UserStatus>,
    pub email_confirmed: Option<bool>,
}

impl UserUpdateParams {
    pub fn email(email: String) -> Self {
        Self {
            name: None,
            description: None,
            link: None,
            location: None,
            email: Some(email),
            password: None,
            user_type: None,
            user_status: None,
            email_confirmed: None,
        }
    }
    pub fn password(password: String) -> Self {
        Self {
            name: None,
            description: None,
            link: None,
            location: None,
            email: None,
            password: Some(password),
            user_type: None,
            user_status: None,
            email_confirmed: None,
        }
    }
    pub fn email_confirmed(confirmed: bool) -> Self {
        Self {
            name: None,
            email: None,
            description: None,
            link: None,
            location: None,
            password: None,
            user_type: None,
            user_status: None,
            email_confirmed: Some(confirmed),
        }
    }
}

#[derive(Debug)]
pub struct UserListResults {
    pub total: i64,
    pub results: Vec<UserEntity>,
}
