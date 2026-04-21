use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::institutions::requests::OptionsQuery, services};

pub async fn get_options(
    State(state): State<AppState>,
    Query(filter): Query<OptionsQuery>,
) -> impl IntoResponse {
    services::institutions::get_options(&state.repo, filter.competition_ids.into_inner())
        .await
        .map(|options| Json(options))
}
