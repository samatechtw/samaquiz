use std::str::FromStr;

use chrono::{Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::{
    entity::quiz_asset_entity::QuizAssetEntity,
    shared::asset::{AssetContentType, AssetState},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "quiz_assets";

    let data = vec![
        QuizAssetEntity {
            id: Uuid::from_str("77bd4ae2-418c-43ef-8c16-ca5462bca1c3").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            size: 90_000_000,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Created,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("8041973b-be11-4eb2-8572-ec5e43008ef6").unwrap(),
            quiz_id: Uuid::from_str("efd3863f-a975-4b2d-9b03-eb3fe28410b9").unwrap(),
            size: 1_000_000,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Created,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("1ee59f27-56bb-4081-a106-3f2fb70afd11").unwrap(),
            quiz_id: Uuid::from_str("5db0613d-0077-48c5-a3f1-f0faefe2a083").unwrap(),
            size: 1000001,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now() - Duration::days(2),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("2bd4ed9b-7a7e-4f2c-b46c-24780ea53053").unwrap(),
            quiz_id: Uuid::from_str("7070ba54-6ed7-4916-b3b6-e7251770d0b1").unwrap(),
            size: 1000002,
            content_type: AssetContentType::Png,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("028ba9f2-f360-423b-83b6-44863b69e211").unwrap(),
            upload_expires_at: Utc::now(),
            created_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("e79912c3-2647-48e1-96c3-5a76e86befbc").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            size: 40000,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("8ce7d0d7-434a-4cb9-8edb-810412121d79").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            size: 1000000,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Created,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("51b9b547-8409-4017-926d-245e131edf70").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            size: 1000001,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now() - Duration::days(2),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("7d43646e-c438-4f26-9170-6a841a9df551").unwrap(),
            quiz_id: Uuid::from_str("efd3863f-a975-4b2d-9b03-eb3fe28410b9").unwrap(),
            size: 1000002,
            content_type: AssetContentType::Png,
            state: AssetState::Created,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now(),
            created_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now(),
        },
        QuizAssetEntity {
            id: Uuid::from_str("73d117e0-e435-4b95-b42f-e81bbb0943ab").unwrap(),
            quiz_id: Uuid::from_str("5db0613d-0077-48c5-a3f1-f0faefe2a083").unwrap(),
            size: 2000000,
            content_type: AssetContentType::Png,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now() - Duration::days(4),
            updated_at: Utc::now(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
