//! # `backend::repositories::organizer::options`
//!
//! ## Responsabilidade
//! Implementa consultas do repositório de `organizer`.
//!
//! ## Lógica de Implementação
//! Executa consultas SQL analíticas com CTEs, agregações e árvore de localização para retornar linhas tipadas com alta densidade de dados.
//!
//! ## Funções
//! - `find_options`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{
    errors::AppResult,
    repositories::{Registry, types::IdNameRow},
};

/// Busca todos os organizadores disponíveis para seleção.
///
/// A consulta retorna o catálogo compacto de organizadores sem filtros,
/// ordenado alfabeticamente por nome.
///
/// # Parâmetros
/// - `repo`: registry que fornece acesso ao pool PostgreSQL.
///
/// # Retorno
/// Vetor de [`IdNameRow`] com `id` e `name` dos organizadores cadastrados.
///
/// # Erros
/// Propaga erros emitidos pelo `sqlx` durante preparação ou execução da query.
pub(super) async fn find_options(repo: &Registry) -> AppResult<Vec<IdNameRow>> {
    let rows = sqlx::query_as(
        "SELECT
                id, name
            FROM organizer
            ORDER BY name",
    )
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
