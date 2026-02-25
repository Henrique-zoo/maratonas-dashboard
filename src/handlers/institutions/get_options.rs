use axum::{Json, extract::{Query, State}};

use crate::{AppState, dtos::filters::{input::InstitutionOptionsQuery, output::FilterDto}, errors::AppResult, repositories::InstitutionRepository};

pub async fn get_options(
    State(state): State<AppState>,
    Query(filter): Query<InstitutionOptionsQuery>
) -> AppResult<Json<Vec<FilterDto>>> {
    let repo: &dyn InstitutionRepository = &state.repo;
    let rows = repo
        .find_option_by_competitions(filter.competition_ids)
        .await?
        .into_iter()
        .map(FilterDto::from)
        .collect();

    Ok(Json(rows))
}