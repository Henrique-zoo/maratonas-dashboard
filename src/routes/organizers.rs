use axum::{Router, routing::get};

use crate::{AppState, handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/organizers/options",
            get(handlers::organizers::get_options),
        )
        .route(
            "/organizers/structures",
            get(handlers::organizers::get_structures),
        )
        .route(
            "/organizers/competitions/{id}/structure",
            get(handlers::organizers::get_structure_by_year),
        )
}
