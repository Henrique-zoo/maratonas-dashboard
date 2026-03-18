use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::institutions::input::{PerformanceOverTimePath, PerformanceOverTimeQuery},
    services,
};

pub async fn get_event_performance_over_time(
    State(state): State<AppState>,
    Path(path): Path<PerformanceOverTimePath>,
    Query(query): Query<PerformanceOverTimeQuery>,
) -> impl IntoResponse {
    services::institutions::get_event_performance_over_time(
        &state.repo,
        path.institution_id,
        path.event_id,
        query.start_year,
        query.end_year,
    )
    .await
    .map(|performance| Json(performance))
}
