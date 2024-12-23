use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 50))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub auth_token: String,
}
