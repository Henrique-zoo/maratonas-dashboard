use axum::{Json, debug_handler, extract::State, response::IntoResponse};

use crate::{AppState, services};

#[debug_handler]
pub async fn get_options(
    State(state): State<AppState>
) -> impl IntoResponse {
    services::organizers::get_options(&state.repo)
        .await
        .map(|options| Json(options))
}