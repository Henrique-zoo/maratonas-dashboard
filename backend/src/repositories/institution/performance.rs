//! # `backend::repositories::institution::performance`
//!
//! ## Responsabilidade
//! Implementa consultas do repositório de `institution`.
//!
//! ## Lógica de Implementação
//! Executa consultas SQL analíticas com CTEs, agregações e árvore de localização para retornar linhas tipadas com alta densidade de dados.
//!
//! ## Funções
//! - `find_event_performance_over_time`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{
    errors::AppResult,
    repositories::{Registry, types::institutions::EventPerformanceRow},
};

/// Busca a série histórica de desempenho de uma instituição em um evento.
///
/// Para cada ano no intervalo informado, a query calcula o melhor ranking da
/// instituição, identifica o time responsável por esse melhor resultado e
/// calcula a média de rankings dos times da instituição naquele ano.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `institution_id`: instituição analisada.
/// - `event_id`: evento usado como recorte.
/// - `start_year`: primeiro ano incluído na consulta.
/// - `end_year`: último ano incluído na consulta.
///
/// # Retorno
/// Vetor de [`EventPerformanceRow`] ordenado por ano.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_event_performance_over_time(
    repo: &Registry,
    institution_id: i32,
    event_id: i32,
    start_year: i32,
    end_year: i32,
) -> AppResult<Vec<EventPerformanceRow>> {
    let rows = sqlx::query_as(
        "WITH selected_event_teams AS (
            SELECT
                i.id AS institution_id,
                e.id AS event_id,
                EXTRACT(YEAR FROM ei.date)::int AS year,
                t.id AS team_id,
                t.name AS team_name,
                te.rank AS rank
            FROM event e
            JOIN event_instance ei ON ei.event_id = e.id
            JOIN team_event te ON te.event_instance_id = ei.id
            JOIN team t ON t.id = te.team_id
            JOIN institution i ON i.id = t.institution_id
            WHERE i.id = $1
                AND e.id = $2
                AND EXTRACT(YEAR FROM ei.date)::int BETWEEN $3 AND $4
        ),
        ranked AS (
            SELECT
                year,
                team_id,
                team_name,
                rank,
                ROW_NUMBER() OVER (
                    PARTITION BY year
                    ORDER BY rank ASC, team_id ASC
                ) AS rn,
                AVG(rank) OVER (PARTITION BY year)::float4 AS medium_performance_rank
            FROM selected_event_teams
        )
        SELECT
            year,
            rank AS best_performance_rank,
            team_id AS best_performance_team_id,
            team_name AS best_performance_team_name,
            medium_performance_rank
        FROM ranked
        WHERE rn = 1
        ORDER BY year",
    )
    .bind(institution_id)
    .bind(event_id)
    .bind(start_year)
    .bind(end_year)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
