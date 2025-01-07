use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::quiz::get_quiz_dto::{to_api_response, GetQuizResponse},
    shared::user::RequestUser,
};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::verify_admin_or_user};

use super::helpers::verify_quiz_exist_relations;

pub async fn get_quiz(
    Path(id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<GetQuizResponse>, ApiError> {
    // Get quiz from DB
    let quiz = verify_quiz_exist_relations(&context, id).await?;

    verify_admin_or_user(&request_user, quiz.user_id.to_string())?;

    Ok(Json(to_api_response(quiz)))
}
