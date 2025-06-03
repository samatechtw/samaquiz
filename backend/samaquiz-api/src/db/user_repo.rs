use std::sync::Arc;

use async_trait::async_trait;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    db_result::list_result,
    password::hash,
    util::{append_and_eq, append_comma, append_limit_offset, option_enum_to_string},
};
use lib_types::{
    dto::user::{list_users_dto::ListUsersQuery, register_user_dto::RegisterUserDto},
    entity::user_entity::{UserCreateResult, UserEntity, UserListResults, UserUpdateParams},
    shared::user::{UserStatus, UserType},
};
use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

pub type DynUserRepo = Arc<dyn UserRepoTrait + Send + Sync>;

#[async_trait]
pub trait UserRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_user(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        dto: RegisterUserDto,
    ) -> Result<UserCreateResult, DbError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), DbError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<UserEntity, DbError>;
    async fn update_user(&self, id: Uuid, params: UserUpdateParams) -> Result<UserEntity, DbError>;
    async fn find_user_by_email(&self, email: String) -> Result<UserEntity, DbError>;
    async fn list_users(&self, query: ListUsersQuery) -> Result<UserListResults, DbError>;
}

pub struct UserRepo {
    pub db: PgPool,
}

const USER_COLUMNS: &str = formatcp!(
    r#"{u}.id, {u}.name, {u}.avatar, {u}.description, {u}.link, {u}.location, {u}.email, {u}.password_hash, {u}.created_at, {u}.updated_at,
{u}.user_type, {u}.user_status, {u}.email_confirmed"#,
    u = "users"
);

fn map_user_entity(row: PgRow) -> Result<UserEntity, sqlx::Error> {
    Ok(UserEntity {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        avatar: row.try_get("avatar")?,
        description: row.try_get("description")?,
        link: row.try_get("link")?,
        location: row.try_get("location")?,
        email: row.try_get("email")?,
        password_hash: row.try_get("password_hash")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        user_type: row.try_get_unchecked("user_type")?,
        user_status: row.try_get_unchecked("user_status")?,
        email_confirmed: row.try_get("email_confirmed")?,
    })
}

fn map_user_list_entity(row: PgRow) -> Result<(UserEntity, i64), sqlx::Error> {
    let count = row.try_get("count")?;
    let entity = map_user_entity(row)?;
    Ok((entity, count))
}

#[async_trait]
impl UserRepoTrait for UserRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_user(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        dto: RegisterUserDto,
    ) -> Result<UserCreateResult, DbError> {
        let password_hash = hash(dto.password).await?;

        Ok(sqlx::query(
            // language=PostgreSQL
            r#"
          INSERT INTO "users" (email, name, avatar, password_hash, user_type, user_status)
          values ($1, $2, $3, $4, $5, $6)
          RETURNING id
      "#,
        )
        .bind(dto.email.clone())
        .bind("")
        .bind("")
        .bind(password_hash)
        .bind(UserType::User.to_string())
        .bind(UserStatus::Active.to_string())
        .try_map(|row: PgRow| {
            Ok(UserCreateResult {
                id: row.try_get("id")?,
            })
        })
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_email_key") => {
                DbError::Unique("email".into())
            }
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_name_key") => {
                DbError::Unique("name".into())
            }
            _ => DbError::Query(e.to_string()),
        })?)
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DbError> {
        let result = sqlx::query(r#"DELETE FROM "users" WHERE id = $1"#)
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(DbError::EntityNotFound());
        }
        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM users WHERE id = $1",
            USER_COLUMNS
        ))
        .bind(id)
        .try_map(map_user_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err)?)
    }

    async fn update_user(&self, id: Uuid, params: UserUpdateParams) -> Result<UserEntity, DbError> {
        let query = QueryBuilder::new("UPDATE users SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "email", params.email, update_count);
        let (query, update_count) = append_comma(query, "name", params.name, update_count);
        let (query, update_count) = append_comma(query, "avatar", params.avatar, update_count);
        let (query, update_count) =
            append_comma(query, "description", params.description, update_count);
        let (query, update_count) = append_comma(query, "link", params.link, update_count);
        let (query, update_count) = append_comma(query, "location", params.location, update_count);

        let password_hash = match params.password {
            Some(p) => Some(hash(p).await?),
            _ => None,
        };
        let (query, update_count) =
            append_comma(query, "password_hash", password_hash, update_count);

        let (query, update_count) = if let Some(user_type) = params.user_type {
            append_comma(
                query,
                "user_type",
                Some(user_type.to_string()),
                update_count,
            )
        } else {
            (query, update_count)
        };

        let (query, update_count) = append_comma(
            query,
            "email_confirmed",
            params.email_confirmed,
            update_count,
        );

        let (mut query, update_count) = if let Some(user_status) = params.user_status {
            append_comma(
                query,
                "user_status",
                Some(user_status.to_string()),
                update_count,
            )
        } else {
            (query, update_count)
        };

        if update_count == 0 {
            return Err(DbError::NoUpdate);
        }

        query.push(" WHERE id = ");
        query.push_bind(id);
        query.push(formatcp!(" RETURNING {}", USER_COLUMNS));

        Ok(query
            .build()
            .try_map(map_user_entity)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::EntityNotFound(),
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_email_key") => {
                    DbError::Unique("email".into())
                }
                _ => DbError::Query(e.to_string()),
            })?)
    }

    async fn find_user_by_email(&self, email: String) -> Result<UserEntity, DbError> {
        let query = sqlx::query(formatcp!(
            r#"SELECT {}
                FROM users
                WHERE email = $1
            "#,
            USER_COLUMNS
        ))
        .bind(email);

        let user = query
            .try_map(map_user_entity)
            .fetch_one(&self.db)
            .await
            .map_err(map_sqlx_err)?;

        Ok(user)
    }

    async fn list_users(&self, query: ListUsersQuery) -> Result<UserListResults, DbError> {
        let filtered_query = if query.user_type.is_none() && query.status.is_none() {
            QueryBuilder::new("SELECT *, COUNT(*) OVER () FROM users")
        } else {
            QueryBuilder::new("SELECT *, COUNT(*) OVER () FROM users WHERE")
        };

        // Filter type
        let (filtered_query, count) = append_and_eq(
            filtered_query,
            "user_type",
            option_enum_to_string(query.user_type),
            0,
        );
        // Filter status
        let (mut filtered_query, _) = append_and_eq(
            filtered_query,
            "user_status",
            option_enum_to_string(query.status),
            count,
        );
        filtered_query = append_limit_offset(filtered_query, query.from, query.to);

        let results = filtered_query
            .build()
            .try_map(map_user_list_entity)
            .fetch_all(&self.db)
            .await?;

        let (results, total) = list_result(results);

        Ok(UserListResults { total, results })
    }
}
