use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        types::{
            IdNameRow,
            competitions::{CompetitionLocationStatsRow, CompetitionStructureRow},
        },
    },
    shared::types::LocationType,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CompetitionRepository: Send + Sync {
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        competition_ids: Vec<i32>,
    ) -> AppResult<Vec<CompetitionStructureRow>>;
    async fn find_location_stats_by_competition(
        &self,
        competition_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<CompetitionLocationStatsRow>>;
}

#[async_trait]
impl CompetitionRepository for Registry {
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        let rows = if let Some(ids) = organizer_ids {
            sqlx::query_as(
                "SELECT
                    id, name
                FROM competition
                WHERE organizer_id = ANY($1::int[])
                ORDER BY name",
            )
            .bind(ids)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT
                    id, name
                FROM competition
                ORDER BY name",
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows)
    }

    async fn find_structures_by_ids(
        &self,
        competitions_ids: Vec<i32>,
    ) -> AppResult<Vec<CompetitionStructureRow>> {
        let rows = sqlx::query_as(
            "SELECT
                c.id AS competition_id,
                c.name AS competition_name,
                c.gender_category AS competition_gender_category,
                c.website_url AS competition_website_url,

                COUNT(DISTINCT t.id) OVER (PARTITION BY c.id) AS competition_total_teams,
                SUM(p.team_total_members) OVER (PARTITION BY c.id) AS competition_total_participants,
                SUM(p.team_female_members) OVER (PARTITION BY c.id) AS competition_female_participants,

                e.id AS event_id,
                e.name AS event_name,
                e.level AS event_level,
                ei.date AS event_date,

                string_agg(elt.name, ', ' ORDER BY elt.depth) AS event_location,
                COUNT(DISTINCT t.id) OVER (PARTITION BY e.id) AS event_total_teams,
                SUM(p.team_total_members) OVER (PARTITION BY e.id) AS event_total_participants,
                SUM(p.team_female_members) OVER (PARTITION BY e.id) AS event_female_participants,
                COUNT(DISTINCT i.id) OVER (PARTITION BY e.id) AS event_total_institutions,

                i.name AS institution_name,
                i.short_name AS institution_short_name,
                string_agg(ilt.name, ', ' ORDER BY ilt.depth) AS institution_location,

                ilt.type AS institution_location_type,

                t.id AS team_id,
                t.name AS team_name,
                te.rank AS team_rank,
                p.team_total_members AS team_total_members,
                p.team_female_members AS team_female_members

            FROM competition c
            JOIN event e ON c.id = e.competition_id
            JOIN event_instance ei ON ei.event_id = e.id
            CROSS JOIN LATERAL get_location_tree(ei.location_id) elt
            JOIN team_event te ON te.event_instance_id = ei.id
            JOIN team t ON t.id = te.team_id
            JOIN institution i ON i.id = t.institution_id

            CROSS JOIN LATERAL get_location_tree(COALESCE(
                te.campus_location_id,
                i.main_location_id)
            ) ilt

            JOIN (
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
            ) p ON p.team_event_id = te.id

            WHERE c.id = ANY($1::int[])
            AND EXTRACT(YEAR FROM ei.date) = (
                SELECT EXTRACT(YEAR FROM MAX(ei2.date))
                FROM event_instance ei2
                JOIN event e2 ON e2.id = ei2.event_id
                WHERE e2.competition_id = c.id
            )

            GROUP BY
                c.id, c.name, c.gender_category, c.website_url,
                e.id, e.name, e.level, ei.date,
                i.id, i.name, i.short_name,
                ilt.type,
                te.id, te.rank,
                t.id, t.name,
                p.team_total_members, p.team_female_members
                
            ORDER BY c.name, e.level, e.name, te.rank, t.name",
        )
        .bind(competitions_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn find_location_stats_by_competition(
        &self,
        competition_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<CompetitionLocationStatsRow>> {
        let rows = sqlx::query_as(
            "SELECT
                lt.id AS location_id,
                lt.name AS location_name,

                COUNT(DISTINCT i.id) AS total_institutions,
                COUNT(DISTINCT t.id) AS total_teams,

                SUM(p.total_participants) AS total_participants,
                SUM(p.female_participants) AS female_participants

            FROM team_event te
            JOIN team t ON t.id = te.team_id
            JOIN institution i ON i.id = t.institution_id
            CROSS JOIN LATERAL get_location_tree(COALESCE(te.campus_location_id, i.main_location_id)) lt
            JOIN event_instance ei ON ei.id = te.event_instance_id
            JOIN event e ON e.id = ei.event_id

            JOIN (
                SELECT
                    tem.team_event_id,
                    COUNT(*) FILTER (WHERE tem.role = 'Contestant') AS total_participants,
                    COUNT(*) FILTER (
                        WHERE tem.role = 'Contestant'
                        AND m.gender = 'Female'
                    ) AS female_participants
                FROM team_event_member tem
                JOIN member m ON m.id = tem.member_id
                GROUP BY tem.team_event_id
            ) p ON p.team_event_id = te.id

            WHERE e.competition_id = $1::int
            AND lt.type = $2::location_type
            AND EXTRACT(YEAR FROM ei.date) = $3::int

            GROUP BY lt.id, lt.name
            ORDER BY lt.name"
        )
        .bind(competition_id)
        .bind(location_type)
        .bind(year)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
