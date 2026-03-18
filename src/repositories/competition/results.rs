use crate::{
    errors::AppResult,
    repositories::{Registry, types::competitions::CompetitionYearResultRow},
};

pub(super) async fn find_competition_results_by_year(
    repo: &Registry,
    competition_id: i32,
    year: i32,
) -> AppResult<Vec<CompetitionYearResultRow>> {
    let rows = sqlx::query_as(
        "WITH selected_event_instance AS (
                SELECT
                    e.competition_id AS competition_id,
                    e.id AS event_id,
                    e.name AS event_name,
                    e.level AS event_level,
                    ei.id AS event_instance_id,
                    ei.date AS event_date,
                    ei.location_id AS event_location_id
                FROM event e
                JOIN event_instance ei ON ei.event_id = e.id
                WHERE e.competition_id = $1
                AND EXTRACT(YEAR FROM ei.date)::int = $2
            ),
            event_location AS (
                SELECT
                    sei.event_instance_id,
                    string_agg(lt.name, ', ' ORDER BY lt.depth) AS event_location
                FROM selected_event_instance sei
                CROSS JOIN LATERAL get_location_tree(sei.event_location_id) lt
                GROUP BY sei.event_instance_id
            ),
            event_team_rows AS (
                SELECT
                    sei.competition_id,
                    sei.event_instance_id,
                    sei.event_id,
                    sei.event_name,
                    sei.event_level,
                    sei.event_date,
                    i.id AS institution_id,
                    i.name AS institution_name,
                    i.short_name AS institution_short_name,
                    i.main_location_id,
                    te.id AS team_event_id,
                    t.id AS team_id,
                    t.name AS team_name,
                    te.rank AS team_rank,
                    te.campus_location_id,
                    COUNT(*) FILTER (WHERE tem.role = 'Contestant')::int4 AS team_total_members,
                    COUNT(*) FILTER (
                        WHERE tem.role = 'Contestant'
                        AND m.gender = 'Female'
                    )::int4 AS team_female_members
                FROM selected_event_instance sei
                JOIN team_event te ON te.event_instance_id = sei.event_instance_id
                JOIN team t ON t.id = te.team_id
                JOIN institution i ON i.id = t.institution_id
                JOIN team_event_member tem ON tem.team_event_id = te.id
                JOIN member m ON m.id = tem.member_id
                GROUP BY
                    sei.competition_id,
                    sei.event_instance_id,
                    sei.event_id,
                    sei.event_name, sei.event_level, sei.event_date,
                    i.id,
                    te.id,
                    t.id
            ),
            location_base AS (
                SELECT DISTINCT
                    etr.competition_id,
                    etr.event_instance_id,
                    etr.team_event_id,
                    COALESCE(etr.campus_location_id, etr.main_location_id) AS location_id
                FROM event_team_rows etr
            ),
            full_location AS (
                SELECT
                    lb.location_id,
                    string_agg(lt.name, ', ' ORDER BY lt.depth) AS institution_location
                FROM location_base lb
                CROSS JOIN LATERAL get_location_tree(lb.location_id) lt
                GROUP BY location_id
            ),
            team_location AS (
                SELECT
                    lb.team_event_id,
                    fl.institution_location
                FROM location_base lb
                JOIN full_location fl ON fl.location_id = lb.location_id
            ),
            team_location_types AS (
                SELECT
                    lb.competition_id,
                    lb.event_instance_id,
                    lt.type AS location_type,
                    lt.depth AS location_depth
                FROM location_base lb
                CROSS JOIN LATERAL get_location_tree(lb.location_id) lt
            ),
            event_location_types AS (
                SELECT
                    event_instance_id,
                    array_agg(DISTINCT location_type) AS event_location_types
                FROM team_location_types
                GROUP BY event_instance_id
            ),
            competition_location_types AS (
                SELECT
                    competition_id,
                    array_agg(DISTINCT location_type) AS competition_location_types
                FROM team_location_types
                GROUP BY competition_id
            )
            SELECT
                clt.competition_location_types,

                etr.event_id,
                etr.event_name,
                etr.event_level,
                etr.event_date,
                el.event_location,
                elt.event_location_types,

                etr.institution_name,
                etr.institution_short_name,
                tl.institution_location,

                etr.team_id,
                etr.team_name,
                etr.team_rank,
                etr.team_total_members,
                etr.team_female_members
            FROM event_team_rows etr
            JOIN competition_location_types clt ON clt.competition_id = etr.competition_id
            JOIN event_location el ON el.event_instance_id = etr.event_instance_id
            JOIN event_location_types elt ON elt.event_instance_id = etr.event_instance_id
            JOIN team_location tl ON tl.team_event_id = etr.team_event_id

            ORDER BY etr.event_level, etr.event_date, etr.event_name, etr.team_rank",
    )
    .bind(competition_id)
    .bind(year)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
