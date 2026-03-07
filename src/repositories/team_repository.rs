use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};

use crate::{errors::AppResult, repositories::{types::{IdNameRow, teams::TeamStructureRow}, Registry}};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TeamRepository: Send + Sync {
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        team_ids: Vec<i32>
    ) -> AppResult<Vec<TeamStructureRow>>;
}

#[async_trait]
impl TeamRepository for Registry {
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>
    ) -> AppResult<Vec<IdNameRow>> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT DISTINCT
                t.id    AS  id,
                t.name  AS  name
            FROM team t "
        );

        let mut first = true;
        if let Some(ids) = competition_ids {
            builder.push(
                "JOIN team_event te
                    ON te.team_id = t.id
                JOIN event e
                    ON te.event_id = e.id "
                );
            builder.push("WHERE e.competition_id = ANY(")
                .push_bind(ids)
                .push(") ");
            first = false;
        }

        if let Some(ids) = institution_ids {
            builder.push(if first { "WHERE " } else { "AND " });
            builder.push("t.institution_id = ANY(")
                .push_bind(ids)
                .push(") ");
        }

        builder.push("ORDER BY t.name");

        let rows = builder
            .build_query_as()
            .fetch_all(&self.pool).await?;

        Ok(rows)
    }

    async fn find_structures_by_ids(
        &self,
        team_ids: Vec<i32>
    ) -> AppResult<Vec<TeamStructureRow>> {
        let rows = sqlx::query_as(
            "-- TODO"
        )
            .bind(team_ids)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}