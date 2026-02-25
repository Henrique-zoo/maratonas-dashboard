use async_trait::async_trait;

use crate::{errors::AppResult, repositories::{types::IdNameRow, Registry}};

#[async_trait]
pub trait InstitutionRepository {
    async fn find_option_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>>;
}

#[async_trait]
impl InstitutionRepository for Registry {
    async fn find_option_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>> {
        let rows = if let Some(ids) = competition_ids{
            sqlx::query_as(
                "SELECT DISTINTC
                    i.id, i.name
                FROM institution i
                JOIN team t
                    ON t.institution_id = i.id
                JOIN team_event te
                    ON te.team_id = t.id
                JOIN event e
                    ON te.event_id = e.id
                JOIN competition c
                    ON e.competition_id = c.id
                WHERE c.id = ANY($1)
                ORDER BY i.name"
            )
            .bind(ids)
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as(
                "SELECT
                    id, name
                FROM institutions
                ORDER BY name"
            )
            .fetch_all(&self.pool).await?
        };
        
        Ok(rows)
    }
}