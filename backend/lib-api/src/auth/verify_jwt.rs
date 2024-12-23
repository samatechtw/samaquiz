use std::str::FromStr;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lib_types::shared::{api_error::ApiErrorCode, user::UserType};
use uuid::Uuid;

use crate::error::api_error::ApiError;

use super::types::{ConfirmClaims, JwtClaims, UserToken};

fn unauthorized() -> ApiError {
    return ApiError::unauthorized().code(ApiErrorCode::InvalidAuth);
}

pub fn verify_jwt(secret: &str, token: &str) -> Result<UserToken, ApiError> {
    let validation = Validation::new(Algorithm::HS256);

    let key = jsonwebtoken::DecodingKey::from_base64_secret(secret)
        .map_err(|_| ApiError::internal_error().message("Auth misconfiguration"))?;
    let decoded =
        jsonwebtoken::decode::<JwtClaims>(token, &key, &validation).map_err(|_| unauthorized())?;

    let user_type = UserType::from_str(&decoded.claims.user_type).map_err(|_| unauthorized())?;

    let user_id = Uuid::parse_str(&decoded.claims.sub).map_err(|_| unauthorized())?;

    Ok(UserToken {
        token: None,
        user_id,
        user_type,
        expires_in: None,
    })
}

pub fn verify_confirm_token(secret: &str, token: &str) -> Result<Uuid, ApiError> {
    let key = &DecodingKey::from_base64_secret(secret)
        .map_err(|_| ApiError::internal_error().message("Auth misconfiguration"))?;
    let decoded = jsonwebtoken::decode::<ConfirmClaims>(&token, key, &Validation::default())
        .map_err(|e| {
            let e_str = e.to_string();
            let mut err = ApiError::bad_request();
            if e_str == "ExpiredSignature" {
                err = err.code(ApiErrorCode::ConfirmExpired)
            }
            err.message(format!("Confirm token decode error: {}", e.to_string()))
        })?;

    let user_id = Uuid::parse_str(&decoded.claims.sub).map_err(|_| unauthorized())?;

    Ok(user_id)
}
