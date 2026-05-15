//! # `backend::repositories::competition::structures`
//!
//! ## Responsabilidade
//! Implementa consultas do repositório de `competition`.
//!
//! ## Lógica de Implementação
//! Executa consultas SQL analíticas com CTEs, agregações e árvore de localização para retornar linhas tipadas com alta densidade de dados.
//!
//! ## Funções
//! - `find_structures_by_ids`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_events_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_structure_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_team_result_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        types::competitions::{
            CompetitionEventsByYearRow, CompetitionStructureRow, CompetitionTeamYearResultRow,
            CompetitionYearStructureRow,
        },
    },
};

/// Busca linhas para montar a estrutura pública de competições.
///
/// Para cada competição informada, a query identifica o último ano com eventos
/// registrados e retorna linhas denormalizadas com metadados da competição,
/// eventos desse ano, localização e resultado dos times.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `competitions_ids`: competições que devem compor o resultado.
///
/// # Retorno
/// Vetor de [`CompetitionStructureRow`] ordenado por competição, nível do
/// evento, nome do evento e ranking do time.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_structures_by_ids(
    repo: &Registry,
    competitions_ids: Vec<i32>,
) -> AppResult<Vec<CompetitionStructureRow>> {
    let rows = sqlx::query_as(
            "WITH selected_events AS (
                SELECT
                    c.id AS competition_id,
                    c.name AS competition_name,
                    c.website_url AS competition_website_url,
                    c.gender_category AS competition_gender_category,

                    e.id AS event_id,
                    e.name AS event_name,
                    e.level AS event_level
                FROM competition c
                JOIN event e ON e.competition_id = c.id
                WHERE c.id = ANY($1::int[])
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
            latest_year_team_rows AS (
                SELECT
                    se.competition_id,
                    se.competition_name,
                    se.competition_website_url,
                    se.competition_gender_category,
                    se.event_id,
                    se.event_name,
                    se.event_level,
                    ei.date AS event_date,
                    ei.location_id AS event_location_id,

                    te.id AS team_event_id,
                    te.rank AS team_rank,
                    te.campus_location_id,

                    t.id AS team_id,
                    t.name AS team_name,

                    i.id AS institution_id,
                    i.name AS institution_name,
                    i.short_name AS institution_short_name,
                    i.main_location_id,

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
                    se.competition_id, se.competition_name, se.competition_gender_category, se.competition_website_url,
                    se.event_id, se.event_name, se.event_level,
                    ei.date, ei.location_id,
                    te.id, te.rank, te.campus_location_id,
                    t.id, t.name,
                    i.id, i.name, i.short_name, i.main_location_id
            ),
            event_location_base AS (
                SELECT DISTINCT
                    lytr.event_id,
                    lytr.event_location_id
                FROM latest_year_team_rows lytr
            ),
            event_location AS (
                SELECT
                    elb.event_id,
                    string_agg(lt.name, ', ' ORDER BY lt.depth) AS event_location
                FROM event_location_base elb
                CROSS JOIN LATERAL get_location_tree(elb.event_location_id) lt
                GROUP BY elb.event_id
            ),
            team_location AS (
                SELECT
                    lytr.team_event_id,
                    string_agg(lt.name, ', ' ORDER BY lt.depth) AS institution_location
                FROM latest_year_team_rows lytr
                CROSS JOIN LATERAL get_location_tree(
                    COALESCE(lytr.campus_location_id, lytr.main_location_id)
                ) lt
                GROUP BY lytr.team_event_id
            ),
            team_location_types AS (
                SELECT DISTINCT
                    lytr.team_event_id,
                    lytr.event_id,
                    lytr.competition_id,
                    lt.type AS location_type,
                    lt.depth AS location_depth
                FROM latest_year_team_rows lytr
                CROSS JOIN LATERAL get_location_tree(
                    COALESCE(lytr.campus_location_id, lytr.main_location_id)
                ) lt
            ),
            event_location_types AS (
                SELECT
                    event_id,
                    array_agg(DISTINCT location_type) AS event_location_types
                FROM team_location_types
                GROUP BY event_id
            ),
            competition_location_types AS (
                SELECT
                    competition_id,
                    array_agg(DISTINCT location_type) AS competition_location_types
                FROM team_location_types
                GROUP BY competition_id
            )
            SELECT
                lytr.competition_id,
                lytr.competition_name,
                lytr.competition_website_url,
                lytr.competition_gender_category,
                cy.competition_years,
                clt.competition_location_types,

                lytr.event_id,
                lytr.event_name,
                lytr.event_level,
                lytr.event_date,
                el.event_location,
                elt.event_location_types,

                lytr.institution_name,
                lytr.institution_short_name,
                tl.institution_location,

                lytr.team_id,
                lytr.team_name,
                lytr.team_rank,
                lytr.team_total_members,
                lytr.team_female_members
            FROM latest_year_team_rows lytr
            JOIN competition_years cy ON cy.competition_id = lytr.competition_id
            JOIN competition_location_types clt ON clt.competition_id = lytr.competition_id
            JOIN event_location el ON el.event_id = lytr.event_id
            JOIN event_location_types elt ON elt.event_id = lytr.event_id
            JOIN team_location tl ON tl.team_event_id = lytr.team_event_id

            ORDER BY lytr.competition_name, lytr.event_level, lytr.event_name, lytr.team_rank",
        )
    .bind(competitions_ids)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}

