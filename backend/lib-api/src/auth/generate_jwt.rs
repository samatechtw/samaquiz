use std::str::FromStr;

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use lib_types::shared::user::UserType;
use uuid::Uuid;

use crate::error::api_error::ApiError;

use super::types::{ConfirmClaims, JwtClaims, UserToken};

pub fn generate_admin_jwt(private_key: &str) -> Result<UserToken, ApiError> {
    generate_jwt(
        Uuid::from_str("083fb9af-a2fd-41b8-bfa2-4747cc87b604").unwrap(),
        UserType::Admin,
        52034400,
        private_key,
    )
}

pub fn generate_jwt(
    user_id: Uuid,
    user_type: UserType,
    // TTL in minutes
    ttl: i64,
    secret: &str,
) -> Result<UserToken, ApiError> {
    let now = Utc::now();
    let mut token_details = UserToken {
        user_id,
        user_type,
        expires_in: Some((now + Duration::minutes(ttl)).timestamp()),
        token: None,
    };

    let claims = JwtClaims {
        sub: token_details.user_id.to_string(),
        user_type: user_type.to_string(),
        exp: token_details.expires_in.ok_or(ApiError::internal_error())?,
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_base64_secret(secret)
            .map_err(|_| ApiError::internal_error())?,
    )
    .map_err(|_| ApiError::internal_error())?;
    token_details.token = Some(token);
    Ok(token_details)
}

pub fn generate_confirm_token(
    user_id: Uuid,
    // TTL in minutes
    ttl: i64,
    secret: String,
) -> Result<String, ApiError> {
    let key = &EncodingKey::from_secret(secret.as_ref());

    let claims = ConfirmClaims {
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::minutes(ttl)).timestamp(),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, key)
        .map_err(|_| ApiError::internal_error().message("Failed to encode confirm token"))?;

    Ok(token)
}
