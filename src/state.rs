use sqlx::PgPool;

use crate::repositories::Registry;

#[derive(Clone)]
pub struct AppState {
    pub repo: Registry
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { repo: Registry::new(pool) }
    }
}