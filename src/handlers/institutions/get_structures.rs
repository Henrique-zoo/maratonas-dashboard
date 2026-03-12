use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::institutions::input::InstitutionStructuresQuery, services};

pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<InstitutionStructuresQuery>,
) -> impl IntoResponse {
    services::institutions::get_structures(&state.repo, filter.intitution_ids.into_inner())
        .await
        .map(|structures| Json(structures))
}
