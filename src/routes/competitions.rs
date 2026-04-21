use axum::{Router, routing::get};

use crate::{AppState, handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/competitions/options",
            get(handlers::competitions::get_options),
        )
        .route(
            "/competitions/structures",
            get(handlers::competitions::get_structures),
        )
        .route(
            "/competitions/{id}/structure",
            get(handlers::competitions::get_structure_by_year),
        )
        .route(
            "/competitions/{id}/stats",
            get(handlers::competitions::get_stats_by_year),
        )
        .route(
            "/competitions/{id}/location_stats",
            get(handlers::competitions::get_location_stats),
        )
}
