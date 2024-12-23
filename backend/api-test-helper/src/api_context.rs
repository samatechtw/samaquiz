use crate::{config::Config, db::api_helper_repo::ApiHelperRepo};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub repo: ApiHelperRepo,
}
