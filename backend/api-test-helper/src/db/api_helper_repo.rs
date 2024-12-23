use std::sync::Arc;

use lib_api::db::db_error::DbError;
use lib_api::db::db_setup::app_db_connect;

use super::app_repo::{AppRepo, DynAppRepo};

#[derive(Clone)]
pub struct ApiHelperRepo {
    pub app: DynAppRepo,
}

pub async fn make_app_repo(db_url: &str, db_name: &str) -> Result<ApiHelperRepo, DbError> {
    let db_url = format!("{}{}", db_url, db_name);
    let db = app_db_connect(&db_url, db_name).await?;

    Ok(ApiHelperRepo {
        app: Arc::new(AppRepo { db }) as DynAppRepo,
    })
}
