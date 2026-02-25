use axum::{Json, extract::State};

use crate::{AppState, dtos::filters::output::FilterDto, errors::AppResult, repositories::OrganizerRepository};

pub async fn get_options(
    State(state): State<AppState>
) -> AppResult<Json<Vec<FilterDto>>> {
    let repo: &dyn OrganizerRepository = &state.repo;
    let rows = repo
        .find_options()
        .await?
        .into_iter()
        .map(FilterDto::from)
        .collect();

    Ok(Json(rows))
}