use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::common::requests::{IdPath, LocationYearQuery},
    services,
};

pub async fn get_location_stats(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<LocationYearQuery>,
) -> impl IntoResponse {
    services::events::get_location_stats(&state.repo, path.id, query.location_type, query.year)
        .await
        .map(|stats| Json(stats))
}
