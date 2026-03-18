use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::institutions::input::StructuresQuery, services};

pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::institutions::get_structures(&state.repo, filter.institution_ids.into_inner())
        .await
        .map(|structures| Json(structures))
}
