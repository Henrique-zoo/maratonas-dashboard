use axum::{Router, routing::get};

use crate::{AppState, handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/teams/options",
            get(handlers::teams::get_options)
        )
        .route(
            "/teams/structures",
            get(handlers::teams::get_structures)
        )
}
