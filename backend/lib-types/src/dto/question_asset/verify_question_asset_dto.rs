use serde::Serialize;

#[derive(Serialize)]
pub struct VerifyQuestionAssetResponse {
    pub verified: bool,
}
