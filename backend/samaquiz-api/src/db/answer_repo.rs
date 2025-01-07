use std::sync::Arc;

use async_trait::async_trait;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    util::append_comma,
};
use lib_types::entity::answer_entity::AnswerEntity;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Acquire, Executor, PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

use super::app_repo::start_transaction;

pub type DynAnswerRepo = Arc<dyn AnswerRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct AnswerCreateProps {
    pub question_id: Uuid,
    pub text: String,
    pub points: i32,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct AnswerUpdateProps {
    pub text: Option<String>,
    pub points: Option<i32>,
}

async fn create_answer_helper<'a, T>(e: T, props: AnswerCreateProps) -> Result<Uuid, DbError>
where
    T: Executor<'a, Database = Postgres>,
{
    let row = sqlx::query(formatcp!(
        // language=PostgreSQL
        r#"
              INSERT INTO "answers" (question_id, text, points)
              values ($1, $2, $3)
              RETURNING id
            "#
    ))
    .bind(props.question_id)
    .bind(props.text)
    .bind(props.points)
    .fetch_one(e)
    .await
    .map_err(|e| match e {
        _ => DbError::Query(e.to_string()),
    })?;
    Ok(row.try_get("id")?)
}

#[async_trait]
pub trait AnswerRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_answer(&self, props: AnswerCreateProps) -> Result<Uuid, DbError>;
    async fn create_answer_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: AnswerCreateProps,
    ) -> Result<Uuid, DbError>;
    async fn update_answer(&self, id: Uuid, props: AnswerUpdateProps) -> Result<(), DbError>;
    async fn update_answer_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: AnswerUpdateProps,
    ) -> Result<(), DbError>;
    async fn update_answer_correct_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        question_id: Uuid,
        id: Uuid,
        is_correct: bool,
    ) -> Result<(), DbError>;
    async fn get_answer_by_id(&self, id: Uuid) -> Result<AnswerEntity, DbError>;
    async fn delete_answer_by_id_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<(), DbError>;
}

pub struct AnswerRepo {
    pub db: PgPool,
}

const ANSWER_COLUMNS: &str = formatcp!(
    r#"{a}.id, {a}.question_id, {a}.text, {a}.is_correct, {a}.points, {a}.created_at, {a}.updated_at"#,
    a = "answers"
);

fn map_answer_entity(row: PgRow) -> Result<AnswerEntity, sqlx::Error> {
    Ok(AnswerEntity {
        id: row.try_get("id")?,
        question_id: row.try_get("question_id")?,
        text: row.try_get("text")?,
        points: row.try_get("points")?,
        is_correct: row.try_get("is_correct")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

#[async_trait]
impl AnswerRepoTrait for AnswerRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_answer_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: AnswerCreateProps,
    ) -> Result<Uuid, DbError> {
        let conn = tx.acquire().await?;
        create_answer_helper(conn, props).await
    }

    async fn create_answer(&self, props: AnswerCreateProps) -> Result<Uuid, DbError> {
        create_answer_helper(&self.db, props).await
    }

    async fn update_answer(&self, id: Uuid, props: AnswerUpdateProps) -> Result<(), DbError> {
        let mut tx = start_transaction(&self.db).await?;
        self.update_answer_tx(&mut tx, id, props).await?;
        tx.commit().await.map_err(|e| DbError::SqlxError(e))?;
        Ok(())
    }

    async fn update_answer_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: AnswerUpdateProps,
    ) -> Result<(), DbError> {
        let query = QueryBuilder::new("UPDATE answers SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "text", props.text, update_count);

        let (mut query, update_count) = append_comma(query, "points", props.points, update_count);

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

    async fn update_answer_correct_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        question_id: Uuid,
        id: Uuid,
        is_correct: bool,
    ) -> Result<(), DbError> {
        sqlx::query(&format!(
            r#"UPDATE answers
            SET is_correct = CASE
                WHEN id = $1 THEN {}
                ELSE {}
            END
            WHERE question_id = $2"#,
            is_correct.to_string(),
            (!is_correct).to_string(),
        ))
        .bind(id)
        .bind(question_id)
        .execute(tx.as_mut())
        .await
        .map_err(map_sqlx_err)?;
        Ok(())
    }

    async fn get_answer_by_id(&self, id: Uuid) -> Result<AnswerEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM answers WHERE id = $1",
            ANSWER_COLUMNS
        ))
        .bind(id)
        .try_map(map_answer_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err)?)
    }

    async fn delete_answer_by_id_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<(), DbError> {
        sqlx::query(r#"DELETE FROM "answers" WHERE id = $1"#)
            .bind(id)
            .execute(tx.as_mut())
            .await?;

        Ok(())
    }
}
