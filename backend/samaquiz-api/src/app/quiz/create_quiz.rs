use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::quiz::create_quiz_dto::{CreateQuizDto, CreateQuizResponse};
use lib_types::entity::quiz_entity::QuizEntity;
use lib_types::shared::quiz::QuizType;
use lib_types::shared::user::RequestUser;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::helpers::get_request_user;
use crate::db::quiz_repo::QuizCreateProps;

fn to_api_response(result: QuizEntity) -> Json<CreateQuizResponse> {
    return Json(CreateQuizResponse { id: result.id });
}

#[debug_handler]
pub async fn create_quiz(
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<CreateQuizDto>,
) -> Result<(StatusCode, Json<CreateQuizResponse>), ApiError> {
    check_bad_form(dto.validate())?;
    let user = get_request_user(&context, &request_user).await?;

    let props = QuizCreateProps {
        user_id: user.id,
        title: dto.title,
        description: dto.description,
        quiz_type: QuizType::Quiz,
    };

    let quiz_result =
        context.repo.quiz.create_quiz(props).await.map_err(|e| {
            ApiError::internal_error().message(format!("Failed to create quiz: {}", e))
        })?;

    Ok((StatusCode::CREATED, to_api_response(quiz_result)))
}
