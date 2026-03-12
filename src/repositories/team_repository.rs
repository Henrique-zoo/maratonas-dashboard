use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        types::{IdNameRow, teams::TeamStructureRow},
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TeamRepository: Send + Sync {
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(&self, team_ids: Vec<i32>) -> AppResult<Vec<TeamStructureRow>>;
}

#[async_trait]
impl TeamRepository for Registry {
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT DISTINCT
                t.id AS id,
                t.name AS name
            FROM team t",
        );

        let mut first = true;
        if let Some(ids) = competition_ids {
            builder.push(
                "JOIN team_event te
                    ON te.team_id = t.id
                JOIN event_instance ei
                    ON te.event_instance_id = ei.id
                JOIN event e
                    ON ei.event_id = e.id ",
            );
            builder
                .push("WHERE e.competition_id = ANY(")
                .push_bind(ids)
                .push(") ");
            first = false;
        }

        if let Some(ids) = institution_ids {
            builder.push(if first { "WHERE " } else { "AND " });
            builder
                .push("t.institution_id = ANY(")
                .push_bind(ids)
                .push(") ");
        }

        builder.push("ORDER BY t.name");

        let rows = builder.build_query_as().fetch_all(&self.pool).await?;

        Ok(rows)
    }

    async fn find_structures_by_ids(&self, team_ids: Vec<i32>) -> AppResult<Vec<TeamStructureRow>> {
        let rows = sqlx::query_as(
            "WITH team_event_stats AS (
                SELECT
                    tem.team_event_id,
                    COUNT(*) FILTER (WHERE tem.role = 'Contestant') AS team_total_members
                FROM team_event_member tem
                GROUP BY tem.team_event_id
            )
            SELECT
                t.id AS team_id,
                t.name AS team_name,

                c.id AS competition_id,
                c.name AS competition_name,
                e.id AS event_id,
                e.name AS event_name,
                tes.team_total_members AS event_total_participants

            FROM team t
            JOIN team_event te ON te.team_id = t.id
            JOIN event_instance ei ON ei.id = te.event_instance_id
            JOIN event e ON e.id = ei.event_id
            JOIN competition c ON c.id = e.competition_id
            JOIN team_event_stats tes ON tes.team_event_id = te.id

            WHERE t.id = ANY($1::int[])

            ORDER BY t.name, c.name, e.level NULLS LAST, e.name",
        )
        .bind(team_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
