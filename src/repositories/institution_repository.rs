use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        types::{IdNameRow, institutions::InstitutionStructureRow},
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait InstitutionRepository: Send + Sync {
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>,
    ) -> AppResult<Vec<InstitutionStructureRow>>;
}

#[async_trait]
impl InstitutionRepository for Registry {
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        let rows = if let Some(ids) = competition_ids {
            sqlx::query_as(
                "SELECT DISTINCT
                    i.id AS id,
                    i.name AS name
                FROM institution i
                JOIN team t ON t.institution_id = i.id
                JOIN team_event te ON te.team_id = t.id
                JOIN event_instance ei ON ei.id = te.event_instance_id
                JOIN event e ON e.id = ei.event_id
                JOIN competition c ON e.competition_id = c.id
                WHERE c.id = ANY($1::int[])
                ORDER BY i.name",
            )
            .bind(ids)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT
                    id, name
                FROM institution
                ORDER BY name",
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows)
    }

    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>,
    ) -> AppResult<Vec<InstitutionStructureRow>> {
        let rows = sqlx::query_as(
            "WITH latest_event_years AS (
                SELECT
                    e.competition_id,
                    EXTRACT(YEAR FROM MAX(ei.date))::int AS latest_year
                FROM event e
                JOIN event_instance ei ON ei.event_id = e.id
                GROUP BY e.competition_id
            ),
            team_event_stats AS (
                SELECT
                    tem.team_event_id,
                    COUNT(*) FILTER (WHERE tem.role = 'Contestant') AS team_total_members,
                    COUNT(*) FILTER (
                        WHERE tem.role = 'Contestant'
                        AND m.gender = 'Female'
                    ) AS team_female_members
                FROM team_event_member tem
                JOIN member m ON m.id = tem.member_id
                GROUP BY tem.team_event_id
            ),
            latest_year_team_events AS (
                SELECT
                    i.id AS institution_id,
                    i.name AS institution_name,
                    c.id AS competition_id,
                    c.name AS competition_name,
                    c.website_url AS competition_website_url,
                    e.id AS event_id,
                    e.name AS event_name,
                    e.level AS event_level,
                    ei.date AS event_date,
                    t.id AS team_id,
                    t.name AS team_name,
                    te.rank AS team_event_rank,
                    stats.team_total_members,
                    stats.team_female_members
                FROM competition c
                JOIN latest_event_years ley ON ley.competition_id = c.id
                JOIN event e ON e.competition_id = c.id
                JOIN event_instance ei ON ei.event_id = e.id
                    AND EXTRACT(YEAR FROM ei.date)::int = ley.latest_year
                JOIN team_event te ON te.event_instance_id = ei.id
                JOIN team t ON t.id = te.team_id
                JOIN institution i ON i.id = t.institution_id
                JOIN team_event_stats stats ON stats.team_event_id = te.id
            ),
            selected_institution_rows AS (
                SELECT *
                FROM latest_year_team_events
                WHERE institution_id = ANY($1::int[])
            ),
            institution_totals AS (
                SELECT
                    institution_id,
                    COUNT(DISTINCT team_id) AS institution_total_teams,
                    SUM(team_total_members) AS institution_total_contestants,
                    SUM(team_female_members) AS institution_female_contestants
                FROM selected_institution_rows
                GROUP BY institution_id
            ),
            competition_totals AS (
                SELECT
                    competition_id,
                    COUNT(DISTINCT team_id) AS competition_total_teams,
                    SUM(team_total_members) AS competition_total_participants
                FROM latest_year_team_events
                GROUP BY competition_id
            ),
            event_totals AS (
                SELECT
                    event_id,
                    COUNT(DISTINCT team_id) AS event_total_teams,
                    SUM(team_total_members) AS event_total_participants
                FROM latest_year_team_events
                GROUP BY event_id
            )
            SELECT
                sir.institution_id,
                sir.institution_name,
                it.institution_total_teams,
                it.institution_total_contestants,
                it.institution_female_contestants,

                sir.competition_id,
                sir.competition_name,
                sir.competition_website_url,
                ct.competition_total_teams,
                ct.competition_total_participants,

                sir.event_id,
                sir.event_name,
                sir.event_date,
                et.event_total_teams,
                et.event_total_participants,

                sir.team_id,
                sir.team_name,
                sir.team_event_rank,
                sir.team_total_members,
                sir.team_female_members
            FROM selected_institution_rows sir
            JOIN institution_totals it ON it.institution_id = sir.institution_id
            JOIN competition_totals ct ON ct.competition_id = sir.competition_id
            JOIN event_totals et ON et.event_id = sir.event_id

            ORDER BY
                sir.institution_name,
                sir.competition_name,
                sir.event_level NULLS LAST,
                sir.event_name,
                sir.team_name",
        )
        .bind(institution_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
