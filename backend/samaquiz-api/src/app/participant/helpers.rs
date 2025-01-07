use lib_api::error::api_error::ApiError;
use lib_types::entity::participant_entity::ParticipantEntityRelations;
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn verify_participant_exist_relations(
    context: &ApiContext,
    id: Uuid,
) -> Result<ParticipantEntityRelations, ApiError> {
    let participant = context
        .repo
        .participant
        .get_participant_relations_by_id(id)
        .await
        .map_err(not_found_or_internal)?;
    Ok(participant)
}
