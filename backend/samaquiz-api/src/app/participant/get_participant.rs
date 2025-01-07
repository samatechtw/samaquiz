use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::participant::get_participant_dto::{to_api_response, GetParticipantResponse},
    shared::user::RequestUser,
};
use uuid::Uuid;

use crate::api_context::ApiContext;

use super::helpers::verify_participant_exist_relations;

pub async fn get_participant(
    Path(participant_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(_request_user): Extension<RequestUser>,
) -> Result<Json<GetParticipantResponse>, ApiError> {
    // Get participant from DB
    let participant = verify_participant_exist_relations(&context, participant_id).await?;

    Ok(Json(to_api_response(participant)))
}
