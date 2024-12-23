use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::headers::{authorization::Bearer, Authorization};

use lib_api::{
    auth::{
        types::UserToken,
        util::{extract_bearer, extract_bearer_optional},
        verify_jwt::verify_jwt,
    },
    error::api_error::ApiError,
};
use lib_types::shared::user::{RequestUser, UserType};

use crate::api_context::ApiContext;

pub async fn verify_user_exist(context: ApiContext, token: &UserToken) -> Result<(), ApiError> {
    if token.user_type.clone() != UserType::Anonymous {
        let _ = context
            .repo
            .user
            .get_user_by_id(token.user_id)
            .await
            .map_err(|_| ApiError::unauthorized())?;
    }
    Ok(())
}

pub async fn auth_admin(
    State(context): State<ApiContext>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let (bearer, request) = extract_bearer(request).await?;

    auth_user_helper(vec![UserType::Admin], bearer, context, request, next).await
}

pub async fn auth_admin_user(
    State(context): State<ApiContext>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let (bearer, request) = extract_bearer(request).await?;

    auth_user_helper(
        vec![UserType::Admin, UserType::User],
        bearer,
        context,
        request,
        next,
    )
    .await
}

/*
pub async fn auth_admin_user_cron(
    State(context): State<ApiContext>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let api_key_opt = request
        .headers()
        .get("X-JOBS-KEY")
        .and_then(|k| k.to_str().ok());

    if let Some(jobs_api_key) = api_key_opt {
        if context.config.jobs_api_key == jobs_api_key {
            request.extensions_mut().insert(RequestUser {
                user_type: UserType::Cron,
                user_id: None,
            });
            Ok(next.run(request).await)
        } else {
            Err(ApiError::forbidden())
        }
    } else {
        let (bearer, request) = extract_bearer(request).await?;
        auth_user_helper(
            vec![UserType::Admin, UserType::User],
            bearer,
            context,
            request,
            next,
        )
        .await
    }
}

pub async fn auth_admin_cron(
    State(context): State<ApiContext>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let api_key_opt = request
        .headers()
        .get("X-JOBS-KEY")
        .and_then(|k| k.to_str().ok());

    if let Some(jobs_api_key) = api_key_opt {
        if context.config.jobs_api_key == jobs_api_key {
            request.extensions_mut().insert(RequestUser {
                user_type: UserType::Cron,
                user_id: None,
            });
            Ok(next.run(request).await)
        } else {
            Err(ApiError::forbidden())
        }
    } else {
        let (bearer, request) = extract_bearer(request).await?;
        auth_user_helper(vec![UserType::Admin], bearer, context, request, next).await
    }
}
 */

pub async fn auth_user(
    State(context): State<ApiContext>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let (bearer, request) = extract_bearer(request).await?;

    auth_user_helper(vec![UserType::User], bearer, context, request, next).await
}

pub async fn auth_admin_user_anonymous(
    State(context): State<ApiContext>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let (bearer_opt, mut request) = extract_bearer_optional(request).await;
    if let Some(bearer) = bearer_opt {
        auth_user_helper(
            vec![UserType::Admin, UserType::User],
            bearer,
            context,
            request,
            next,
        )
        .await
    } else {
        request.extensions_mut().insert(RequestUser {
            user_type: UserType::Anonymous,
            user_id: None,
        });
        Ok(next.run(request).await)
    }
}

async fn auth_user_helper(
    expected_types: Vec<UserType>,
    auth: Authorization<Bearer>,
    context: ApiContext,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    match verify_jwt(&context.config.app_auth_secret, auth.token()) {
        Ok(user_token) => {
            if expected_types.contains(&user_token.user_type) {
                verify_user_exist(context, &user_token).await?;
                request.extensions_mut().insert(RequestUser {
                    user_type: user_token.user_type,
                    user_id: Some(user_token.user_id),
                });
                Ok(next.run(request).await)
            } else {
                Err(ApiError::forbidden())
            }
        }
        Err(e) => Err(e),
    }
}
