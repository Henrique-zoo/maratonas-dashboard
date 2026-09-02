//! # `backend::repositories::institution::options`
//!
//! ## Responsabilidade
//! Implementa consultas do repositório de `institution`.
//!
//! ## Lógica de Implementação
//! Executa consultas SQL analíticas com CTEs, agregações e árvore de localização para retornar linhas tipadas com alta densidade de dados.
//!
//! ## Funções
//! - `find_event_options`: Lista eventos e anos em que uma instituição possui participação registrada.
//! - `find_options_by_competitions`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        types::{IdNameRow, institutions::InstitutionEventOptionRow},
    },
};

/// Lista todos os eventos lógicos em que a instituição participou, com a
/// competição e os anos disponíveis para análise histórica.
pub(super) async fn find_event_options(
    repo: &Registry,
    institution_id: i32,
) -> AppResult<Vec<InstitutionEventOptionRow>> {
    let rows = sqlx::query_as(
        "SELECT
            e.id AS event_id,
            e.name AS event_name,
            c.id AS competition_id,
            c.name AS competition_name,
            ARRAY_AGG(
                DISTINCT EXTRACT(YEAR FROM ei.date)::int
                ORDER BY EXTRACT(YEAR FROM ei.date)::int
            ) AS years
        FROM institution i
        JOIN team t ON t.institution_id = i.id
        JOIN team_event te ON te.team_id = t.id
        JOIN event_instance ei ON ei.id = te.event_instance_id
        JOIN event e ON e.id = ei.event_id
        JOIN competition c ON c.id = e.competition_id
        WHERE i.id = $1
        GROUP BY e.id, e.name, c.id, c.name
        ORDER BY c.name, e.name",
    )
    .bind(institution_id)
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}

/// Busca instituições disponíveis para seleção.
///
/// Quando `competition_ids` é `Some`, a consulta retorna apenas instituições
/// com participação nas competições informadas. Quando é `None`, retorna todas
/// as instituições cadastradas. Em ambos os casos o resultado é ordenado por
/// nome.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `competition_ids`: lista opcional de competições usada como filtro.
///
/// # Retorno
/// Vetor de [`IdNameRow`] com `id` e `name` das instituições encontradas.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_options_by_competitions(
    repo: &Registry,
    competition_ids: Option<Vec<i32>>,
) -> AppResult<Vec<IdNameRow>> {
    let rows = if let Some(ids) = competition_ids {
        sqlx::query_as(
            "SELECT DISTINCT
                i.id AS id,
                i.name AS name
            FROM institution i
            JOIN team t ON t.institution_id = i.id
            JOIN team_event te ON te.team_id = t.id
            JOIN event_instance ei ON ei.id = te.event_instance_id
            JOIN event e ON e.id = ei.event_id
            JOIN competition c ON e.competition_id = c.id
            WHERE c.id = ANY($1::int[])
            ORDER BY i.name",
        )
        .bind(ids)
        .fetch_all(&repo.pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT
                id, name
            FROM institution
            ORDER BY name",
        )
        .fetch_all(&repo.pool)
        .await?
    };

    Ok(rows)
}
