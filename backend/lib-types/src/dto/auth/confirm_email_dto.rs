use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmEmailDto {
    pub code: String,
}
