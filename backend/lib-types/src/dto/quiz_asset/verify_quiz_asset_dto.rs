use serde::Serialize;

#[derive(Serialize)]
pub struct VerifyQuizAssetResponse {
    pub verified: bool,
}
