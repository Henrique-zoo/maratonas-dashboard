use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::filters::input::CompetitionOptionsQuery, services};

#[debug_handler]
pub async fn get_options(
    State(state): State<AppState>,
    Query(filter): Query<CompetitionOptionsQuery>,
) -> impl IntoResponse {
    services::competitions::get_options(&state.repo, filter.organizer_ids.into_inner())
        .await
        .map(|options| Json(options))
}
