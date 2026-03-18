use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::teams::input::StructuresQuery, services};

pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::teams::get_structures(&state.repo, filter.team_ids.into_inner())
        .await
        .map(|structures| Json(structures))
}
