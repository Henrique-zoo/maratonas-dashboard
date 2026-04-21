use axum::{Router, routing::get};

use crate::{AppState, controllers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/teams/options", get(controllers::teams::get_options))
        .route("/teams/structures", get(controllers::teams::get_structures))
        .route(
            "/teams/{team_id}/competitions/{competition_id}",
            get(controllers::teams::get_structure_by_year),
        )
}
