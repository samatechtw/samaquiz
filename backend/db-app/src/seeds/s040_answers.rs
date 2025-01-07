use std::str::FromStr;

use chrono::{Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::entity::answer_entity::AnswerEntity;
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "answers";

    let data: Vec<AnswerEntity> = vec![
        AnswerEntity {
            id: Uuid::from_str("f424b181-ec35-4a3b-9b81-d467cd0cbe7b").unwrap(),
            question_id: Uuid::from_str("11354d45-903d-4493-9b96-5f07497b01e1").unwrap(),
            text: "Q1 Answer 1".into(),
            is_correct: true,
            points: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("66f41b1d-7777-4045-8ecf-30519a3a0a30").unwrap(),
            question_id: Uuid::from_str("11354d45-903d-4493-9b96-5f07497b01e1").unwrap(),
            text: "Q1 Answer 2".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("869563bf-63b0-47d4-812e-be31e73fe79a").unwrap(),
            question_id: Uuid::from_str("11354d45-903d-4493-9b96-5f07497b01e1").unwrap(),
            text: "Q1 Answer 3".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(2),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("a314a297-5d3c-42f3-a7a7-7d8726aafd1f").unwrap(),
            question_id: Uuid::from_str("11354d45-903d-4493-9b96-5f07497b01e1").unwrap(),
            text: "Q1 Answer 4".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("d06315f6-6304-45c0-9020-983b232e4701").unwrap(),
            question_id: Uuid::from_str("813a13c9-4562-4fa3-8d23-f46a079a57de").unwrap(),
            text: "Q2 Answer 1".into(),
            is_correct: true,
            points: 1,
            created_at: Utc::now() - Duration::days(4),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("acd3ffa8-c94b-4a9a-9d83-bb713fff076b").unwrap(),
            question_id: Uuid::from_str("813a13c9-4562-4fa3-8d23-f46a079a57de").unwrap(),
            text: "Q2 Answer 2".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(5),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("160f6b6f-cb13-4d43-96d3-6c17e1a5d679").unwrap(),
            question_id: Uuid::from_str("a82c0a88-10eb-43cd-b057-7214bb598111").unwrap(),
            text: "Q3 Answer 1".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(6),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("5b857643-f296-42eb-b65c-4adddde33ca3").unwrap(),
            question_id: Uuid::from_str("a82c0a88-10eb-43cd-b057-7214bb598111").unwrap(),
            text: "Q3 Answer 2".into(),
            is_correct: true,
            points: 1,
            created_at: Utc::now() - Duration::days(7),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("45c4fda5-2482-4c3b-96d2-18806c57c36f").unwrap(),
            question_id: Uuid::from_str("4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd").unwrap(),
            text: "Q4 Answer 1".into(),
            is_correct: true,
            points: 1,
            created_at: Utc::now() - Duration::days(8),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("a8685eee-6d99-41e5-b4c5-c1711c5fc5cb").unwrap(),
            question_id: Uuid::from_str("4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd").unwrap(),
            text: "Q4 Answer 2".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(9),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("08534ea9-5bd7-42ea-9dfb-c0a8991f2095").unwrap(),
            question_id: Uuid::from_str("535b9aab-21af-494d-a349-ca2c66e8a4ec").unwrap(),
            text: "Quiz3 Answer 1".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(10),
            updated_at: Utc::now(),
        },
        AnswerEntity {
            id: Uuid::from_str("f945befc-0a6a-472a-825a-9a5709ef97eb").unwrap(),
            question_id: Uuid::from_str("535b9aab-21af-494d-a349-ca2c66e8a4ec").unwrap(),
            text: "Quiz3 Answer 2".into(),
            is_correct: false,
            points: 1,
            created_at: Utc::now() - Duration::days(11),
            updated_at: Utc::now(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
