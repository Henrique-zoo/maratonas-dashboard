use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::filters::input::{IdPath, LocationYearStatsQuery},
    services,
};

pub async fn get_location_stats(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<LocationYearStatsQuery>,
) -> impl IntoResponse {
    services::competitions::get_location_stats(
        &state.repo,
        path.id,
        query.location_type,
        query.year,
    )
    .await
    .map(|stats| Json(stats))
}
