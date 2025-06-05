use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    db_result::list_result,
    util::{append_comma, append_in, append_limit_offset, append_op, append_order_by, DbOp},
};
use lib_types::{
    dto::{
        quiz::list_quizzes_dto::{ListQuizzesQuery, QuizSortColumn},
        sort_direction::SortDirection,
    },
    entity::{
        question_entity::QuestionEntity,
        quiz_entity::{QuizEntity, QuizEntityRelations, QuizListResults},
        quiz_session_entity::QuizSessionEntity,
    },
    shared::quiz::QuizType,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

use super::app_repo::start_transaction;

pub type DynQuizRepo = Arc<dyn QuizRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizCreateProps {
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub quiz_type: QuizType,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct QuizUpdateProps {
    pub title: Option<String>,
    pub description: Option<String>,
    pub questions_order: Option<Vec<String>>,
    pub intro_background_url: Option<String>,
}

impl QuizUpdateProps {
    pub fn questions_order(order: Vec<String>) -> Self {
        Self {
            title: None,
            description: None,
            questions_order: Some(order),
            intro_background_url: None,
        }
    }
}

#[async_trait]
pub trait QuizRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_quiz(&self, props: QuizCreateProps) -> Result<QuizEntity, DbError>;
    async fn update_quiz(&self, id: Uuid, props: QuizUpdateProps) -> Result<QuizEntity, DbError>;
    async fn update_quiz_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: QuizUpdateProps,
    ) -> Result<QuizEntity, DbError>;
    async fn get_quiz_by_id(&self, id: Uuid) -> Result<QuizEntity, DbError>;
    async fn get_quiz_relations_by_id(&self, id: Uuid) -> Result<QuizEntityRelations, DbError>;
    async fn list_quizzes(&self, query: ListQuizzesQuery) -> Result<QuizListResults, DbError>;
    async fn delete_quiz_by_id(&self, id: Uuid) -> Result<(), DbError>;
}

pub struct QuizRepo {
    pub db: PgPool,
}

const QUIZ_COLUMNS: &str = formatcp!(
    r#"{q}.id, {q}.user_id, {q}.title, {q}.description, {q}.quiz_type, {q}.questions_order, {q}.intro_background_url, {q}.created_at, {q}.updated_at"#,
    q = "quizzes"
);

const QUIZ_RELATION_COLUMNS: &str = formatcp!(
    r#"{quizzes}, {qu}.id as qu_id, {qu}.text as qu_text, {qu}.question_type as qu_question_type, {qu}.answers_order as qu_answers_order, {qu}.asset_url as qu_asset_url, {qu}.created_at as qu_created_at, {qu}.updated_at as qu_updated_at, {s}.id as s_id, {s}.code as s_code, {s}.start_time as s_start_time, {s}.end_time as s_end_time, {s}.question_end_time as s_question_end_time, {s}.question_index as s_question_index, {s}.question_duration as s_question_duration, {s}.status as s_status, {s}.created_at as s_created_at, {s}.updated_at as s_updated_at"#,
    quizzes = QUIZ_COLUMNS,
    qu = "qu",
    s = "s",
);

