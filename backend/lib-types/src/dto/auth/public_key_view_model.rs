use serde::Serialize;

#[derive(Serialize)]
pub struct PublicKey {
    pub public_key: String,
}
