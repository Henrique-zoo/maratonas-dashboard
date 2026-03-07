use axum::{Json, extract::{Query, State}, response::IntoResponse};

use crate::{AppState, dtos::filters::input::TeamOptionQuery, services};

pub async fn get_options(
    State(state): State<AppState>,
    Query(filters): Query<TeamOptionQuery>
) -> impl IntoResponse {
    services::teams::get_options(&state.repo, filters.competition_ids.into_inner(), filters.institution_ids.into_inner())
        .await
        .map(|options| Json(options))
}