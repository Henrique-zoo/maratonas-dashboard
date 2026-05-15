//! # `backend::repositories::competition::options`
//!
//! ## Responsabilidade
//! Implementa consultas do repositório de `competition`.
//!
//! ## Lógica de Implementação
//! Executa consultas SQL analíticas com CTEs, agregações e árvore de localização para retornar linhas tipadas com alta densidade de dados.
//!
//! ## Funções
//! - `find_options_by_organizers`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{
    errors::AppResult,
    repositories::{Registry, types::IdNameRow},
};

/// Busca competições disponíveis para seleção.
///
/// Quando `organizer_ids` é `Some`, a consulta restringe o resultado às
/// competições vinculadas aos organizadores informados. Quando é `None`,
/// retorna todas as competições cadastradas. Em ambos os casos o resultado é
/// ordenado alfabeticamente por nome.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
/// - `organizer_ids`: lista opcional de organizadores usada como filtro.
///
/// # Retorno
/// Vetor de [`IdNameRow`] com `id` e `name` das competições encontradas.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação, bind ou execução da
/// query.
pub(super) async fn find_options_by_organizers(
    repo: &Registry,
    organizer_ids: Option<Vec<i32>>,
) -> AppResult<Vec<IdNameRow>> {
    let rows = if let Some(ids) = organizer_ids {
        sqlx::query_as(
            "SELECT
                id, name
            FROM competition
            WHERE organizer_id = ANY($1::int[])
            ORDER BY name",
        )
        .bind(ids)
        .fetch_all(&repo.pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT
                id, name
            FROM competition
            ORDER BY name",
        )
        .fetch_all(&repo.pool)
        .await?
    };

    Ok(rows)
}
