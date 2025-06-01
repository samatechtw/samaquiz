use std::sync::Arc;

use async_trait::async_trait;
use const_format::formatcp;
use lib_api::db::db_error::{map_sqlx_err, DbError};
use lib_types::{
    dto::quiz_response::create_quiz_response_dto::CreateQuizResponseResponse,
    entity::quiz_response_entity::QuizResponseEntity,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
use uuid::Uuid;

pub type DynQuizResponseRepo = Arc<dyn QuizResponseRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizResponseCreateProps {
    pub participant_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Uuid,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizResponseVerifyUniqueProps {
    pub participant_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Uuid,
}

#[async_trait]
pub trait QuizResponseRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_quiz_response(
        &self,
        props: QuizResponseCreateProps,
    ) -> Result<CreateQuizResponseResponse, DbError>;
    async fn verify_quiz_response_unique(
        &self,
        props: QuizResponseVerifyUniqueProps,
    ) -> Result<bool, DbError>;
}

pub struct QuizResponseRepo {
    pub db: PgPool,
}

const QUIZ_RESPONSE_COLUMNS: &str = formatcp!(
    r#"{p}.id, {p}.participant_id, {p}.question_id, {p}.answer_id, {p}.is_correct, {p}.created_at"#,
    p = "responses"
);

fn map_quiz_response_entity(row: PgRow) -> Result<QuizResponseEntity, sqlx::Error> {
    Ok(QuizResponseEntity {
        id: row.try_get("id")?,
        participant_id: row.try_get("participant_id")?,
        question_id: row.try_get("question_id")?,
        answer_id: row.try_get("answer_id")?,
        is_correct: row.try_get("is_correct")?,
        created_at: row.try_get("created_at")?,
    })
}

#[async_trait]
impl QuizResponseRepoTrait for QuizResponseRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_quiz_response(
        &self,
        props: QuizResponseCreateProps,
    ) -> Result<CreateQuizResponseResponse, DbError> {
        let row = sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"INSERT INTO "responses" (participant_id, question_id, answer_id, is_correct)
                values ($1, $2, $3, $4)
                RETURNING id
            "#
        ))
        .bind(props.participant_id)
        .bind(props.question_id)
        .bind(props.answer_id)
        .bind(props.is_correct)
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            _ => DbError::Query(e.to_string()),
        })?;
        let id = row.try_get("id")?;

        let count_row =
            sqlx::query("SELECT COUNT(*) as count FROM responses WHERE question_id = $1")
                .bind(props.question_id)
                .fetch_one(&self.db)
                .await
                .map_err(map_sqlx_err)?;

        let count = count_row.try_get("count")?;
        println!("COUNT {}, {}", props.question_id, count);

        Ok(CreateQuizResponseResponse {
            id,
            is_correct: props.is_correct,
            question_response_count: count,
        })
    }

    async fn verify_quiz_response_unique(
        &self,
        props: QuizResponseVerifyUniqueProps,
    ) -> Result<bool, DbError> {
        let result = sqlx::query(
            "SELECT id FROM responses WHERE participant_id = $1 AND question_id = $2 AND answer_id = $3"
        )
        .bind(props.participant_id)
        .bind(props.question_id)
        .bind(props.answer_id)
        .try_map(map_quiz_response_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err);

        match result {
            Ok(_) => Ok(false),
            Err(e) => match e {
                DbError::SqlxError(sqlx::Error::RowNotFound) => Ok(true),
                DbError::EntityNotFound() => Ok(true),
                _ => Err(e),
            },
        }
    }
}
