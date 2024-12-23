use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::shared::question::QuestionType;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

#[derive(Debug, Serialize, sqlx::Type)]
pub struct QuestionEntityDbProps {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
    pub answers_order: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "questions";

    let data: Vec<QuestionEntityDbProps> = vec![
        QuestionEntityDbProps {
            id: Uuid::from_str("11354d45-903d-4493-9b96-5f07497b01e1").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            text: "Quiz 1 Question 1".into(),
            question_type: QuestionType::MultipleChoice,
            answers_order: vec![
                "f424b181-ec35-4a3b-9b81-d467cd0cbe7b".into(),
                "66f41b1d-7777-4045-8ecf-30519a3a0a30".into(),
                "869563bf-63b0-47d4-812e-be31e73fe79a".into(),
                "a314a297-5d3c-42f3-a7a7-7d8726aafd1f".into(),
            ],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        QuestionEntityDbProps {
            id: Uuid::from_str("813a13c9-4562-4fa3-8d23-f46a079a57de").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            text: "Quiz 1 Question 2".into(),
            question_type: QuestionType::MultipleChoice,
            answers_order: vec![
                "d06315f6-6304-45c0-9020-983b232e4701".into(),
                "acd3ffa8-c94b-4a9a-9d83-bb713fff076b".into(),
            ],
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuestionEntityDbProps {
            id: Uuid::from_str("a82c0a88-10eb-43cd-b057-7214bb598111").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            text: "Quiz 1 Question 3".into(),
            question_type: QuestionType::MultipleChoice,
            answers_order: vec![
                "160f6b6f-cb13-4d43-96d3-6c17e1a5d679".into(),
                "5b857643-f296-42eb-b65c-4adddde33ca3".into(),
            ],
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuestionEntityDbProps {
            id: Uuid::from_str("4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            text: "Quiz 1 Question 4".into(),
            question_type: QuestionType::MultipleChoice,
            answers_order: vec![
                "45c4fda5-2482-4c3b-96d2-18806c57c36f".into(),
                "a8685eee-6d99-41e5-b4c5-c1711c5fc5cb".into(),
            ],
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuestionEntityDbProps {
            id: Uuid::from_str("a15ff0f0-8825-4b64-8a16-2add5747bf42").unwrap(),
            quiz_id: Uuid::from_str("efd3863f-a975-4b2d-9b03-eb3fe28410b9").unwrap(),
            text: "Quiz 2 Question 1".into(),
            question_type: QuestionType::MultipleChoice,
            answers_order: vec![],
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuestionEntityDbProps {
            id: Uuid::from_str("d8e51677-32f8-4d71-aa68-e7ea3a0e8135").unwrap(),
            quiz_id: Uuid::from_str("efd3863f-a975-4b2d-9b03-eb3fe28410b9").unwrap(),
            text: "Quiz 2 Question 2".into(),
            question_type: QuestionType::MultipleChoice,
            answers_order: vec![],
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuestionEntityDbProps {
            id: Uuid::from_str("535b9aab-21af-494d-a349-ca2c66e8a4ec").unwrap(),
            quiz_id: Uuid::from_str("5db0613d-0077-48c5-a3f1-f0faefe2a083").unwrap(),
            text: "Quiz 3 Question 1".into(),
            question_type: QuestionType::MultipleChoice,
            answers_order: vec![
                "08534ea9-5bd7-42ea-9dfb-c0a8991f2095".into(),
                "f945befc-0a6a-472a-825a-9a5709ef97eb".into(),
            ],
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
