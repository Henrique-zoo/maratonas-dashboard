//! # `backend::controllers::teams`
//!
//! ## Responsabilidade
//! Agrupa os controllers por domínio da API.
//!
//! ## Lógica de Implementação
//! Expõe handlers HTTP pequenos e orientados a caso de uso.
//!
//! ## Funções
//! - `get_options`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_structures`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
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
    dtos::{
        common::requests::YearQuery,
        teams::requests::{CompetitionStructurePath, OptionsQuery, StructuresQuery},
    },
    services,
};

/// Retorna opções de times para filtros da API.
///
/// Extrai filtros opcionais de competições e instituições da query string e
/// delega a regra de obtenção ao service de times.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filters`: query com filtros opcionais de competições e instituições.
///
/// # Retorno
/// Resposta JSON com a lista de opções ou erro convertido por `IntoResponse`.
pub async fn get_options(
    State(state): State<AppState>,
    Query(filters): Query<OptionsQuery>,
) -> impl IntoResponse {
    services::teams::get_options(
        &state.repo,
        filters.competition_ids.into_inner(),
        filters.institution_ids.into_inner(),
    )
    .await
    .map(Json)
}

/// Retorna estruturas completas dos times solicitados.
///
/// Extrai os IDs de times da query string e delega a montagem da árvore ao
/// service de times.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filter`: query com a lista opcional de times.
///
/// # Retorno
/// Resposta JSON com as estruturas de times ou erro convertido por
/// `IntoResponse`.
pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::teams::get_structures(&state.repo, filter.team_ids.into_inner())
        .await
        .map(Json)
}

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
