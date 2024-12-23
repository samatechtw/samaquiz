use crate::api_context::ApiContext;
use axum::{routing::post, Router};

use super::reset_db_app;

pub fn app_router() -> Router<ApiContext> {
    Router::new().route("/actions/reset/db-app", post(reset_db_app::reset))
}
