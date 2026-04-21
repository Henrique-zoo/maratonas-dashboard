use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::organizers::requests::StructuresQuery, services};

pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::organizers::get_structures(&state.repo, filter.organizer_ids.into_inner())
        .await
        .map(|structures| Json(structures))
}
