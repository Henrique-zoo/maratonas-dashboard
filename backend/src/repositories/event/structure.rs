//! Consulta da identidade e das ocorrências anuais de um evento lógico.

use crate::{
    errors::AppResult,
    repositories::{Registry, types::events::EventYearStructureRow},
};

/// Retorna os metadados autoritativos do evento e todas as suas instâncias no
/// ano solicitado. Quando o ano não é informado, usa o último ano disponível
/// para o próprio evento.
pub(super) async fn find_structure(
    repo: &Registry,
    event_id: i32,
    year: Option<i32>,
) -> AppResult<Vec<EventYearStructureRow>> {
    let rows = sqlx::query_as(
        "WITH event_context AS (
            SELECT
                e.id AS event_id,
                e.name AS event_name,
                e.level AS event_level,
                e.scope AS event_scope,
                c.id AS competition_id,
                c.name AS competition_name,
                COALESCE($2::int, MAX(EXTRACT(YEAR FROM ei.date))::int) AS selected_year,
                ARRAY_AGG(
                    DISTINCT EXTRACT(YEAR FROM ei.date)::int
                    ORDER BY EXTRACT(YEAR FROM ei.date)::int
                ) AS available_years
            FROM event e
            JOIN competition c ON c.id = e.competition_id
            JOIN event_instance ei ON ei.event_id = e.id
            WHERE e.id = $1
            GROUP BY e.id, e.name, e.level, e.scope, c.id, c.name
        ),
        selected_instances AS (
            SELECT
                ec.*,
                ei.id AS event_instance_id,
                ei.date AS event_date,
                ei.location_id AS event_location_id
            FROM event_context ec
            JOIN event_instance ei ON ei.event_id = ec.event_id
                AND EXTRACT(YEAR FROM ei.date)::int = ec.selected_year
        ),
        event_location AS (
            SELECT
                si.event_instance_id,
                STRING_AGG(lt.name, ', ' ORDER BY lt.depth) AS event_location
            FROM selected_instances si
            CROSS JOIN LATERAL get_location_tree(si.event_location_id) lt
            GROUP BY si.event_instance_id
        ),
        event_location_types AS (
            SELECT
                si.event_id,
                COALESCE(
                    ARRAY_AGG(DISTINCT lt.type ORDER BY lt.type)
                        FILTER (WHERE lt.type IS NOT NULL),
                    ARRAY[]::location_type[]
                ) AS event_location_types
            FROM selected_instances si
            LEFT JOIN team_event te ON te.event_instance_id = si.event_instance_id
            LEFT JOIN team t ON t.id = te.team_id
            LEFT JOIN institution i ON i.id = t.institution_id
            LEFT JOIN LATERAL get_location_tree(
                COALESCE(te.campus_location_id, i.main_location_id)
            ) lt ON te.id IS NOT NULL
            GROUP BY si.event_id
        )
        SELECT
            si.event_id,
            si.event_name,
            si.event_level,
            si.event_scope,
            si.competition_id,
            si.competition_name,
            si.selected_year,
            si.available_years,
            elt.event_location_types,
            si.event_instance_id,
            si.event_date,
            el.event_location
        FROM selected_instances si
        JOIN event_location el ON el.event_instance_id = si.event_instance_id
        JOIN event_location_types elt ON elt.event_id = si.event_id
        ORDER BY si.event_date, si.event_instance_id",
    )
    .bind(event_id)
    .bind(year)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
