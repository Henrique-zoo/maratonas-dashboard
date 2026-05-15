//! # `backend::controllers::competitions::get_stats_by_year`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Extrai parâmetros (`Path`, `Query` e `State`), delega ao service correspondente e transforma o resultado em `Json`/`IntoResponse`.
//!
//! ## Funções
//! - `get_stats_by_year`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::common::requests::{IdPath, YearQuery},
    services,
};

/// Retorna estatísticas anuais consolidadas de uma competição.
///
/// Extrai o ID da competição do path e o ano da query string, delegando a
/// validação ao service de competições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `path`: path com o identificador da competição.
/// - `query`: query com o ano de referência.
///
/// # Retorno
/// Resposta JSON com totais anuais ou erro convertido por `IntoResponse`.
pub async fn get_stats_by_year(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<YearQuery>,
) -> impl IntoResponse {
    services::competitions::get_stats_by_year(&state.repo, path.id, query.year)
        .await
        .map(Json)
}
