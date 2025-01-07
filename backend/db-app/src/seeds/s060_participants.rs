use std::str::FromStr;

use chrono::{Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::entity::participant_entity::ParticipantEntity;
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "participants";

    let data: Vec<ParticipantEntity> = vec![
        ParticipantEntity {
            id: Uuid::from_str("9ee64573-c84f-439e-8cc7-a0b7b1f1906f").unwrap(),
            session_id: Uuid::from_str("ce782f8a-01bb-4228-9e50-4db0248f14cd").unwrap(),
            user_id: Some(Uuid::from_str("028ba9f2-f360-423b-83b6-44863b69e211").unwrap()),
            name: "S1P1".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat2.png".into(),
            points: 0,
            created_at: Utc::now(),
        },
        ParticipantEntity {
            id: Uuid::from_str("bf78d346-09b2-4512-b1fd-2413b6b68c5b").unwrap(),
            session_id: Uuid::from_str("ce782f8a-01bb-4228-9e50-4db0248f14cd").unwrap(),
            user_id: None,
            name: "S1P2".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat3.png".into(),
            points: 5,
            created_at: Utc::now() - Duration::hours(1),
        },
        ParticipantEntity {
            id: Uuid::from_str("ff9afb95-41fe-4cd8-b434-5bfe2cdcde58").unwrap(),
            session_id: Uuid::from_str("ce782f8a-01bb-4228-9e50-4db0248f14cd").unwrap(),
            user_id: None,
            name: "S1P3".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat4.png".into(),
            points: 3,
            created_at: Utc::now() - Duration::hours(2),
        },
        ParticipantEntity {
            id: Uuid::from_str("a7d74cb8-17a0-499d-b47d-0b8e1d1b3cdf").unwrap(),
            session_id: Uuid::from_str("dab9771d-e818-435f-98de-6a734189ba7d").unwrap(),
            user_id: None,
            name: "S2P1".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat5.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(3),
        },
        ParticipantEntity {
            id: Uuid::from_str("b6a25521-bc3b-4b05-974d-6b120ac8b467").unwrap(),
            session_id: Uuid::from_str("dab9771d-e818-435f-98de-6a734189ba7d").unwrap(),
            user_id: None,
            name: "S2P2".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat6.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(4),
        },
        ParticipantEntity {
            id: Uuid::from_str("b4ca8f1d-f737-4e7d-9e92-350ae24cfa53").unwrap(),
            session_id: Uuid::from_str("ffb8d80d-086f-4ce3-999b-ae9842afb564").unwrap(),
            user_id: None,
            name: "S3P1".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat7.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(5),
        },
        ParticipantEntity {
            id: Uuid::from_str("878d7365-d47f-44c9-8268-c4e38155338d").unwrap(),
            session_id: Uuid::from_str("ffb8d80d-086f-4ce3-999b-ae9842afb564").unwrap(),
            user_id: None,
            name: "S3P2".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(6),
        },
        ParticipantEntity {
            id: Uuid::from_str("9f6a4524-621f-49cd-b1b3-cef3a54ff8db").unwrap(),
            session_id: Uuid::from_str("ffb8d80d-086f-4ce3-999b-ae9842afb564").unwrap(),
            user_id: None,
            name: "S3P3".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(7),
        },
        ParticipantEntity {
            id: Uuid::from_str("2b74bcec-95f2-4423-9ac8-d9bacb054e8e").unwrap(),
            session_id: Uuid::from_str("ffb8d80d-086f-4ce3-999b-ae9842afb564").unwrap(),
            user_id: None,
            name: "S3P4".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(8),
        },
        ParticipantEntity {
            id: Uuid::from_str("7c885923-a458-48f5-b48e-d84c7e995930").unwrap(),
            session_id: Uuid::from_str("d12520fe-7f5a-46a7-9682-f138c7f81078").unwrap(),
            user_id: None,
            name: "S4P1".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(9),
        },
        ParticipantEntity {
            id: Uuid::from_str("3a659330-d7f8-4628-97f1-206dd5bef817").unwrap(),
            session_id: Uuid::from_str("d12520fe-7f5a-46a7-9682-f138c7f81078").unwrap(),
            user_id: None,
            name: "S4P2".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(10),
        },
        ParticipantEntity {
            id: Uuid::from_str("ea1fcc27-a573-4a1b-9604-73591d089e14").unwrap(),
            session_id: Uuid::from_str("29ac8d30-9489-495d-8d79-677c04a4a9b8").unwrap(),
            user_id: None,
            name: "S6P1".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(11),
        },
        ParticipantEntity {
            id: Uuid::from_str("70471877-bca7-48c8-b336-09b45eb15d05").unwrap(),
            session_id: Uuid::from_str("29ac8d30-9489-495d-8d79-677c04a4a9b8").unwrap(),
            user_id: None,
            name: "S6P2".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(12),
        },
        ParticipantEntity {
            id: Uuid::from_str("5393b80b-a1b4-4e95-8b67-5df37486c0f5").unwrap(),
            session_id: Uuid::from_str("4053f5a5-6d54-42f7-83e5-52b7c3a300a1").unwrap(),
            user_id: None,
            name: "S7P1".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(13),
        },
        ParticipantEntity {
            id: Uuid::from_str("48d00458-3d90-483e-830c-d6a3949cfe56").unwrap(),
            session_id: Uuid::from_str("4053f5a5-6d54-42f7-83e5-52b7c3a300a1").unwrap(),
            user_id: None,
            name: "S7P2".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(14),
        },
        ParticipantEntity {
            id: Uuid::from_str("fa633fc8-571a-4462-8fa8-19da088f0356").unwrap(),
            session_id: Uuid::from_str("4053f5a5-6d54-42f7-83e5-52b7c3a300a1").unwrap(),
            user_id: None,
            name: "S7P3".into(),
            avatar: "http://localhost:8080/src/theme/img/cats/cat8.png".into(),
            points: 0,
            created_at: Utc::now() - Duration::hours(15),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
