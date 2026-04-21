use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::competitions::input::{CompetitionByYearQuery, TeamCompetitionStructureQuery},
    services,
};

pub async fn get_structure_by_year(
    State(state): State<AppState>,
    Path(path): Path<TeamCompetitionStructureQuery>,
    Query(query): Query<CompetitionByYearQuery>,
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
