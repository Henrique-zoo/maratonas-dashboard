use axum::{Json, extract::{Query, State}};

use crate::{AppState, dtos::filters::{input::TeamOptionQuery, output::FilterDto}, errors::AppResult, repositories::TeamRepository};

pub async fn get_options(
    State(state): State<AppState>,
    Query(filters): Query<TeamOptionQuery>
) -> AppResult<Json<Vec<FilterDto>>> {
    let repo: &dyn TeamRepository = &state.repo;
    let rows = repo
        .find_option_by_competitions_and_instructions(filters.competition_ids, filters.institution_ids)
        .await?
        .into_iter()
        .map(FilterDto::from)
        .collect();

    Ok(Json(rows))
}