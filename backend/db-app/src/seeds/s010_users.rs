use std::str::FromStr;

use chrono::Utc;
use lib_api::db::db_error::DbError;
use lib_api::db::password::hash;
use lib_types::{
    entity::user_entity::UserEntity,
    shared::user::{UserStatus, UserType},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "users";

    let data = vec![
        UserEntity {
            id: Uuid::from_str("083fb9af-a2fd-41b8-bfa2-4747cc87b604").unwrap(),
            name: "Admin".into(),
            description: "SamaTech Admin".into(),
            link: "https://samatech.tw".into(),
            location: "Hong Kong".into(),
            email: "admin1@samatech.tw".into(),
            password_hash: hash("admin.password1".into()).await.unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::Admin,
            user_status: UserStatus::Active,
            email_confirmed: true,
        },
        UserEntity {
            id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            name: "user1@samatech.tw".into(),
            description: "First User".into(),
            link: "https://samatech.tw/user/2213d9fc-3693-47ed-a495-cd5e7fc6dd0e".into(),
            location: "Japan".into(),
            email: "user1@samatech.tw".into(),
            password_hash: hash("password1".into()).await.unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::User,
            user_status: UserStatus::Active,
            email_confirmed: true,
        },
        UserEntity {
            id: Uuid::from_str("749a72a8-14cd-4893-98eb-45bed891463d").unwrap(),
            name: "tech_dude".into(),
            description: "Just here to learn!.".into(),
            link: "https://me.hello.com".into(),
            location: "California".into(),
            email: "user2@samatech.tw".into(),
            password_hash: hash("password2".into()).await.unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::User,
            user_status: UserStatus::Blocked,
            email_confirmed: true,
        },
        UserEntity {
            id: Uuid::from_str("028ba9f2-f360-423b-83b6-44863b69e211").unwrap(),
            name: "user3@samatech.tw".into(),
            description: "Big buyer".into(),
            link: "".into(),
            location: "Europe".into(),
            email: "user3@samatech.tw".into(),
            password_hash: hash("password3".into()).await.unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::User,
            user_status: UserStatus::Active,
            email_confirmed: false,
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
