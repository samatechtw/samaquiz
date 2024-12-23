use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ResetPasswordDto {
    #[validate(email)]
    pub email: String,
}

pub type ResetPasswordResponse = ();
