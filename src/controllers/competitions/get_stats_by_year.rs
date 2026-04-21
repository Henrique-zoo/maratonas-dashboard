use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::common::requests::{IdPath, YearQuery},
    services,
};

pub async fn get_stats_by_year(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<YearQuery>,
) -> impl IntoResponse {
    services::competitions::get_stats_by_year(&state.repo, path.id, query.year)
        .await
        .map(|stats| Json(stats))
}
