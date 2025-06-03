use crate::{
    api_context::ApiContext,
    app::{auth, user},
    util::auth::{auth_admin, auth_admin_user, auth_admin_user_anonymous},
};
use axum::{
    handler::Handler,
    http::StatusCode,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{any, get, patch, post},
    Router,
};

use super::{
    answer::{
        create_answer::create_answer, delete_answer::delete_answer, update_answer::update_answer,
    },
    health,
    participant::{create_participant::create_participant, get_participant::get_participant},
    question::{
        create_question::create_question, delete_question::delete_question,
        get_question::get_question, update_question::update_question,
    },
    quiz::{
        create_quiz::create_quiz, delete_quiz::delete_quiz, get_quiz::get_quiz,
        list_quizzes::list_quizzes, update_quiz::update_quiz,
    },
    quiz_response::create_quiz_response::create_quiz_response,
    quiz_session::{
        create_quiz_session::create_quiz_session, get_quiz_session::get_quiz_session,
        get_quiz_session_leaders::get_quiz_session_leaders,
        get_quiz_session_participant_count::get_quiz_session_participant_count,
        list_quiz_sessions::list_quiz_sessions, update_quiz_session::update_quiz_session,
    },
    websocket::ws_handler::ws_handler,
};

pub fn app_router(context: &ApiContext) -> Router<ApiContext> {
    Router::new().nest("/api", api_router(context))
}

pub fn api_router(context: &ApiContext) -> Router<ApiContext> {
    Router::new()
        .route("/healthz", get(health::get_app_health::get_app_health))
        .route("/ws/{session_id}", any(ws_handler))
        .route(
            "/users",
            get(user::list_users::list_users)
                .route_layer(from_fn_with_state(context.clone(), auth_admin)),
        )
        .route(
            "/users/{user_id}",
            get(user::get_user::get_user)
                .patch(user::update_user::update_user)
                .delete(user::delete_user::delete_user)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/users/registrations",
            post(user::register_user::register_user),
        )
        .route("/auth/logins", post(auth::login_user::login_user))
        .route(
            "/auth/logins/reset_password",
            post(auth::reset_password::reset_password),
        )
        .route(
            "/auth/logins/passwords",
            patch(auth::update_password::update_password)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/auth/confirm_email",
            post(auth::confirm_email::confirm_email),
        )
        .route(
            "/auth/resend_confirm_email",
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
            "/quizzes/{quiz_id}",
            get(get_quiz)
                .patch(update_quiz)
                .delete(delete_quiz)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes/{quiz_id}/questions",
            post(create_question).route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/questions/{question_id}",
            get(get_question)
                .patch(update_question.layer(from_fn_with_state(context.clone(), auth_admin_user)))
                .delete(
                    delete_question.layer(from_fn_with_state(context.clone(), auth_admin_user)),
                ),
        )
        .route(
            "/questions/{question_id}/answers",
            post(create_answer).route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/answers/{answer_id}",
            patch(update_answer)
                .delete(delete_answer)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quizzes/{quiz_id}/sessions",
            post(create_quiz_session)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quiz_sessions/code/{code}",
            get(get_quiz_session).route_layer(from_fn_with_state(
                context.clone(),
                auth_admin_user_anonymous,
            )),
        )
        .route(
            "/quiz_sessions",
            get(list_quiz_sessions)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quiz_sessions/{session_id}",
            patch(update_quiz_session)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quiz_sessions/{session_id}/join",
            post(create_participant).route_layer(from_fn_with_state(
                context.clone(),
                auth_admin_user_anonymous,
            )),
        )
        .route(
            "/quiz_sessions/{session_id}/queries/participant_count",
            get(get_quiz_session_participant_count)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/quiz_sessions/{session_id}/queries/leaders",
            get(get_quiz_session_leaders)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/participants/{participant_id}",
            get(get_participant).route_layer(from_fn_with_state(
                context.clone(),
                auth_admin_user_anonymous,
            )),
        )
        .route(
            "/quiz_responses",
            post(create_quiz_response).route_layer(from_fn_with_state(
                context.clone(),
                auth_admin_user_anonymous,
            )),
        )
        .route("/{*path}", get(handler_404)) // Handle unknown routes under /api
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Resource not found")
}
