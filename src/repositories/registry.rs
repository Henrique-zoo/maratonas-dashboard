use sqlx::PgPool;

#[derive(Clone)]
pub struct Registry {
    pub(super) pool: PgPool
}

impl Registry {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}