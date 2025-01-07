use std::sync::Arc;

use async_trait::async_trait;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    util::append_comma,
};
use lib_types::{
    entity::quiz_session_entity::{QuizSessionEntity, QuizSessionEntityRelations},
    shared::quiz_session::QuizSessionStatus,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

use super::app_repo::start_transaction;

pub type DynQuizSessionRepo = Arc<dyn QuizSessionRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizSessionCreateProps {
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub host_name: String,
    pub host_avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizSessionUpdateProps {
    pub code: Option<String>,
    pub host_name: Option<String>,
    pub host_avatar: Option<String>,
    pub start_time: Option<i64>,
    pub question_end_time: Option<i64>,
    pub question_duration: Option<i64>,
    pub question_index: Option<i64>,
    pub end_time: Option<i64>,
    pub status: Option<QuizSessionStatus>,
}

#[async_trait]
pub trait QuizSessionRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_quiz_session(&self, props: QuizSessionCreateProps) -> Result<Uuid, DbError>;
    async fn update_quiz_session(
        &self,
        id: Uuid,
        props: QuizSessionUpdateProps,
    ) -> Result<(), DbError>;
    async fn update_quiz_session_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: QuizSessionUpdateProps,
    ) -> Result<(), DbError>;
    async fn get_quiz_session_by_id(&self, id: Uuid) -> Result<QuizSessionEntity, DbError>;
    async fn get_quiz_session_by_code(
        &self,
        code: String,
    ) -> Result<QuizSessionEntityRelations, DbError>;
    async fn get_quiz_session_relations_by_id(
        &self,
        id: Uuid,
    ) -> Result<QuizSessionEntityRelations, DbError>;
}

pub struct QuizSessionRepo {
    pub db: PgPool,
}

const QUIZ_SESSION_COLUMNS: &str = formatcp!(
    r#"{q}.id, {q}.quiz_id, {q}.user_id, {q}.code, {q}.host_name, {q}.host_avatar, {q}.start_time, {q}.end_time, {q}.question_end_time, {q}.question_index, {q}.question_duration, {q}.status, {q}.created_at, {q}.updated_at"#,
    q = "quiz_sessions"
);

