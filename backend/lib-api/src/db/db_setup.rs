use std::{
    str::FromStr,
    thread,
    time::{self, Duration},
};

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    query_scalar, PgPool,
};

use super::db_error::DbError;

fn database_exists_conn(url: &str, db_name: &str) -> Result<PgConnectOptions, DbError> {
    let options = PgConnectOptions::from_str(url)
        .map_err(|e| DbError::Create(e.to_string()))?
        .database(if db_name == "postgres" {
            "template1"
        } else {
            "postgres"
        });

    Ok(options)
}

async fn database_exists(url: &str, db_name: &str) -> Result<bool, DbError> {
    let options = database_exists_conn(url, db_name)?;

    // Use a pool for acquire_timeout
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .map_err(|e| DbError::Create(e.to_string()))?;

    let exists: bool = query_scalar("select exists(SELECT 1 from pg_database WHERE datname = $1)")
        .bind(db_name)
        .fetch_one(&pool)
        .await
        .map_err(|e| DbError::Create(e.to_string()))?;

    pool.close().await;

    Ok(exists)
}

pub async fn wait_db(db_url: &str, db_name: &str) -> bool {
    for _ in 0..60 {
        match database_exists(db_url, db_name).await {
            Ok(exists) => {
                if exists {
                    return true;
                }
                println!("Database not ready")
            }
            Err(e) => println!("Failed to connect db: {}", e.to_string()),
        };
        // This will wait 5 minutes at most
        thread::sleep(time::Duration::from_secs(5));
    }
    return false;
}

pub async fn app_db_connect(db_url: &str, db_name: &str) -> Result<PgPool, DbError> {
    // Wait for database to start
    if !wait_db(db_url, db_name).await {
        panic!("Database not found: {}", &db_url);
    }

    Ok(PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(db_url)
        .await
        .map_err(|e| DbError::Create(e.to_string()))?)
}
