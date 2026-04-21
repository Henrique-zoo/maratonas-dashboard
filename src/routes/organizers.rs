use axum::{Router, routing::get};

use crate::{AppState, controllers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/organizers/options",
            get(controllers::organizers::get_options),
        )
        .route(
            "/organizers/structures",
            get(controllers::organizers::get_structures),
        )
        .route(
            "/organizers/competitions/{id}/structure",
            get(controllers::organizers::get_structure_by_year),
        )
}
