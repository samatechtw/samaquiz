use std::sync::Arc;

use async_trait::async_trait;
use lib_api::db::db_error::DbError;
use sqlx::PgPool;

pub type DynAppRepo = Arc<dyn AppRepoTrait + Send + Sync>;

#[async_trait]
pub trait AppRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn reset(&self) -> Result<(), DbError>;
}

pub struct AppRepo {
    pub db: PgPool,
}

#[async_trait]
impl AppRepoTrait for AppRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn reset(&self) -> Result<(), DbError> {
        let tables = vec!["users"];

        for table in tables.iter() {
            let result = sqlx::query(&format!(
                r#"TRUNCATE TABLE "{}" RESTART IDENTITY CASCADE"#,
                table
            ))
            .execute(&self.db)
            .await;
            if let Err(e) = result {
                return Err(DbError::Query(e.to_string()));
            }
        }
        Ok(())
    }
}
