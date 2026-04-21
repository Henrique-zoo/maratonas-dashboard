use axum::{Router, routing::get};

use crate::{AppState, handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/institutions/options",
            get(handlers::institutions::get_options),
        )
        .route(
            "/institutions/structures",
            get(handlers::institutions::get_structures),
        )
        .route(
            "/institutions/{institution_id}/events/{event_id}",
            get(handlers::institutions::get_event_performance_over_time),
        )
}
