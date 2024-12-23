use lib_api::db::db_setup::wait_db;
use lib_api::db::db_error::DbError;
use lib_types::shared::core::ExecEnv;
use sqlx::PgPool;

async fn db_query_helper(pool: &PgPool, db_name: &str, query: &str) -> Result<(), sqlx::Error> {
    let query = str::replace(query, "$1", db_name);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn db_exists(pool: &PgPool, db_name: &str) -> bool {
    println!("[INFO] Check if app DB exists...");

    let result: Result<(i32,), _> =
        sqlx::query_as(r#"SELECT 1 FROM pg_database WHERE datname = 'db_app'"#)
            .bind(db_name)
            .fetch_one(pool)
            .await;

    match result {
        Ok((exists,)) => exists == 1,
        Err(_) => false,
    }
}

async fn db_reset(pool: &PgPool, db_name: &str) {
    println!("[INFO] Drop database {}...", db_name);

    let _ = db_query_helper(
        pool,
        db_name,
        "UPDATE pg_database SET datallowconn = 'false' WHERE datname = '$1'",
    )
    .await;
    let _ = db_query_helper(
        pool,
        db_name,
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$1'",
    )
    .await;
    let _ = db_query_helper(pool, db_name, "DROP DATABASE $1").await;
}

pub async fn db_setup(env_str: &str, db_url: &str, db_name: &str) -> Result<bool, DbError> {
    let env: ExecEnv = env_str
        .to_owned()
        .parse()
        .map_err(|e: strum::ParseError| DbError::Parse(e.to_string()))?;

    let setup_db_url = format!("{}postgres", db_url);

    // Wait for DB to exist
    wait_db(&setup_db_url, "postgres").await;

    let setup_conn = PgPool::connect(&setup_db_url).await?;
    let mut exists = db_exists(&setup_conn, db_name).await;

    if env == ExecEnv::Dev {
        db_reset(&setup_conn, db_name).await;
        exists = false;
    }
    if !exists {
        println!("[INFO] Create database {}...", db_name);
        db_query_helper(&setup_conn, db_name, "CREATE DATABASE $1")
            .await
            .unwrap();
    }

    println!("[INFO] Database is initialized");
    Ok(exists)
}
