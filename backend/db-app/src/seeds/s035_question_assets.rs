use std::str::FromStr;

use chrono::{Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::{
    entity::question_asset_entity::QuestionAssetEntity,
    shared::asset::{AssetContentType, AssetState},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "question_assets";

    let data = vec![
        QuestionAssetEntity {
            id: Uuid::from_str("e79912c3-2647-48e1-96c3-5a76e86befbc").unwrap(),
            question_id: Uuid::from_str("11354d45-903d-4493-9b96-5f07497b01e1").unwrap(),
            size: 40000,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        QuestionAssetEntity {
            id: Uuid::from_str("8ce7d0d7-434a-4cb9-8edb-810412121d79").unwrap(),
            question_id: Uuid::from_str("813a13c9-4562-4fa3-8d23-f46a079a57de").unwrap(),
            size: 1000000,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Created,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuestionAssetEntity {
            id: Uuid::from_str("51b9b547-8409-4017-926d-245e131edf70").unwrap(),
            question_id: Uuid::from_str("4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd").unwrap(),
            size: 1000001,
            content_type: AssetContentType::Jpeg,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now() - Duration::days(2),
            updated_at: Utc::now(),
        },
        QuestionAssetEntity {
            id: Uuid::from_str("7d43646e-c438-4f26-9170-6a841a9df551").unwrap(),
            question_id: Uuid::from_str("a15ff0f0-8825-4b64-8a16-2add5747bf42").unwrap(),
            size: 1000002,
            content_type: AssetContentType::Png,
            state: AssetState::Uploaded,
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            upload_expires_at: Utc::now(),
            created_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now(),
        },
        QuestionAssetEntity {
            id: Uuid::from_str("73d117e0-e435-4b95-b42f-e81bbb0943ab").unwrap(),
            question_id: Uuid::from_str("535b9aab-21af-494d-a349-ca2c66e8a4ec").unwrap(),
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
