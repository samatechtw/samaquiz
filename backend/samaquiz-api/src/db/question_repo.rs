use std::sync::Arc;

use async_trait::async_trait;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    util::append_comma,
};
use lib_types::{
    entity::{
        answer_entity::AnswerEntity,
        question_entity::{QuestionAssetEntityRelation, QuestionEntity, QuestionEntityRelations},
    },
    shared::question::QuestionType,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

use super::app_repo::start_transaction;

pub type DynQuestionRepo = Arc<dyn QuestionRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuestionCreateProps {
    pub quiz_id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuestionUpdateProps {
    pub text: Option<String>,
    pub question_type: Option<QuestionType>,
    pub answers_order: Option<Vec<String>>,
}

impl QuestionUpdateProps {
    pub fn answers_order(order: Vec<String>) -> Self {
        Self {
            text: None,
            question_type: None,
            answers_order: Some(order),
        }
    }
}

#[async_trait]
pub trait QuestionRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_question_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: QuestionCreateProps,
    ) -> Result<Uuid, DbError>;
    async fn update_question(&self, id: Uuid, props: QuestionUpdateProps) -> Result<(), DbError>;
    async fn update_question_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: QuestionUpdateProps,
    ) -> Result<(), DbError>;
    async fn get_question_by_id(&self, id: Uuid) -> Result<QuestionEntity, DbError>;
    async fn get_question_relations_by_id(
        &self,
        id: Uuid,
        all_assets: bool,
    ) -> Result<QuestionEntityRelations, DbError>;
    async fn delete_question_by_id_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<(), DbError>;
}

pub struct QuestionRepo {
    pub db: PgPool,
}

const QUESTION_COLUMNS: &str = formatcp!(
    r#"{q}.id, {q}.quiz_id, {q}.text, {q}.question_type, {q}.answers_order, {q}.created_at, {q}.updated_at, a.id as a_id, a.size a_size, a.content_type as a_content_type"#,
    q = "questions"
);

const QUESTION_RELATION_COLUMNS: &str = formatcp!(
    r#"{questions}, {an}.id as an_id, {an}.question_id as an_question_id, {an}.text as an_text, {an}.is_correct as an_is_correct, {an}.points as an_points, {an}.created_at as an_created_at, {an}.updated_at as an_updated_at"#,
    questions = QUESTION_COLUMNS,
    an = "an",
);

fn map_question_entity(row: PgRow) -> Result<QuestionEntity, sqlx::Error> {
    let question_id: Uuid = row.try_get("id")?;
    let asset = if let Ok(asset_id) = row.try_get::<Uuid, &str>("a_id") {
        Some(QuestionAssetEntityRelation {
            id: asset_id,
            size: row.try_get("a_size")?,
            content_type: row.try_get_unchecked("a_content_type")?,
            question_id: question_id.clone(),
        })
    } else {
        None
    };
    Ok(QuestionEntity {
        id: row.try_get("id")?,
        quiz_id: row.try_get("quiz_id")?,
        text: row.try_get("text")?,
        question_type: row.try_get_unchecked("question_type")?,
        answers_order: row.try_get("answers_order")?,
        asset,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_question_relation_entity(
    row: &PgRow,
    answers: Vec<AnswerEntity>,
) -> Result<QuestionEntityRelations, sqlx::Error> {
    let question_id: Uuid = row.try_get("id")?;
    let asset = if let Ok(asset_id) = row.try_get::<Uuid, &str>("a_id") {
        Some(QuestionAssetEntityRelation {
            id: asset_id,
            size: row.try_get("a_size")?,
            content_type: row.try_get_unchecked("a_content_type")?,
            question_id: question_id.clone(),
        })
    } else {
        None
    };
    Ok(QuestionEntityRelations {
        id: question_id,
        quiz_id: row.try_get("quiz_id")?,
        text: row.try_get("text")?,
        question_type: row.try_get_unchecked("question_type")?,
        answers_order: row.try_get("answers_order")?,
        answers,
        asset,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

#[async_trait]
impl QuestionRepoTrait for QuestionRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_question_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: QuestionCreateProps,
    ) -> Result<Uuid, DbError> {
        let row = sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"
              INSERT INTO "questions" (quiz_id, text, question_type)
              values ($1, $2, $3)
              RETURNING id
            "#
        ))
        .bind(props.quiz_id)
        .bind(props.text)
        .bind(props.question_type.to_string())
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e| match e {
            _ => DbError::Query(e.to_string()),
        })?;
        Ok(row.try_get("id")?)
    }

    async fn update_question(&self, id: Uuid, props: QuestionUpdateProps) -> Result<(), DbError> {
        let mut tx = start_transaction(&self.db).await?;
        self.update_question_tx(&mut tx, id, props).await?;
        tx.commit().await.map_err(|e| DbError::SqlxError(e))?;
        Ok(())
    }

    async fn update_question_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: QuestionUpdateProps,
    ) -> Result<(), DbError> {
        let query = QueryBuilder::new("UPDATE questions SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "text", props.text, update_count);
        let (query, update_count) = append_comma(
            query,
            "question_type",
            props.question_type.and_then(|s| Some(s.to_string())),
            update_count,
        );

        let (mut query, update_count) =
            append_comma(query, "answers_order", props.answers_order, update_count);

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

    async fn get_question_by_id(&self, id: Uuid) -> Result<QuestionEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM questions
                LEFT OUTER JOIN question_assets a on a.question_id = questions.id
            WHERE questions.id = $1",
            QUESTION_COLUMNS
        ))
        .bind(id)
        .try_map(map_question_entity)
        .fetch_one(&self.db)
        .await?)
    }

    async fn get_question_relations_by_id(
        &self,
        id: Uuid,
        all_assets: bool,
    ) -> Result<QuestionEntityRelations, DbError> {
        let asset_query = if all_assets {
            ""
        } else {
            " AND a.state = 'Uploaded'"
        }
        .to_string();

        let rows = sqlx::query(&format!(
            r#"SELECT {} FROM "questions"
        LEFT OUTER JOIN answers an on an.question_id = questions.id
        LEFT OUTER JOIN question_assets a on a.question_id = questions.id{}
        WHERE questions.id = $1"#,
            QUESTION_RELATION_COLUMNS, asset_query
        ))
        .bind(id)
        .fetch_all(&self.db)
        .await
        .map_err(map_sqlx_err)?;

        let first = rows.iter().nth(0).ok_or(DbError::EntityNotFound())?;
        let mut answers: Vec<AnswerEntity> = vec![];
        for row in rows.iter() {
            if row.try_get::<Option<Uuid>, &str>("an_id")?.is_none() {
                continue;
            }
            answers.push(AnswerEntity {
                id: row.try_get("an_id")?,
                question_id: row.try_get("an_question_id")?,
                text: row.try_get("an_text")?,
                is_correct: row.try_get("an_is_correct")?,
                points: row.try_get("an_points")?,
                created_at: row.try_get("an_created_at")?,
                updated_at: row.try_get("an_updated_at")?,
            });
        }
        let question = map_question_relation_entity(first, answers).map_err(map_sqlx_err)?;
        Ok(question)
    }

    async fn delete_question_by_id_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<(), DbError> {
        sqlx::query(r#"DELETE FROM "questions" WHERE id = $1"#)
            .bind(id)
            .execute(tx.as_mut())
            .await?;

        Ok(())
    }
}
