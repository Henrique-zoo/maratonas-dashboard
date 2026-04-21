use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::competitions::requests::OptionsQuery, services};

pub async fn get_options(
    State(state): State<AppState>,
    Query(filter): Query<OptionsQuery>,
) -> impl IntoResponse {
    services::competitions::get_options(&state.repo, filter.organizer_ids.into_inner())
        .await
        .map(|options| Json(options))
}
