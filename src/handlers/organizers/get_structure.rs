use axum::{Json, extract::{Query, State}, response::IntoResponse};

use crate::{AppState, dtos::filters::input::CompetitionOptionsQuery, services};

pub async fn get_structure(
    State(state): State<AppState>,
    Query(filter): Query<CompetitionOptionsQuery>
) -> impl IntoResponse {
    services::organizers::get_structure(
        &state.repo,
        filter.organizer_ids
    )
        .await
        .map(|structure| Json(structure))
}