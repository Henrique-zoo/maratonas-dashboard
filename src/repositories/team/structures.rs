use crate::{
    errors::AppResult,
    repositories::{Registry, types::teams::TeamStructureRow},
};

pub(super) async fn find_structures_by_ids(
    repo: &Registry,
    team_ids: Vec<i32>,
) -> AppResult<Vec<TeamStructureRow>> {
    let rows = sqlx::query_as(
        "WITH selected_teams AS (
            SELECT t.id, t.name
            FROM team t
            WHERE t.id = ANY($1::int[])
        ),
        team_competition_years AS (
            SELECT
                st.id AS team_id,
                st.name AS team_name,
                c.id AS competition_id,
                c.name AS competition_name,
                c.website_url AS competition_website_url,
                c.gender_category AS competition_gender_category,
                MAX(EXTRACT(YEAR FROM ei.date))::int AS latest_year,
                ARRAY_AGG(
                    DISTINCT EXTRACT(YEAR FROM ei.date)::int
                    ORDER BY EXTRACT(YEAR FROM ei.date)::int
                ) AS competition_years
            FROM team_event te
            JOIN event_instance ei ON ei.id = te.event_instance_id
            JOIN event e ON e.id = ei.event_id
            JOIN competition c ON c.id = e.competition_id
            JOIN selected_teams st ON st.id = te.team_id
            GROUP BY st.id, c.id
        ),
        latest_team_events AS (
            SELECT
                te.id AS team_event_id,
                te.team_id,
                e.competition_id,
                ei.id AS event_instance_id,
                e.id AS event_id,
                e.name AS event_name,
                e.level AS event_level,
                ei.date AS event_date,
                ei.location_id AS event_location_id,
                e.scope AS event_scope,
                te.rank AS team_event_rank
            FROM team_event te
            JOIN event_instance ei ON ei.id = te.event_instance_id
            JOIN event e ON e.id = ei.event_id
            JOIN team_competition_years tcy ON tcy.team_id = te.team_id
                AND tcy.competition_id = e.competition_id
                AND EXTRACT(YEAR FROM ei.date)::int = tcy.latest_year
        ),
        latest_event_instances AS (
            SELECT DISTINCT
                lte.event_instance_id,
                lte.event_location_id
            FROM latest_team_events lte
        ),
        event_location AS (
            SELECT
                lei.event_instance_id,
                STRING_AGG(lt.name, ', ' ORDER BY lt.depth) AS event_location
            FROM latest_event_instances lei
            CROSS JOIN LATERAL get_location_tree(lei.event_location_id) lt
            GROUP BY lei.event_instance_id
        ),
        team_event_stats AS (
            SELECT
                tem.team_event_id,
                COUNT(*) FILTER (WHERE tem.role = 'Contestant')::int4 AS team_total_members,
                COUNT(*) FILTER (
                    WHERE tem.role = 'Contestant'
                    AND m.gender = 'Female'
                )::int4 AS team_female_members
            FROM team_event_member tem
            JOIN member m ON m.id = tem.member_id
            JOIN latest_team_events lte ON lte.team_event_id = tem.team_event_id
            GROUP BY tem.team_event_id
        )
        SELECT
            tcy.team_id,
            tcy.team_name,
            COALESCE(tes.team_total_members, 0) AS team_total_members,
            COALESCE(tes.team_female_members, 0) AS team_female_members,

            tcy.competition_id,
            tcy.competition_name,
            tcy.competition_website_url,
            tcy.competition_gender_category,
            tcy.competition_years,

            lte.event_id,
            lte.event_name,
            lte.event_level,
            lte.event_date,
            COALESCE(el.event_location, '') AS event_location,
            lte.event_scope,
            lte.team_event_rank
        FROM team_competition_years tcy
        JOIN latest_team_events lte ON lte.competition_id = tcy.competition_id
            AND lte.team_id = tcy.team_id
        LEFT JOIN team_event_stats tes ON tes.team_event_id = lte.team_event_id
        LEFT JOIN event_location el ON el.event_instance_id = lte.event_instance_id
        ORDER BY tcy.team_name, tcy.competition_name, lte.team_event_rank, lte.event_level, lte.event_name",
    )
    .bind(team_ids)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
