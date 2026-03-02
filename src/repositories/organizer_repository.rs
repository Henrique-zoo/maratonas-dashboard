use async_trait::async_trait;

use crate::{errors::AppResult, repositories::{Registry, types::{IdNameRow, organizers::OrganizerStructureRow}}};

#[async_trait]
pub trait OrganizerRepository: Send + Sync {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>>;
    async fn find_structure_by_organizer_ids(
        &self,
        organizer_ids: Vec<i32>
    ) -> AppResult<Vec<OrganizerStructureRow>>;
}

#[async_trait]
impl OrganizerRepository for Registry {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>> {
        let rows = sqlx::query_as("SELECT id, name FROM organizer")
            .fetch_all(&self.pool).await?;

        Ok(rows)
    }

    async fn find_structure_by_organizer_ids(
        &self,
        organizer_ids: Vec<i32>
    ) -> AppResult<Vec<OrganizerStructureRow>> {
        let rows = sqlx::query_as(
            "SELECT DISTINCT
                o.id AS organizer_id,
                o.name AS organizer_name,
                o.website_url AS organizer_website_url,
                c.id AS competition_id,
                c.name AS competition_name,
                c.gender_category AS competition_gender_category,
                c.website_url AS competition_website_url,
                e.id AS event_id,
                e.name AS event_name
            FROM organizer o
            LEFT JOIN competition c
                ON o.id = c.organizer_id
            LEFT JOIN event e
                ON c.id = e.competition_id
            WHERE o.id = ANY($1::int[])
            ORDER BY organizer_name"
        )
        .bind(organizer_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}