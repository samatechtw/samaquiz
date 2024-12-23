use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdatePasswordDto {
    #[validate(length(min = 8, max = 50))]
    pub password: String,
}

pub type UpdatePasswordResponse = ();
