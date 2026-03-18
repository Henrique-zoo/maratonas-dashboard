use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::{competitions::input::CompetitionByYearQuery, filters::input::IdPath},
    services,
};

pub async fn get_stats_by_year(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<CompetitionByYearQuery>,
) -> impl IntoResponse {
    services::events::get_stats_by_year(&state.repo, path.id, query.year)
        .await
        .map(|stats| Json(stats))
}