/// Busca eventos de uma competição em um ano com totais agregados.
///
/// A query retorna uma linha por instância de evento, incluindo localização do
/// evento, tipos de localização observados nas equipes participantes e totais
/// de instituições, times e participantes.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `competition_id`: competição usada como recorte principal.
/// - `year`: ano das instâncias de evento consideradas.
///
/// # Retorno
/// Vetor de [`CompetitionEventsByYearRow`] ordenado por nível, data e nome do
/// evento.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_events_by_year(
    repo: &Registry,
    competition_id: i32,
    year: i32,
) -> AppResult<Vec<CompetitionEventsByYearRow>> {
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
                    sei.event_date,
                    i.id AS institution_id,
                    i.main_location_id,
                    te.id AS team_event_id,
                    te.team_id,
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
                    sei.event_date,
                    i.id,
                    te.id,
                    te.team_id
            ),
            team_locations AS (
                SELECT DISTINCT
                    etr.competition_id,
                    etr.event_instance_id,
                    COALESCE(etr.campus_location_id, etr.main_location_id) AS location_id
                FROM event_team_rows etr
            ),
            team_location_types AS (
                SELECT
                    tl.competition_id,
                    tl.event_instance_id,
                    lt.type AS location_type,
                    lt.depth AS location_depth
                FROM team_locations tl
                CROSS JOIN LATERAL get_location_tree(tl.location_id) lt
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
            ),
            event_totals AS (
                SELECT
                    event_instance_id,
                    COUNT(DISTINCT institution_id)::int4 AS event_total_institutions,
                    COUNT(DISTINCT team_id)::int4 AS event_total_teams,
                    SUM(team_total_members)::int4 AS event_total_participants,
                    SUM(team_female_members)::int4 AS event_female_participants
                FROM event_team_rows
                GROUP BY event_instance_id
            )
            SELECT
                clt.competition_location_types,

                sei.event_id,
                sei.event_name,
                sei.event_level,
                sei.event_date,
                el.event_location,
                et.event_total_institutions,
                et.event_total_teams,
                et.event_total_participants,
                et.event_female_participants,
                elt.event_location_types

            FROM selected_event_instance sei
            JOIN competition_location_types clt ON clt.competition_id = sei.competition_id
            JOIN event_location el ON el.event_instance_id = sei.event_instance_id
            JOIN event_totals et ON et.event_instance_id = sei.event_instance_id
            JOIN event_location_types elt ON elt.event_instance_id = sei.event_instance_id

            ORDER BY sei.event_level, sei.event_date, sei.event_name",
    )
    .bind(competition_id)
    .bind(year)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}

/// Busca linhas da estrutura detalhada de uma competição em um ano.
///
/// Cada linha representa a participação de um time em um evento do ano,
/// incluindo dados da instituição, localização resolvida e ranking. O service
/// consome essas linhas para montar a árvore `eventos -> times`.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `competition_id`: competição usada como recorte principal.
/// - `year`: ano das instâncias de evento consideradas.
///
/// # Retorno
/// Vetor de [`CompetitionYearStructureRow`] ordenado por nível, data, nome do
/// evento e ranking do time.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_structure_by_year(
    repo: &Registry,
    competition_id: i32,
    year: i32,
) -> AppResult<Vec<CompetitionYearStructureRow>> {
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

/// Busca o resultado anual de um time em uma competição.
///
/// A query retorna os eventos disputados pelo time no ano informado, incluindo
/// localização, escopo, ranking e totais de membros da participação.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `team_id`: time analisado.
/// - `competition_id`: competição usada como recorte.
/// - `year`: ano das instâncias de evento consideradas.
///
/// # Retorno
/// Vetor de [`CompetitionTeamYearResultRow`] ordenado por ranking, nível, nome
/// e data do evento.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_team_result_by_year(
    repo: &Registry,
    team_id: i32,
    competition_id: i32,
    year: i32,
) -> AppResult<Vec<CompetitionTeamYearResultRow>> {
    let rows = sqlx::query_as(
        "WITH selected_team_events AS (
            SELECT
                te.id AS team_event_id,
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
            WHERE te.team_id = $1
            AND e.competition_id = $2
            AND EXTRACT(YEAR FROM ei.date)::int = $3
        ),
        event_location AS (
            SELECT
                ste.event_instance_id,
                STRING_AGG(lt.name, ', ' ORDER BY lt.depth) AS event_location
            FROM selected_team_events ste
            CROSS JOIN LATERAL get_location_tree(ste.event_location_id) lt
            GROUP BY ste.event_instance_id
        ),
        team_event_stats AS (
            SELECT
                ste.team_event_id,
                COUNT(*) FILTER (WHERE tem.role = 'Contestant')::int4 AS team_total_members,
                COUNT(*) FILTER (
                    WHERE tem.role = 'Contestant'
                    AND m.gender = 'Female'
                )::int4 AS team_female_members
            FROM selected_team_events ste
            LEFT JOIN team_event_member tem ON tem.team_event_id = ste.team_event_id
            LEFT JOIN member m ON m.id = tem.member_id
            GROUP BY ste.team_event_id
        )
        SELECT
            COALESCE(tes.team_total_members, 0) AS team_total_members,
            COALESCE(tes.team_female_members, 0) AS team_female_members,

            ste.event_id,
            ste.event_name,
            ste.event_level,
            ste.event_date,
            COALESCE(el.event_location, '') AS event_location,
            ste.event_scope,
            ste.team_event_rank
        FROM selected_team_events ste
        LEFT JOIN team_event_stats tes ON tes.team_event_id = ste.team_event_id
        LEFT JOIN event_location el ON el.event_instance_id = ste.event_instance_id
        ORDER BY ste.team_event_rank, ste.event_level, ste.event_name, ste.event_date",
    )
    .bind(team_id)
    .bind(competition_id)
    .bind(year)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
