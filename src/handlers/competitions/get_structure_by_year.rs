use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::competitions::input::CompetitionByYearQuery, services};

pub async fn get_structure_by_year(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(query): Query<CompetitionByYearQuery>,
) -> impl IntoResponse {
    services::competitions::get_structure_by_year(&state.repo, id, query.year)
        .await
        .map(|results| Json(results))
}
