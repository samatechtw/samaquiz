use std::sync::Arc;

use async_trait::async_trait;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    db_result::list_result,
    util::{append_comma, append_limit_offset, append_op, append_order_by, DbOp},
};
use lib_types::{
    dto::{
        participant::{
            create_participant_dto::CreateParticipantResponse,
            list_participants_dto::{ListParticipantsQuery, ParticipantSortColumn},
        },
        quiz_session::{
            get_quiz_session_leaders::{GetQuizSessionLeadersResponse, QuizSessionLeader},
            get_quiz_session_participant_count_dto::GetQuizSessionParticipantCountResponse,
        },
        sort_direction::SortDirection,
    },
    entity::participant_entity::{
        ParticipantEntity, ParticipantEntityRelations, ParticipantListResults,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::{postgres::PgRow, PgPool, QueryBuilder, Row};
use uuid::Uuid;

pub type DynParticipantRepo = Arc<dyn ParticipantRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ParticipantCreateProps {
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ParticipantUpdateProps {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub points: Option<i32>,
    pub user_id: Option<Uuid>,
}

impl ParticipantUpdateProps {
    pub fn points(points: i32) -> ParticipantUpdateProps {
        Self {
            name: None,
            avatar: None,
            points: Some(points),
            user_id: None,
        }
    }
}

#[async_trait]
pub trait ParticipantRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_participant(
        &self,
        props: ParticipantCreateProps,
    ) -> Result<CreateParticipantResponse, DbError>;
    async fn update_participant(
        &self,
        id: Uuid,
        props: ParticipantUpdateProps,
    ) -> Result<(), DbError>;
    async fn get_participant_by_id(&self, id: Uuid) -> Result<ParticipantEntity, DbError>;
    async fn get_participant_relations_by_id(
        &self,
        id: Uuid,
    ) -> Result<ParticipantEntityRelations, DbError>;
    async fn list_participants(
        &self,
        query: ListParticipantsQuery,
    ) -> Result<ParticipantListResults, DbError>;
    async fn get_participant_count(
        &self,
        id: Uuid,
    ) -> Result<GetQuizSessionParticipantCountResponse, DbError>;
    async fn get_session_leaders(&self, id: Uuid)
        -> Result<GetQuizSessionLeadersResponse, DbError>;
}

pub struct ParticipantRepo {
    pub db: PgPool,
}

const PARTICIPANT_COLUMNS: &str = formatcp!(
    r#"{p}.id, {p}.session_id, {p}.user_id, {p}.name, {p}.avatar, {p}.points, {p}.created_at"#,
    p = "participants"
);

fn map_participant_entity(row: PgRow) -> Result<ParticipantEntity, sqlx::Error> {
    Ok(ParticipantEntity {
        id: row.try_get("id")?,
        session_id: row.try_get("session_id")?,
        user_id: row.try_get("user_id")?,
        name: row.try_get("name")?,
        avatar: row.try_get("avatar")?,
        points: row.try_get("points")?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_participant_relations_entity(row: PgRow) -> Result<ParticipantEntityRelations, sqlx::Error> {
    Ok(ParticipantEntityRelations {
        id: row.try_get("id")?,
        session_id: row.try_get("session_id")?,
        user_id: row.try_get("user_id")?,
        name: row.try_get("name")?,
        avatar: row.try_get("avatar")?,
        points: row.try_get("points")?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_participant_list_entity(row: PgRow) -> Result<(ParticipantEntity, i64), sqlx::Error> {
    let count = row.try_get("count")?;
    let entity = map_participant_entity(row)?;
    Ok((entity, count))
}

#[async_trait]
impl ParticipantRepoTrait for ParticipantRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_participant(
        &self,
        props: ParticipantCreateProps,
    ) -> Result<CreateParticipantResponse, DbError> {
        let row = sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"INSERT INTO "participants" (session_id, user_id, name, avatar)
                values ($1, $2, $3, $4)
                RETURNING id
            "#
        ))
        .bind(props.session_id)
        .bind(props.user_id)
        .bind(props.name)
        .bind(props.avatar)
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            _ => DbError::Query(e.to_string()),
        })?;
        let id = row.try_get("id")?;

        let count_row =
            sqlx::query("SELECT COUNT(*) as count FROM participants WHERE session_id = $1")
                .bind(props.session_id)
                .fetch_one(&self.db)
                .await
                .map_err(map_sqlx_err)?;

        let count = count_row.try_get("count")?;

        Ok(CreateParticipantResponse { id, count })
    }

    async fn update_participant(
        &self,
        id: Uuid,
        props: ParticipantUpdateProps,
    ) -> Result<(), DbError> {
        let query = QueryBuilder::new("UPDATE participants SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "user_id", props.user_id, update_count);
        let (query, update_count) = append_comma(query, "name", props.name, update_count);

        let (query, update_count) = append_comma(query, "avatar", props.avatar, update_count);
        let (mut query, update_count) = append_comma(query, "points", props.points, update_count);

        if update_count == 0 {
            return Err(DbError::NoUpdate);
        }

        query.push(" WHERE id = ");
        query.push_bind(id);

        query.build().execute(&self.db).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::EntityNotFound(),
            _ => DbError::Query(e.to_string()),
        })?;

        Ok(())
    }

    async fn get_participant_by_id(&self, id: Uuid) -> Result<ParticipantEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM participants WHERE id = $1",
            PARTICIPANT_COLUMNS
        ))
        .bind(id)
        .try_map(map_participant_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err)?)
    }

    async fn get_participant_relations_by_id(
        &self,
        id: Uuid,
    ) -> Result<ParticipantEntityRelations, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM participants WHERE id = $1",
            PARTICIPANT_COLUMNS
        ))
        .bind(id)
        .try_map(map_participant_relations_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err)?)
    }

    async fn list_participants(
        &self,
        query: ListParticipantsQuery,
    ) -> Result<ParticipantListResults, DbError> {
        let mut filtered_query = QueryBuilder::new(format!(
            "SELECT {}, COUNT(participants.id) OVER () FROM \"participants\"",
            PARTICIPANT_COLUMNS
        ));

        if query.session_id.is_some() || query.user_id.is_some() {
            filtered_query.push(" WHERE");
        }
        let count = 0;

        // Filter user_id
        let (filtered_query, _) = if let Some(user_id) = query.user_id {
            let (mut q, c) = append_op(filtered_query, DbOp::And, count);
            q.push(" participants.user_id::text = ");
            q.push_bind(user_id);
            (q, c)
        } else {
            (filtered_query, count)
        };
        // Filter session_id
        let (mut filtered_query, _) = if let Some(session_id) = query.session_id {
            let (mut q, c) = append_op(filtered_query, DbOp::And, count);
            q.push(" participants.session_id::text = ");
            q.push_bind(session_id);
            (q, c)
        } else {
            (filtered_query, count)
        };
        // ORDER BY
        let column = to_string(&query.column.unwrap_or(ParticipantSortColumn::CreatedAt))
            .map_err(|e| DbError::Serialize(e.to_string()))?;
        let direction = query.direction.unwrap_or(SortDirection::Desc);

        filtered_query = append_order_by(filtered_query, column, direction.to_string());
        filtered_query = append_limit_offset(filtered_query, query.from, query.to);

        let results = filtered_query
            .build()
            .try_map(map_participant_list_entity)
            .fetch_all(&self.db)
            .await?;

        let (results, total) = list_result(results);

        Ok(ParticipantListResults { total, results })
    }

    async fn get_participant_count(
        &self,
        id: Uuid,
    ) -> Result<GetQuizSessionParticipantCountResponse, DbError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM participants WHERE session_id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(map_sqlx_err)?;

        Ok(GetQuizSessionParticipantCountResponse {
            count: row.try_get("count")?,
        })
    }

    async fn get_session_leaders(
        &self,
        session_id: Uuid,
    ) -> Result<GetQuizSessionLeadersResponse, DbError> {
        let rows = sqlx::query(formatcp!(
            "SELECT {} FROM participants WHERE session_id = $1 ORDER BY points DESC LIMIT 20",
            PARTICIPANT_COLUMNS
        ))
        .bind(session_id)
        .fetch_all(&self.db)
        .await
        .map_err(map_sqlx_err)?;

        let mut leaders: Vec<QuizSessionLeader> = vec![];
        for row in rows.iter() {
            leaders.push(QuizSessionLeader {
                name: row.try_get("name")?,
                points: row.try_get("points")?,
                avatar: row.try_get("avatar")?,
            });
        }

        Ok(GetQuizSessionLeadersResponse { leaders })
    }
}
