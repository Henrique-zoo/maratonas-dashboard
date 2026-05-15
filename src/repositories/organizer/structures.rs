//! # `backend::repositories::organizer::structures`
//!
//! ## Responsabilidade
//! Implementa consultas do repositório de `organizer`.
//!
//! ## Lógica de Implementação
//! Executa consultas SQL analíticas com CTEs, agregações e árvore de localização para retornar linhas tipadas com alta densidade de dados.
//!
//! ## Funções
//! - `find_structures_by_ids`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{
    errors::AppResult,
    repositories::{Registry, types::organizers::OrganizerStructureRow},
};

/// Busca linhas para montar a estrutura pública de organizadores.
///
/// Para cada organizador informado, a query percorre competições e eventos,
/// considera o último ano disponível de cada competição e agrega totais de
/// instituições, times e participantes por evento.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `organizer_ids`: organizadores que devem compor o resultado.
///
/// # Retorno
/// Vetor de [`OrganizerStructureRow`] ordenado por organizador, competição,
/// nível e nome do evento.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_structures_by_ids(
    repo: &Registry,
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
        competition_years AS (
            SELECT
                se.competition_id,
                array_agg(
                    DISTINCT EXTRACT(YEAR FROM ei.date)::int
                    ORDER BY EXTRACT(YEAR FROM ei.date)::int
                ) AS competition_years
            FROM selected_events se
            JOIN event_instance ei ON ei.event_id = se.event_id
            GROUP BY se.competition_id
        ),
        competition_latest_year AS (
            SELECT
                se.competition_id,
                MAX(EXTRACT(YEAR FROM ei.date))::int AS latest_year
            FROM selected_events se
            JOIN event_instance ei ON ei.event_id = se.event_id
            GROUP BY se.competition_id
        ),
        latest_year_event_team_rows AS (
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
                ei.id AS event_instance_id,
                ei.date AS event_date,
                ei.location_id AS event_location_id,
                te.id AS team_event_id,
                i.id AS institution_id,
                te.team_id,
                COUNT(*) FILTER (WHERE tem.role = 'Contestant')::int4 AS team_total_members,
                COUNT(*) FILTER (
                    WHERE tem.role = 'Contestant'
                    AND m.gender = 'Female'
                )::int4 AS team_female_members
            FROM selected_events se
            JOIN competition_latest_year cly ON cly.competition_id = se.competition_id
            JOIN event_instance ei ON ei.event_id = se.event_id
                AND EXTRACT(YEAR FROM ei.date)::int = cly.latest_year
            JOIN team_event te ON te.event_instance_id = ei.id
            JOIN team t ON t.id = te.team_id
            JOIN institution i ON i.id = t.institution_id
            JOIN team_event_member tem ON tem.team_event_id = te.id
            JOIN member m ON m.id = tem.member_id
            GROUP BY
                se.organizer_id, se.organizer_name, se.organizer_website_url, se.competition_id,
                se.competition_name, se.competition_website_url, se.competition_gender_category,
                se.event_id, se.event_name, se.event_level,
                ei.id, ei.date, ei.location_id,
                i.id,
                te.id,
                te.team_id
        ),
        selected_organizer_rows AS (
            SELECT
                lyetr.organizer_id,
                lyetr.organizer_name,
                lyetr.organizer_website_url,
                lyetr.competition_id,
                lyetr.competition_name,
                lyetr.competition_website_url,
                lyetr.competition_gender_category,
                cy.competition_years,
                lyetr.event_instance_id,
                lyetr.event_id,
                lyetr.event_name,
                lyetr.event_level,
                lyetr.event_date
            FROM latest_year_event_team_rows lyetr
            JOIN competition_years cy ON cy.competition_id = lyetr.competition_id
            GROUP BY
                lyetr.organizer_id,
                lyetr.organizer_name,
                lyetr.organizer_website_url,
                lyetr.competition_id,
                lyetr.competition_name,
                lyetr.competition_website_url,
                lyetr.competition_gender_category,
                cy.competition_years,
                lyetr.event_instance_id,
                lyetr.event_id,
                lyetr.event_name,
                lyetr.event_level,
                lyetr.event_date
        ),
        event_totals AS (
            SELECT
                event_instance_id,
                COUNT(DISTINCT institution_id)::int4 AS event_total_institutions,
                COUNT(DISTINCT team_id)::int4 AS event_total_teams,
                SUM(team_total_members)::int4 AS event_total_participants,
                SUM(team_female_members)::int4 AS event_female_participants
            FROM latest_year_event_team_rows
            GROUP BY event_instance_id
        ),
        event_location_base AS (
            SELECT DISTINCT
                event_instance_id,
                event_location_id
            FROM latest_year_event_team_rows
        ),
        event_location AS (
            SELECT
                elb.event_instance_id,
                string_agg(lt.name, ', ' ORDER BY lt.depth) AS event_location
            FROM event_location_base elb
            CROSS JOIN LATERAL get_location_tree(elb.event_location_id) lt
            GROUP BY elb.event_instance_id
        ),
        team_location_types AS (
            SELECT DISTINCT
                lyetr.event_instance_id,
                lyetr.competition_id,
                lt.type AS location_type
            FROM latest_year_event_team_rows lyetr
            JOIN team_event te ON te.id = lyetr.team_event_id
            JOIN team t ON t.id = lyetr.team_id
            JOIN institution i ON i.id = t.institution_id
            CROSS JOIN LATERAL get_location_tree(
                COALESCE(te.campus_location_id, i.main_location_id)
            ) lt
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
            sor.organizer_id,
            sor.organizer_name,
            sor.organizer_website_url,

            sor.competition_id,
            sor.competition_name,
            sor.competition_website_url,
            sor.competition_gender_category,
            sor.competition_years,
            clt.competition_location_types,

            sor.event_id,
            sor.event_name,
            sor.event_level,
            sor.event_date,
            el.event_location,
            et.event_total_institutions,
            et.event_total_teams,
            et.event_total_participants,
            et.event_female_participants,
            elt.event_location_types
        FROM selected_organizer_rows sor
        JOIN event_totals et ON et.event_instance_id = sor.event_instance_id
        JOIN event_location el ON el.event_instance_id = sor.event_instance_id
        JOIN competition_location_types clt ON clt.competition_id = sor.competition_id
        JOIN event_location_types elt ON elt.event_instance_id = sor.event_instance_id

        ORDER BY sor.organizer_name, sor.competition_id, sor.event_level, sor.event_name",
    )
    .bind(organizer_ids)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
