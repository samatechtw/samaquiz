use std::str::FromStr;

use chrono::{Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::{entity::quiz_entity::QuizEntity, shared::quiz::QuizType};
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "quizzes";

    let data = vec![
        QuizEntity {
            id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            title: "My First Quiz".into(),
            description: "A quiz about my favorite things".into(),
            quiz_type: QuizType::Quiz,
            questions_order: vec![
                "11354d45-903d-4493-9b96-5f07497b01e1".into(),
                "813a13c9-4562-4fa3-8d23-f46a079a57de".into(),
                "a82c0a88-10eb-43cd-b057-7214bb598111".into(),
                "4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd".into(),
            ],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        QuizEntity {
            id: Uuid::from_str("efd3863f-a975-4b2d-9b03-eb3fe28410b9").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            title: "Second Quiz".into(),
            description: "A quiz about nothing much".into(),
            quiz_type: QuizType::Quiz,
            questions_order: vec![
                "a15ff0f0-8825-4b64-8a16-2add5747bf42".into(),
                "d8e51677-32f8-4d71-aa68-e7ea3a0e8135".into(),
            ],
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuizEntity {
            id: Uuid::from_str("5db0613d-0077-48c5-a3f1-f0faefe2a083").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            title: "The Third".into(),
            description: "Just a little quiz test.".into(),
            quiz_type: QuizType::Quiz,
            questions_order: vec!["535b9aab-21af-494d-a349-ca2c66e8a4ec".into()],
            created_at: Utc::now() - Duration::days(2),
            updated_at: Utc::now(),
        },
        QuizEntity {
            id: Uuid::from_str("7070ba54-6ed7-4916-b3b6-e7251770d0b1").unwrap(),
            user_id: Uuid::from_str("028ba9f2-f360-423b-83b6-44863b69e211").unwrap(),
            title: "The Third".into(),
            description: "Just a little quiz test.".into(),
            quiz_type: QuizType::Quiz,
            questions_order: vec![],
            created_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
