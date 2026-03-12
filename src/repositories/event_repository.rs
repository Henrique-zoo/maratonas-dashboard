use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{Registry, types::events::EventLocationStatsRow},
    shared::types::LocationType,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait EventRepository {
    async fn find_location_stats_by_event(
        &self,
        event_id: i32,
        location_id: LocationType,
        year: i32,
    ) -> AppResult<Vec<EventLocationStatsRow>>;
}

#[async_trait]
impl EventRepository for Registry {
    async fn find_location_stats_by_event(
        &self,
        event_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<EventLocationStatsRow>> {
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

            WHERE e.id = $1::int
            AND lt.type = $2::location_type
            AND EXTRACT(YEAR FROM ei.date) = $3::int

            GROUP BY lt.id, lt.name
            ORDER BY lt.name"
        )
        .bind(event_id)
        .bind(location_type)
        .bind(year)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