const QUIZ_SESSION_RELATION_COLUMNS: &str =
    formatcp!(r#"{quiz_sessions}"#, quiz_sessions = QUIZ_SESSION_COLUMNS);

fn map_quiz_session_entity(row: PgRow) -> Result<QuizSessionEntity, sqlx::Error> {
    Ok(QuizSessionEntity {
        id: row.try_get("id")?,
        quiz_id: row.try_get("quiz_id")?,
        user_id: row.try_get("user_id")?,
        code: row.try_get("code")?,
        start_time: row.try_get("start_time")?,
        end_time: row.try_get("end_time")?,
        question_end_time: row.try_get("question_end_time")?,
        question_index: row.try_get("question_index")?,
        question_duration: row.try_get("question_duration")?,
        status: row.try_get_unchecked("status")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_quiz_session_relation_entity(
    row: &PgRow,
) -> Result<QuizSessionEntityRelations, sqlx::Error> {
    Ok(QuizSessionEntityRelations {
        id: row.try_get("id")?,
        quiz_id: row.try_get("quiz_id")?,
        user_id: row.try_get("user_id")?,
        code: row.try_get("code")?,
        host_name: row.try_get("host_name")?,
        host_avatar: row.try_get("host_avatar")?,
        start_time: row.try_get("start_time")?,
        end_time: row.try_get("end_time")?,
        question_end_time: row.try_get("question_end_time")?,
        question_index: row.try_get("question_index")?,
        question_duration: row.try_get("question_duration")?,
        status: row.try_get_unchecked("status")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

#[async_trait]
impl QuizSessionRepoTrait for QuizSessionRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_quiz_session(&self, props: QuizSessionCreateProps) -> Result<Uuid, DbError> {
        let row = sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"
              INSERT INTO "quiz_sessions" (quiz_id, user_id, code, host_name, host_avatar, status, question_duration)
              values ($1, $2, $3, $4, $5, $6, $7)
              RETURNING id
            "#
        ))
        .bind(props.quiz_id)
        .bind(props.user_id)
        .bind(props.code)
        .bind(props.host_name)
        .bind(props.host_avatar)
        .bind(QuizSessionStatus::Ready.to_string())
        .bind(30 * 1000) // question_duration = 30 seconds
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            _ => DbError::Query(e.to_string()),
        })?;
        Ok(row.try_get("id")?)
    }

    async fn update_quiz_session(
        &self,
        id: Uuid,
        props: QuizSessionUpdateProps,
    ) -> Result<(), DbError> {
        let mut tx = start_transaction(&self.db).await?;
        self.update_quiz_session_tx(&mut tx, id, props).await?;
        tx.commit().await.map_err(|e| DbError::SqlxError(e))?;
        Ok(())
    }

    async fn update_quiz_session_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: QuizSessionUpdateProps,
    ) -> Result<(), DbError> {
        let query = QueryBuilder::new("UPDATE quiz_sessions SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "code", props.code, update_count);
        let (query, update_count) = append_comma(query, "host_name", props.host_name, update_count);
        let (query, update_count) =
            append_comma(query, "host_avatar", props.host_avatar, update_count);
        let (query, update_count) =
            append_comma(query, "start_time", props.start_time, update_count);
        let (query, update_count) = append_comma(query, "end_time", props.end_time, update_count);
        let (query, update_count) = append_comma(
            query,
            "question_end_time",
            props.question_end_time,
            update_count,
        );
        let (query, update_count) =
            append_comma(query, "question_index", props.question_index, update_count);
        let (query, update_count) = append_comma(
            query,
            "question_duration",
            props.question_duration,
            update_count,
        );
        let (mut query, update_count) = append_comma(
            query,
            "status",
            props.status.and_then(|s| Some(s.to_string())),
            update_count,
        );

        if update_count == 0 {
            return Err(DbError::NoUpdate);
        }

        query.push(" WHERE id = ");
        query.push_bind(id);

        query
            .build()
            .execute(tx.as_mut())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::EntityNotFound(),
                _ => DbError::Query(e.to_string()),
            })?;

        Ok(())
    }

    async fn get_quiz_session_by_id(&self, id: Uuid) -> Result<QuizSessionEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM quiz_sessions
            WHERE quiz_sessions.id = $1",
            QUIZ_SESSION_COLUMNS
        ))
        .bind(id)
        .try_map(map_quiz_session_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err)?)
    }

    async fn get_quiz_session_by_code(
        &self,
        code: String,
    ) -> Result<QuizSessionEntityRelations, DbError> {
        let rows = sqlx::query(formatcp!(
            "SELECT {} FROM quiz_sessions
            WHERE quiz_sessions.code = $1",
            QUIZ_SESSION_COLUMNS
        ))
        .bind(code)
        .fetch_all(&self.db)
        .await
        .map_err(map_sqlx_err)?;

        let first = rows.iter().nth(0).ok_or(DbError::EntityNotFound())?;
        let quiz_session = map_quiz_session_relation_entity(first).map_err(map_sqlx_err)?;
        Ok(quiz_session)
    }

    async fn get_quiz_session_relations_by_id(
        &self,
        id: Uuid,
    ) -> Result<QuizSessionEntityRelations, DbError> {
        let rows = sqlx::query(&format!(
            r#"SELECT {} FROM "quiz_sessions"
        WHERE quiz_sessions.id = $1"#,
            QUIZ_SESSION_RELATION_COLUMNS
        ))
        .bind(id)
        .fetch_all(&self.db)
        .await
        .map_err(map_sqlx_err)?;

        let first = rows.iter().nth(0).ok_or(DbError::EntityNotFound())?;
        let quiz_session = map_quiz_session_relation_entity(first).map_err(map_sqlx_err)?;
        Ok(quiz_session)
    }
}
