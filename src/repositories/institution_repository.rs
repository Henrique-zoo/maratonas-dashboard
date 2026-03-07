use async_trait::async_trait;

use crate::{errors::AppResult, repositories::{types::{IdNameRow, institutions::InstitutionStructureRow}, Registry}};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait InstitutionRepository: Send + Sync {
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>
    ) -> AppResult<Vec<InstitutionStructureRow>>;
}

#[async_trait]
impl InstitutionRepository for Registry {
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>> {
        let rows = if let Some(ids) = competition_ids{
            sqlx::query_as(
                "SELECT DISTINTC
                    i.id    AS  id,
                    i.name  AS  name
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
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT
                    id, name
                FROM institution
                ORDER BY name"
            )
            .fetch_all(&self.pool)
            .await?
        };
        
        Ok(rows)
    }

    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>
    ) -> AppResult<Vec<InstitutionStructureRow>> {
        let rows = sqlx::query_as(
            "SELECT
                i.id AS institution_id
                i.name AS institution_name
                t.id AS team_id
                t.name AS team_name
                
                COUNT(*) FILTER"
        )
        .bind(institution_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}