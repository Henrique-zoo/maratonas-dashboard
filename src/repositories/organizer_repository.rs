use async_trait::async_trait;

use crate::{errors::AppResult, repositories::{types::IdNameRow, Registry}};

#[async_trait]
pub trait OrganizerRepository {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>>;
}

#[async_trait]
impl OrganizerRepository for Registry {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>> {
        let rows = sqlx::query_as("SELECT id, name FROM organizer")
            .fetch_all(&self.pool).await?;

        Ok(rows)
    }
}