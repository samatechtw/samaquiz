use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::asset::{AssetContentType, AssetState};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizAssetEntity {
    pub id: Uuid,
    pub size: i64,
    pub content_type: AssetContentType,
    pub state: AssetState,
    pub user_id: Uuid,
    pub quiz_id: Uuid,
    pub upload_expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl QuizAssetEntity {
    pub fn relative_url(&self) -> String {
        let ext = self.content_type.get_ext();
        let container = self.user_id.to_string();

        format!("{}/{}.{}", container, self.id.to_string(), ext)
    }
}

#[derive(Debug)]
pub struct QuizAssetListResults {
    pub total: i64,
    pub total_usage: u64,
    pub results: Vec<QuizAssetEntity>,
}
