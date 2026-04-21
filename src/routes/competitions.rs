use axum::{Router, routing::get};

use crate::{AppState, controllers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/competitions/options",
            get(controllers::competitions::get_options),
        )
        .route(
            "/competitions/structures",
            get(controllers::competitions::get_structures),
        )
        .route(
            "/competitions/{id}/structure",
            get(controllers::competitions::get_structure_by_year),
        )
        .route(
            "/competitions/{id}/stats",
            get(controllers::competitions::get_stats_by_year),
        )
        .route(
            "/competitions/{id}/location_stats",
            get(controllers::competitions::get_location_stats),
        )
}
