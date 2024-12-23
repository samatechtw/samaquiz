use crate::{
    api_context::ApiContext,
    app::{auth, user},
    util::auth::{auth_admin, auth_admin_user},
};
use axum::{
    http::StatusCode,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, patch, post},
    Router,
};

use super::{
    answer::{
        create_answer::create_answer, delete_answer::delete_answer, update_answer::update_answer,
    },
    health,
    question::{
        create_question::create_question, delete_question::delete_question,
        get_question::get_question, update_question::update_question,
    },
    quiz::{
        create_quiz::create_quiz, delete_quiz::delete_quiz, get_quiz::get_quiz,
        list_quizzes::list_quizzes, update_quiz::update_quiz,
    },
};

pub fn app_router(context: &ApiContext) -> Router<ApiContext> {
    Router::new().nest("/api", api_router(context))
}

pub fn api_router(context: &ApiContext) -> Router<ApiContext> {
    Router::new()
        .route("/healthz", get(health::get_app_health::get_app_health))
        .route(
            "/users",
            get(user::list_users::list_users)
                .route_layer(from_fn_with_state(context.clone(), auth_admin)),
        )
        .route(
            "/users/:user_id",
            get(user::get_user::get_user)
                .patch(user::update_user::update_user)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/users/registrations",
            post(user::register_user::register_user),
        )
        .route("/auth/logins", post(auth::login_user::login_user))
        .route(
            "/auth/logins/reset-password",
            post(auth::reset_password::reset_password),
        )
        .route(
            "/auth/logins/passwords",
            patch(auth::update_password::update_password)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/auth/confirm-email",
            post(auth::confirm_email::confirm_email),
        )
        .route(
            "/auth/resend-confirm-email",
            post(auth::resend_confirm_email::resend_confirm_email)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes",
            get(list_quizzes)
                .post(create_quiz)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes/:quiz_id",
            get(get_quiz)
                .patch(update_quiz)
                .delete(delete_quiz)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes/:quiz_id/questions",
            post(create_question).route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes/:quiz_id/questions/:question_id",
            get(get_question)
                .patch(update_question)
                .delete(delete_question)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes/:quiz_id/questions/:question_id/answers",
            post(create_answer).route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes/:quiz_id/questions/:question_id/answers/:answer_id",
            patch(update_answer)
                .delete(delete_answer)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route("/*path", get(handler_404)) // Handle unknown routes under /api
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Resource not found")
}
