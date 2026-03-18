use axum::{Router, routing::get};

use crate::{AppState, handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/events/{id}/location_stats",
            get(handlers::events::get_location_stats),
        )
        .route(
            "/events/{id}/stats",
            get(handlers::events::get_stats_by_year),
        )
}
