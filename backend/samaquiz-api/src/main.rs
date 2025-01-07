use std::sync::Arc;

use axum::http::{header, HeaderValue, Method};
use axum::Router;
use clap::Parser;
use lib_api::clients::s3_client::S3Client;
use lib_api::util::log::{create_trace_layer, setup_logging};
use samaquiz_api::api_context::ApiContext;
use samaquiz_api::app::app_router::app_router;
use samaquiz_api::app::websocket::ws_state::init_ws_state;
use samaquiz_api::config::Config;
use samaquiz_api::db::app_repo::AppRepo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    sqlx::any::install_default_drivers();

    // Allows running from workspace root, or crate directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let _ = dotenvy::from_path(format!("{manifest_dir}/.env"));

    const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

    let config = Config::parse();
    let host = config.api_host.clone();
    let port = config.api_port.clone();

    let origins: Vec<HeaderValue> = config
        .api_cors
        .clone()
        .into_iter()
        .map(|origin| origin.parse::<HeaderValue>().unwrap())
        .collect();

    let api_url = format!("{}:{}", host, port);

    // Setup tracing
    let _guard = setup_logging(CRATE_NAME);

    let app_repo = AppRepo::new(&config.database_url, &config.db_name)
        .await
        .unwrap();

    let s3_client = S3Client::new(
        config.s3_url.clone(),
        config.s3_access_key_id.clone(),
        config.s3_secret_access_key.clone(),
    );

    let context = ApiContext {
        config: Arc::new(config),
        repo: app_repo,
        s3_client,
        ws_state: init_ws_state(),
    };

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin(origins)
        .allow_headers(vec![
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::HEAD,
            Method::OPTIONS,
            Method::DELETE,
        ]);

    // Run server
    let mut app = Router::new()
        .merge(app_router(&context))
        .with_state(context)
        .layer(ServiceBuilder::new().layer(cors));
    // Enables logging. Use `RUST_LOG=trace`
    app = create_trace_layer(app);

    let listener = tokio::net::TcpListener::bind(api_url).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
