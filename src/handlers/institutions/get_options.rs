use axum::{Json, extract::{Query, State}, response::IntoResponse};

use crate::{AppState, dtos::filters::input::InstitutionOptionsQuery, services};

pub async fn get_options(
    State(state): State<AppState>,
    Query(filter): Query<InstitutionOptionsQuery>
) -> impl IntoResponse {
    services::institutions::get_options(
        &state.repo,
        filter.competition_ids
    )
        .await
        .map(|options| Json(options))
}