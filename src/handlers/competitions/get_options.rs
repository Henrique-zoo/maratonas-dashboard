use axum::{Json, extract::{Query, State}};

use crate::{AppState, dtos::filters::{input::CompetitionOptionsQuery, output::FilterDto}, errors::AppResult, repositories::CompetitionRepository};

pub async fn get_options(
    State(state): State<AppState>,
    Query(filter): Query<CompetitionOptionsQuery>
) -> AppResult<Json<Vec<FilterDto>>> {
    let repo: &dyn CompetitionRepository = &state.repo;
    let rows = repo
        .find_option_by_organizers(filter.organizer_ids)
        .await?
        .into_iter()
        .map(FilterDto::from)
        .collect();

    Ok(Json(rows))
}