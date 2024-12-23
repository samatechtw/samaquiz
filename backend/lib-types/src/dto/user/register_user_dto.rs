use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct RegisterUserDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 50))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: Uuid,
}
