use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::QJson;
use lib_types::dto::quiz::quiz_view_model::{to_api_response, QuizViewModel};
use lib_types::dto::quiz::update_quiz_dto::UpdateQuizDto;
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;

use crate::app::helpers::verify_admin_or_user;
use crate::db::quiz_repo::QuizUpdateProps;

use super::helpers::verify_quiz_exist_relations;

pub async fn update_quiz(
    Path(quiz_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    QJson(dto): QJson<UpdateQuizDto>,
) -> Result<(StatusCode, Json<QuizViewModel>), ApiError> {
    check_bad_form(dto.validate())?;

    // Get quiz to be updated
    let quiz_to_be_updated = verify_quiz_exist_relations(&context, quiz_id).await?;

    // Verify request
    verify_admin_or_user(&request_user, quiz_to_be_updated.user_id.to_string())?;

    if let Some(questions_order) = &dto.questions_order {
        if quiz_to_be_updated.questions.len() != questions_order.len() {
            return Err(ApiError::bad_request().code(ApiErrorCode::OrderLength));
        }
        let q_ids: Vec<String> = quiz_to_be_updated
            .questions
            .iter()
            .map(|q| q.id.to_string())
            .collect();
        for new_id in questions_order.iter() {
            if !q_ids.iter().any(|id| id == new_id) {
                return Err(ApiError::bad_request().code(ApiErrorCode::OrderValue));
            }
        }
    }

    let props = QuizUpdateProps {
        title: dto.title,
        description: dto.description,
        questions_order: dto.questions_order,
    };

    // Update quiz
    let quiz_result = context
        .repo
        .quiz
        .update_quiz(quiz_id, props)
        .await
        .map_err(|e| match e {
            _ => ApiError::internal_error().message(format!("Failed to update quiz: {}", e)),
        })?;

    // Return response
    Ok((StatusCode::OK, Json(to_api_response(quiz_result))))
}
