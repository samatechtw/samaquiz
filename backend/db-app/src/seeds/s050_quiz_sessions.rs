use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::shared::quiz_session::QuizSessionStatus;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizSessionEntityDbProps {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub host_name: String,
    pub host_avatar: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub question_end_time: Option<i64>,
    pub question_index: Option<i64>,
    pub question_duration: i64,
    pub status: QuizSessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "quiz_sessions";

    let data: Vec<QuizSessionEntityDbProps> = vec![
        QuizSessionEntityDbProps {
            id: Uuid::from_str("ce782f8a-01bb-4228-9e50-4db0248f14cd").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            code: "abc".into(),
            host_name: "Host 1".into(),
            host_avatar: Some("http://localhost:8080/src/theme/img/cats/cat1.png".into()),
            start_time: None,
            end_time: None,
            question_end_time: None,
            question_index: None,
            question_duration: 30 * 1000,
            status: QuizSessionStatus::Ready,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        QuizSessionEntityDbProps {
            id: Uuid::from_str("dab9771d-e818-435f-98de-6a734189ba7d").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            host_name: "Host 2".into(),
            host_avatar: Some("http://localhost:8080/src/theme/img/cats/cat3.png".into()),
            code: "123abc".into(),
            start_time: Some((Utc::now() + Duration::seconds(30)).timestamp_millis()),
            end_time: None,
            question_end_time: None,
            question_index: None,
            question_duration: 30 * 1000,
            status: QuizSessionStatus::Ready,
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        },
        QuizSessionEntityDbProps {
            id: Uuid::from_str("ffb8d80d-086f-4ce3-999b-ae9842afb564").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            host_name: "Host 3".into(),
            host_avatar: None,
            code: "activequiz".into(),
            start_time: Some((Utc::now() - Duration::minutes(2)).timestamp_millis()),
            end_time: None,
            question_end_time: Some((Utc::now() + Duration::seconds(10)).timestamp_millis()),
            question_index: Some(0),
            question_duration: 30 * 1000,
            status: QuizSessionStatus::Active,
            created_at: Utc::now() - Duration::days(2),
            updated_at: Utc::now(),
        },
        QuizSessionEntityDbProps {
            id: Uuid::from_str("d12520fe-7f5a-46a7-9682-f138c7f81078").unwrap(),
            quiz_id: Uuid::from_str("d6599ea6-818c-4687-8522-86bf880019c4").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            host_name: "Host 4".into(),
            host_avatar: Some("http://localhost:8080/src/theme/img/cats/cat5.png".into()),
            code: "myquiz".into(),
            start_time: Some((Utc::now() - Duration::minutes(30)).timestamp_millis()),
            end_time: Some((Utc::now() - Duration::minutes(10)).timestamp_millis()),
            question_end_time: None,
            question_index: None,
            question_duration: 30 * 1000,
            status: QuizSessionStatus::Complete,
            created_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now(),
        },
        QuizSessionEntityDbProps {
            id: Uuid::from_str("31e8823b-7b72-4529-a0f8-94bf77bdf9b5").unwrap(),
            quiz_id: Uuid::from_str("efd3863f-a975-4b2d-9b03-eb3fe28410b9").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            host_name: "Host 5".into(),
            host_avatar: Some("http://localhost:8080/src/theme/img/cats/cat7.png".into()),
            code: "quiz2-1".into(),
            start_time: Some((Utc::now() + Duration::seconds(1)).timestamp_millis()),
            end_time: None,
            question_end_time: None,
            question_index: None,
            question_duration: 30 * 1000,
            status: QuizSessionStatus::Ready,
            created_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now(),
        },
        QuizSessionEntityDbProps {
            id: Uuid::from_str("29ac8d30-9489-495d-8d79-677c04a4a9b8").unwrap(),
            quiz_id: Uuid::from_str("efd3863f-a975-4b2d-9b03-eb3fe28410b9").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            host_name: "Host 6".into(),
            host_avatar: None,
            code: "quiz2-2".into(),
            start_time: Some((Utc::now() - Duration::minutes(120)).timestamp_millis()),
            end_time: None,
            question_end_time: None,
            question_index: None,
            question_duration: 30 * 1000,
            status: QuizSessionStatus::Canceled,
            created_at: Utc::now() - Duration::days(4),
            updated_at: Utc::now(),
        },
        QuizSessionEntityDbProps {
            id: Uuid::from_str("4053f5a5-6d54-42f7-83e5-52b7c3a300a1").unwrap(),
            quiz_id: Uuid::from_str("5db0613d-0077-48c5-a3f1-f0faefe2a083").unwrap(),
            user_id: Uuid::from_str("2213d9fc-3693-47ed-a495-cd5e7fc6dd0e").unwrap(),
            host_name: "Host 7".into(),
            host_avatar: None,
            code: "one-question".into(),
            start_time: Some((Utc::now() - Duration::minutes(2)).timestamp_millis()),
            end_time: None,
            question_end_time: Some((Utc::now() + Duration::seconds(1)).timestamp_millis()),
            question_index: Some(1),
            question_duration: 30 * 1000,
            status: QuizSessionStatus::Active,
            created_at: Utc::now() - Duration::days(5),
            updated_at: Utc::now(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
