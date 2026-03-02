use async_trait::async_trait;

use crate::{errors::AppResult, repositories::{types::IdNameRow, Registry}};

#[async_trait]
pub trait CompetitionRepository: Send + Sync {
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>>;
}

#[async_trait]
impl CompetitionRepository for Registry {
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>> {
        let rows = if let Some(ids) = organizer_ids {
            sqlx::query_as(
               "SELECT 
                   id, name 
               FROM competition
               WHERE organizer_id = ANY($1)
               ORDER BY name"
           )
           .bind(ids)
           .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as(
                "SELECT 
                    id, name
                FROM competition
                ORDER BY name"
            )
            .fetch_all(&self.pool).await?
        };

        Ok(rows)
    }
}