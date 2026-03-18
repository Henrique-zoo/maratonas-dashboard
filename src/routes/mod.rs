use axum::Router;

use crate::AppState;

mod competitions;
mod events;
mod institutions;
mod organizers;
mod teams;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(competitions::router())
        .merge(organizers::router())
        .merge(institutions::router())
        .merge(teams::router())
        .merge(events::router())
}
