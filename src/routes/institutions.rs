use axum::{Router, routing::get};

use crate::{AppState, controllers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/institutions/options",
            get(controllers::institutions::get_options),
        )
        .route(
            "/institutions/structures",
            get(controllers::institutions::get_structures),
        )
        .route(
            "/institutions/{institution_id}/events/{event_id}",
            get(controllers::institutions::get_event_performance_over_time),
        )
}
