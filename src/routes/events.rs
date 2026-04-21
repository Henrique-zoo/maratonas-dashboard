use axum::{Router, routing::get};

use crate::{AppState, controllers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/events/{id}/location_stats",
            get(controllers::events::get_location_stats),
        )
        .route(
            "/events/{id}/stats",
            get(controllers::events::get_stats_by_year),
        )
}
