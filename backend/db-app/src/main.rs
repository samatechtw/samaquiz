use std::{env, time::Duration};

use db_app::{db_setup::db_setup, seeds::seed_all};
use lib_api::db::db_error::DbError;
use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), DbError> {
    sqlx::any::install_default_drivers();

    // Allows running from workspace root, or crate directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let _ = dotenvy::from_path(format!("{manifest_dir}/.env"));
    let db_url = env::var("DATABASE_URL").expect("Missing env DATABASE_URL");
    let db_name = env::var("DB_NAME").expect("Missing env DB_NAME");
    let exec_env = env::var("EXEC_ENV").expect("Missing env EXEC_ENV");

    let exists = db_setup(&exec_env, &db_url, &db_name).await.unwrap();

    let db_result = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&format!("{}{}", db_url, db_name))
        .await;

    if let Ok(db) = db_result {
        println!("[INFO] Migrate database...");
        sqlx::migrate!().run(&db).await.unwrap();

        // Seed database if the database was created above
        if exists {
            println!("[INFO] Skipped seeding database")
        } else {
            seed_all(&db).await.unwrap();
            println!("[INFO] Seeded app database");
        }
    } else {
        println!("[db-app] Failed to connect to database!");
    }
    // Sleep forever to avoid container restart
    loop {
        println!("[INFO] Sleeping for a few years");
        sleep(Duration::from_millis(68719476734)).await;
    }
}
