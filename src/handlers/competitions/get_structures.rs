use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::competitions::input::CompetitionStructuresQuery, services};

pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<CompetitionStructuresQuery>,
) -> impl IntoResponse {
    services::competitions::get_structures(&state.repo, filter.competition_ids.into_inner())
        .await
        .map(|structures| Json(structures))
}
