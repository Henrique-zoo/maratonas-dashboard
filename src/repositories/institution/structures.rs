use crate::{
    errors::AppResult,
    repositories::{Registry, types::institutions::InstitutionStructureRow},
};

pub(super) async fn find_structures_by_ids(
    repo: &Registry,
    institution_ids: Vec<i32>,
) -> AppResult<Vec<InstitutionStructureRow>> {
    let rows = sqlx::query_as(
        "WITH selected_institution_teams AS (
            SELECT
                i.id AS institution_id,
                i.name AS institution_name,
                i.short_name AS institution_short_name,
                i.main_location_id AS institution_main_location_id,

                t.id AS team_id,
                t.name AS team_name
            FROM institution i
            JOIN team t ON t.institution_id = i.id
            WHERE i.id = ANY($1::int[])
        ),
        institution_location AS (
            SELECT
                sit.institution_id,
                STRING_AGG(lt.name, ', ' ORDER BY lt.depth) AS institution_location
            FROM selected_institution_teams sit
            CROSS JOIN LATERAL get_location_tree(sit.institution_main_location_id) lt
            GROUP BY sit.institution_id
        ),
        competition_latest_year AS (
            SELECT
                e.competition_id,
                MAX(EXTRACT(YEAR FROM ei.date))::int AS latest_year
            FROM event e
            JOIN event_instance ei ON ei.event_id = e.id
            GROUP BY e.competition_id
        ),
        latest_event_instances AS (
            SELECT
                sit.team_id,
                sit.team_name,
                te.id AS team_event_id,
                te.rank AS team_event_rank,

                sit.institution_id,
                sit.institution_name,
                sit.institution_short_name,

                e.id AS event_id,
                e.name AS event_name,
                ei.date AS event_date,
                e.level AS event_level,
                e.scope AS event_scope,
                c.id AS competition_id,
                c.name AS competition_name,
                c.website_url AS competition_website_url
            FROM selected_institution_teams sit
            JOIN team_event te ON te.team_id = sit.team_id
            JOIN event_instance ei ON ei.id = te.event_instance_id
            JOIN event e ON ei.event_id = e.id
            JOIN competition c ON c.id = e.competition_id
            JOIN competition_latest_year cly ON cly.competition_id = c.id
            WHERE EXTRACT(YEAR FROM ei.date)::int = cly.latest_year
        ),
        team_totals AS (
            SELECT
                lei.team_event_id,
                COUNT(*) FILTER (WHERE tem.role = 'Contestant')::int4 AS team_total_members,
                COUNT(*) FILTER (
                    WHERE tem.role = 'Contestant'
                    AND m.gender = 'Female'
                )::int4 AS team_female_members
            FROM latest_event_instances lei
            JOIN team_event_member tem ON tem.team_event_id = lei.team_event_id
            JOIN member m ON m.id = tem.member_id
            GROUP BY lei.team_event_id
        )
        SELECT
            lei.institution_id,
            lei.institution_name,
            lei.institution_short_name,
            il.institution_location,

            lei.competition_id,
            lei.competition_name,
            lei.competition_website_url,

            lei.event_id,
            lei.event_name,
            lei.event_date,
            lei.event_level,
            lei.event_scope,

            lei.team_id,
            lei.team_name,
            lei.team_event_rank,
            tt.team_total_members,
            tt.team_female_members
        FROM latest_event_instances lei
        JOIN institution_location il ON il.institution_id = lei.institution_id
        JOIN team_totals tt ON tt.team_event_id = lei.team_event_id",
    )
    .bind(institution_ids)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
