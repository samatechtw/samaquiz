use std::sync::Arc;

use api_test_helper::{
    api_context::ApiContext, app::app_router::app_router, config::Config,
    db::api_helper_repo::make_app_repo,
};
use axum::Router;
use clap::Parser;
use dotenvy::dotenv;
use lib_api::util::log::{create_trace_layer, setup_logging};

#[tokio::main]
async fn main() {
    sqlx::any::install_default_drivers();
    let _ = dotenv();

    const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

    // Parse config
    let config = Config::parse();
    let host = config.api_host.clone();
    let port = config.api_port.clone();

    let api_url = format!("{}:{}", host, port);

    // Setup tracing
    let _guard = setup_logging(CRATE_NAME);

    let app_repo = make_app_repo(&config.database_url, &config.db_name)
        .await
        .unwrap();

    let context = ApiContext {
        config: Arc::new(config),
        repo: app_repo,
    };

    // Run server
    let mut app = Router::new().merge(app_router()).with_state(context);
    // Enables logging. Use `RUST_LOG=trace`
    app = create_trace_layer(app);

    let listener = tokio::net::TcpListener::bind(api_url).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
