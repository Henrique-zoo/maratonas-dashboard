use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        types::{IdNameRow, organizers::OrganizerStructureRow},
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait OrganizerRepository: Send + Sync {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        organizer_ids: Vec<i32>,
    ) -> AppResult<Vec<OrganizerStructureRow>>;
}

#[async_trait]
impl OrganizerRepository for Registry {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>> {
        let rows = sqlx::query_as("SELECT id, name FROM organizer")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn find_structures_by_ids(
        &self,
        organizer_ids: Vec<i32>,
    ) -> AppResult<Vec<OrganizerStructureRow>> {
        let rows = sqlx::query_as(
            "WITH selected_events AS (
                SELECT
                    o.id AS organizer_id,
                    o.name AS organizer_name,
                    o.website_url AS organizer_website_url,
                    c.id AS competition_id,
                    c.name AS competition_name,
                    c.website_url AS competition_website_url,
                    c.gender_category AS competition_gender_category,
                    e.id AS event_id,
                    e.name AS event_name,
                    e.level AS event_level
                FROM organizer o
                JOIN competition c ON c.organizer_id = o.id
                JOIN event e ON e.competition_id = c.id
                WHERE o.id = ANY($1::int[])
            ),
            event_metadata AS (
                SELECT
                    se.organizer_id,
                    se.organizer_name,
                    se.organizer_website_url,
                    se.competition_id,
                    se.competition_name,
                    se.competition_website_url,
                    se.competition_gender_category,
                    se.event_id,
                    se.event_name,
                    se.event_level,
                    MAX(ei.date) AS event_date,
                    array_agg(
                        DISTINCT EXTRACT(YEAR FROM ei.date)::int
                        ORDER BY EXTRACT(YEAR FROM ei.date)::int
                    ) AS event_years
                FROM selected_events se
                JOIN event_instance ei ON ei.event_id = se.event_id
                GROUP BY
                    se.organizer_id,
                    se.organizer_name,
                    se.organizer_website_url,
                    se.competition_id,
                    se.competition_name,
                    se.competition_website_url,
                    se.competition_gender_category,
                    se.event_id,
                    se.event_name,
                    se.event_level
            ),
            latest_event_team_rows AS (
                SELECT
                    em.organizer_id,
                    em.organizer_name,
                    em.organizer_website_url,
                    em.competition_id,
                    em.competition_name,
                    em.competition_website_url,
                    em.competition_gender_category,
                    em.event_id,
                    em.event_name,
                    em.event_level,
                    em.event_date,
                    em.event_years,
                    te.team_id,
                    COUNT(*) FILTER (WHERE tem.role = 'Contestant') AS team_total_members,
                    COUNT(*) FILTER (
                        WHERE tem.role = 'Contestant'
                        AND m.gender = 'Female'
                    ) AS team_female_members
                FROM event_metadata em
                JOIN event_instance ei ON ei.event_id = em.event_id
                    AND ei.date = em.event_date
                JOIN team_event te ON te.event_instance_id = ei.id
                JOIN team_event_member tem ON tem.team_event_id = te.id
                JOIN member m ON m.id = tem.member_id
                GROUP BY
                    em.organizer_id,
                    em.organizer_name,
                    em.organizer_website_url,
                    em.competition_id,
                    em.competition_name,
                    em.competition_website_url,
                    em.competition_gender_category,
                    em.event_id,
                    em.event_name,
                    em.event_level,
                    em.event_date,
                    em.event_years,
                    te.team_id
            ),
            selected_organizer_rows AS (
                SELECT
                    letr.organizer_id,
                    letr.organizer_name,
                    letr.organizer_website_url,
                    letr.competition_id,
                    letr.competition_name,
                    letr.competition_website_url,
                    letr.competition_gender_category,
                    letr.event_id,
                    letr.event_name,
                    letr.event_level,
                    letr.event_date,
                    letr.event_years
                FROM latest_event_team_rows letr
                GROUP BY
                    letr.organizer_id,
                    letr.organizer_name,
                    letr.organizer_website_url,
                    letr.competition_id,
                    letr.competition_name,
                    letr.competition_website_url,
                    letr.competition_gender_category,
                    letr.event_id,
                    letr.event_name,
                    letr.event_level,
                    letr.event_date,
                    letr.event_years
            ),
            event_totals AS (
                SELECT
                    event_id,
                    COUNT(DISTINCT team_id) AS event_total_teams,
                    SUM(team_total_members) AS event_total_participants,
                    SUM(team_female_members) AS event_female_participants
                FROM latest_event_team_rows
                GROUP BY event_id
            ),
            competition_totals AS (
                SELECT
                    competition_id,
                    COUNT(DISTINCT team_id) AS competition_total_teams,
                    SUM(team_total_members) AS competition_total_participants,
                    SUM(team_female_members) AS competition_female_participants
                FROM latest_event_team_rows
                GROUP BY competition_id
            )
            SELECT
                sor.organizer_id,
                sor.organizer_name,
                sor.organizer_website_url,

                sor.competition_id,
                sor.competition_name,
                sor.competition_website_url,
                sor.competition_gender_category,
                ct.competition_total_teams,
                ct.competition_total_participants,
                ct.competition_female_participants,

                sor.event_id,
                sor.event_name,
                sor.event_level,
                sor.event_date,
                et.event_total_teams,
                et.event_total_participants,
                et.event_female_participants,
                sor.event_years
            FROM selected_organizer_rows sor
            JOIN competition_totals ct ON ct.competition_id = sor.competition_id
            JOIN event_totals et ON et.event_id = sor.event_id

            ORDER BY sor.organizer_name, sor.competition_id, sor.event_level, sor.event_name",
        )
        .bind(organizer_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}

/*
"SELECT DISTINCT
    o.id AS organizer_id,
    o.name AS organizer_name,
    o.website_url AS organizer_website_url,

    c.id AS competition_id,
    c.name AS competition_name,
    c.gender_category AS competition_gender_category,
    c.website_url AS competition_website_url,

    COUNT(DISTINCT t.id) OVER (PARTITION BY c.id) AS competition_total_teams,

    COUNT(*) FILTER (WHERE tem.role = 'Contestant') OVER (PARTITION BY c.id)
        AS competition_total_participants,

    COUNT(*) FILTER (
        WHERE tem.role = 'Contestant'
        AND m.gender = 'Female'
    ) OVER (PARTITION BY c.id) AS competition_female_participants,

    e.id AS event_id,
    e.name AS event_name,

    COUNT(DISTINCT t.id) OVER (PARTITION BY e.id) AS event_total_teams,

    COUNT(*) FILTER (WHERE tem.role = 'Contestant') OVER (PARTITION BY e.id)
        AS event_total_participants,

    COUNT(*) FILTER (
        WHERE tem.role = 'Contestant'
        AND m.gender = 'Female'
    ) OVER (PARTITION BY e.id) AS event_female_participants

FROM organizer o
JOIN competition c ON o.id = c.organizer_id
JOIN event e ON c.id = e.competition_id
JOIN team_event te ON e.id = te.event_id
JOIN team t ON t.id = te.team_id
JOIN team_event_member tem ON te.id = tem.team_event_id
JOIN member m ON m.id = tem.member_id

WHERE c.id = ANY($1::int[])
    AND EXTRACT(YEAR FROM e.date) = (
        SELECT EXTRACT(YEAR FROM MAX(e2.date))
        FROM event e2
        WHERE e2.competition_id = c.id
    )

GROUP BY
    o.id, o.name, o.website_url,
    c.id, c.name, c.gender_category, c.website_url,
    e.id, e.name
    
ORDER BY o.name, c.name, e.name"
*/
