use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        types::events::{EventLocationStatsRow, EventYearStatsRow},
    },
    shared::types::LocationType,
};

pub(super) async fn find_location_stats_by_event(
    repo: &Registry,
    event_id: i32,
    location_type: LocationType,
    year: i32,
) -> AppResult<Vec<EventLocationStatsRow>> {
    let rows = sqlx::query_as(
        "SELECT
            lt.id AS location_id,
            lt.name AS location_name,

            COUNT(DISTINCT i.id)::int4 AS total_institutions,
            COUNT(DISTINCT t.id)::int4 AS total_teams,

            SUM(p.total_participants)::int4 AS total_participants,
            SUM(p.female_participants)::int4 AS female_participants

        FROM team_event te
        JOIN team t ON t.id = te.team_id
        JOIN institution i ON i.id = t.institution_id
        CROSS JOIN LATERAL get_location_tree(COALESCE(te.campus_location_id, i.main_location_id)) lt
        JOIN event_instance ei ON ei.id = te.event_instance_id
        JOIN event e ON e.id = ei.event_id

        JOIN (
            SELECT
                tem.team_event_id,
                COUNT(*)::int4 FILTER (WHERE tem.role = 'Contestant') AS total_participants,
                COUNT(*)::int4 FILTER (
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
        ORDER BY lt.name",
    )
    .bind(event_id)
    .bind(location_type)
    .bind(year)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}

pub(super) async fn find_event_stats_by_year(
    repo: &Registry,
    event_id: i32,
    year: i32,
) -> AppResult<EventYearStatsRow> {
    let rows = sqlx::query_as(
        "SELECT
            COUNT(DISTINCT i.id)::int4 AS total_institutions,
            COUNT(DISTINCT t.id)::int4 AS total_teams,
            SUM
            SUM
        FROM event e
        JOIN event_instance ei ON ei.event_id = e.id
        JOIN team_event te ON te.event_instance_id = ei.id
        JOIN team t ON t.id = te.team_id
        JOIN institution i ON i.id = t.institution_id
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

        WHERE e.id = $1
            AND EXTRACT(YEAR FROM ei.date)::int = $2",
    )
    .bind(event_id)
    .bind(year)
    .fetch_one(&repo.pool)
    .await?;

    Ok(rows)
}
