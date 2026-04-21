use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::{common::requests::YearQuery, teams::requests::CompetitionStructurePath},
    services,
};

pub async fn get_structure_by_year(
    State(state): State<AppState>,
    Path(path): Path<CompetitionStructurePath>,
    Query(query): Query<YearQuery>,
) -> impl IntoResponse {
    services::teams::get_structure_by_year(
        &state.repo,
        path.team_id,
        path.competition_id,
        query.year,
    )
    .await
    .map(|structure| Json(structure))
}