fn map_quiz_entity(row: PgRow) -> Result<QuizEntity, sqlx::Error> {
    Ok(QuizEntity {
        id: row.try_get("id")?,
        user_id: row.try_get("user_id")?,
        title: row.try_get("title")?,
        description: row.try_get("description")?,
        quiz_type: row.try_get_unchecked("quiz_type")?,
        questions_order: row.try_get("questions_order")?,
        intro_background_url: row.try_get("intro_background_url")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_quiz_relation_entity(
    row: &PgRow,
    questions: Vec<QuestionEntity>,
    sessions: Vec<QuizSessionEntity>,
) -> Result<QuizEntityRelations, sqlx::Error> {
    let quiz_id: Uuid = row.try_get("id")?;
    Ok(QuizEntityRelations {
        id: quiz_id,
        user_id: row.try_get("user_id")?,
        title: row.try_get("title")?,
        description: row.try_get("description")?,
        quiz_type: row.try_get_unchecked("quiz_type")?,
        questions_order: row.try_get("questions_order")?,
        questions,
        sessions,
        intro_background_url: row.try_get("intro_background_url")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_quiz_list_entity(row: PgRow) -> Result<(QuizEntity, i64), sqlx::Error> {
    let count = row.try_get("count")?;
    let entity = map_quiz_entity(row)?;
    Ok((entity, count))
}

#[async_trait]
impl QuizRepoTrait for QuizRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_quiz(&self, props: QuizCreateProps) -> Result<QuizEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"
              INSERT INTO "quizzes" (user_id, title, description, quiz_type, intro_background_url)
              values ($1, $2, $3, $4, $5)
              RETURNING {}
            "#,
            QUIZ_COLUMNS
        ))
        .bind(props.user_id)
        .bind(props.title)
        .bind(props.description)
        .bind(props.quiz_type.to_string())
        .bind("".to_string())
        .try_map(map_quiz_entity)
        .fetch_one(&self.db)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?)
    }

    async fn update_quiz(&self, id: Uuid, props: QuizUpdateProps) -> Result<QuizEntity, DbError> {
        let mut tx = start_transaction(&self.db).await?;
        let result = self.update_quiz_tx(&mut tx, id, props).await?;
        tx.commit().await.map_err(|e| DbError::SqlxError(e))?;
        Ok(result)
    }

    async fn update_quiz_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: QuizUpdateProps,
    ) -> Result<QuizEntity, DbError> {
        let query = QueryBuilder::new("UPDATE quizzes SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "title", props.title, update_count);
        let (query, update_count) =
            append_comma(query, "description", props.description, update_count);
        let (query, update_count) = append_comma(
            query,
            "intro_background_url",
            props.intro_background_url,
            update_count,
        );
        let (mut query, update_count) = append_comma(
            query,
            "questions_order",
            props.questions_order,
            update_count,
        );

        if update_count == 0 {
            return Err(DbError::NoUpdate);
        }

        query.push(" WHERE id = ");
        query.push_bind(id);
        query.push(formatcp!(" RETURNING {}", QUIZ_COLUMNS));

        Ok(query
            .build()
            .try_map(map_quiz_entity)
            .fetch_one(tx.as_mut())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::EntityNotFound(),
                _ => DbError::Query(e.to_string()),
            })?)
    }

    async fn get_quiz_by_id(&self, id: Uuid) -> Result<QuizEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM \"quizzes\" WHERE id = $1",
            QUIZ_COLUMNS
        ))
        .bind(id)
        .try_map(map_quiz_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err)?)
    }

    async fn get_quiz_relations_by_id(&self, id: Uuid) -> Result<QuizEntityRelations, DbError> {
        let rows = sqlx::query(&format!(
            r#"SELECT {} FROM "quizzes"
            LEFT OUTER JOIN questions qu on qu.quiz_id = quizzes.id
            LEFT OUTER JOIN quiz_sessions s on s.quiz_id = quizzes.id
            WHERE quizzes.id = $1"#,
            QUIZ_RELATION_COLUMNS
        ))
        .bind(id)
        .fetch_all(&self.db)
        .await
        .map_err(map_sqlx_err)?;

        let first = rows.iter().nth(0).ok_or(DbError::EntityNotFound())?;
        let mut questions: Vec<QuestionEntity> = vec![];
        let mut sessions: Vec<QuizSessionEntity> = vec![];
        let mut used_ids: HashSet<Uuid> = HashSet::new();
        for row in rows.iter() {
            if let Some(question_id) = row.try_get::<Option<Uuid>, &str>("qu_id")? {
                if !used_ids.contains(&question_id) {
                    used_ids.insert(question_id.clone());
                    questions.push(QuestionEntity {
                        id: question_id,
                        quiz_id: id,
                        text: row.try_get("qu_text")?,
                        asset_url: row.try_get("qu_asset_url")?,
                        question_type: row.try_get_unchecked("qu_question_type")?,
                        answers_order: row.try_get("qu_answers_order")?,
                        created_at: row.try_get("qu_created_at")?,
                        updated_at: row.try_get("qu_updated_at")?,
                    });
                }
            }
            if let Some(session_id) = row.try_get::<Option<Uuid>, &str>("s_id")? {
                if !used_ids.contains(&session_id) {
                    used_ids.insert(session_id.clone());
                    sessions.push(QuizSessionEntity {
                        id: session_id,
                        quiz_id: id,
                        user_id: row.try_get("user_id")?,
                        code: row.try_get("s_code")?,
                        start_time: row.try_get("s_start_time")?,
                        end_time: row.try_get("s_end_time")?,
                        question_end_time: row.try_get("s_question_end_time")?,
                        question_index: row.try_get("s_question_index")?,
                        question_duration: row.try_get("s_question_duration")?,
                        status: row.try_get_unchecked("s_status")?,
                        created_at: row.try_get("s_created_at")?,
                        updated_at: row.try_get("s_updated_at")?,
                    });
                }
            }
        }
        let quiz = map_quiz_relation_entity(first, questions, sessions).map_err(map_sqlx_err)?;
        Ok(quiz)
    }

    async fn list_quizzes(&self, query: ListQuizzesQuery) -> Result<QuizListResults, DbError> {
        let mut filtered_query = QueryBuilder::new(format!(
            "SELECT {}, COUNT(quizzes.id) OVER () FROM \"quizzes\"",
            QUIZ_COLUMNS
        ));

        if query.types.is_some() || query.user_id.is_some() {
            filtered_query.push(" WHERE");
        }

        // Filter quiz_type
        let (filtered_query, count) = append_in(filtered_query, "quiz_type", query.types, 0);
        // Filter user_id
        let (mut filtered_query, _) = if let Some(user_id) = query.user_id {
            let (mut q, c) = append_op(filtered_query, DbOp::And, count);
            q.push(" quizzes.user_id::text = ");
            q.push_bind(user_id);
            (q, c)
        } else {
            (filtered_query, count)
        };
        // ORDER BY
        let column = to_string(&query.column.unwrap_or(QuizSortColumn::CreatedAt))
            .map_err(|e| DbError::Serialize(e.to_string()))?;
        let direction = query.direction.unwrap_or(SortDirection::Desc);

        filtered_query = append_order_by(filtered_query, column, direction.to_string());
        filtered_query = append_limit_offset(filtered_query, query.from, query.to);

        let results = filtered_query
            .build()
            .try_map(map_quiz_list_entity)
            .fetch_all(&self.db)
            .await?;

        let (results, total) = list_result(results);

        Ok(QuizListResults { total, results })
    }

    async fn delete_quiz_by_id(&self, id: Uuid) -> Result<(), DbError> {
        sqlx::query(r#"DELETE FROM "quizzes" WHERE id = $1"#)
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
