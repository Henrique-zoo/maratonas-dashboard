//! # `backend::controllers::teams::get_structure_by_year`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `teams`.
//!
//! ## Lógica de Implementação
//! Extrai parâmetros (`Path`, `Query` e `State`), delega ao service correspondente e transforma o resultado em `Json`/`IntoResponse`.
//!
//! ## Funções
//! - `get_structure_by_year`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
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
    dtos::{common::requests::YearQuery, teams::requests::CompetitionStructurePath},
    services,
};

/// Retorna a estrutura anual de um time em uma competição.
///
/// Extrai time e competição do path e o ano da query string, delegando a
/// validação e montagem da resposta ao service de times.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `path`: path com `team_id` e `competition_id`.
/// - `query`: query com o ano de referência.
///
/// # Retorno
/// Resposta JSON com a estrutura anual ou erro convertido por `IntoResponse`.
pub async fn get_structure_by_year(
    State(state): State<AppState>,
    Path(path): Path<CompetitionStructurePath>,
    Query(query): Query<YearQuery>,
) -> impl IntoResponse {
    services::teams::get_structure_by_year(
        &state.repo,
        path.team_id,
        path.competition_id,
        query.year,
    )
    .await
    .map(Json)
}
