//! # `backend::controllers::competitions`
//!
//! ## Responsabilidade
//! Agrupa os controllers por domínio da API.
//!
//! ## Lógica de Implementação
//! Expõe submódulos especializados para manter handlers pequenos e orientados a caso de uso.
//!
//! ## Funções
//! - `get_options`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_structures`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_location_stats`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_stats_by_year`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
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
        common::requests::{IdPath, LocationYearQuery, YearQuery},
        competitions::requests::{OptionsQuery, StructuresQuery},
    },
    services,
};

/// Retorna opções de competições para filtros da API.
///
/// Extrai filtros de organizadores da query string e delega a regra de
/// obtenção ao service de competições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filter`: query com filtros opcionais de organizadores.
///
/// # Retorno
/// Resposta JSON com a lista de opções ou erro convertido por `IntoResponse`.
pub async fn get_options(
    State(state): State<AppState>,
    Query(filter): Query<OptionsQuery>,
) -> impl IntoResponse {
    services::competitions::get_options(&state.repo, filter.organizer_ids.into_inner())
        .await
        .map(Json)
}

/// Retorna estruturas completas das competições solicitadas.
///
/// Extrai os IDs de competições da query string e delega a montagem da árvore
/// ao service de competições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filter`: query com a lista opcional de competições.
///
/// # Retorno
/// Resposta JSON com as estruturas de competições ou erro convertido por
/// `IntoResponse`.
pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::competitions::get_structures(&state.repo, filter.competition_ids.into_inner())
        .await
        .map(Json)
}

/// Retorna estatísticas de competição agrupadas por localização.
///
/// Extrai o ID da competição do path e `location_type`/`year` da query string,
/// delegando a validação e consulta ao service de competições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `path`: path com o identificador da competição.
/// - `query`: query com tipo de localização e ano.
///
/// # Retorno
/// Resposta JSON com estatísticas por localização ou erro convertido por
/// `IntoResponse`.
pub async fn get_location_stats(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<LocationYearQuery>,
) -> impl IntoResponse {
    services::competitions::get_location_stats(
        &state.repo,
        path.id,
        query.location_type,
        query.year,
    )
    .await
    .map(Json)
}

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

/// Retorna a estrutura anual de uma competição.
///
/// Extrai o ID da competição do path e o ano da query string, delegando a
/// validação e montagem da resposta ao service de competições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `id`: identificador da competição no path.
/// - `query`: query com o ano de referência.
///
/// # Retorno
/// Resposta JSON com a estrutura anual ou erro convertido por `IntoResponse`.
pub async fn get_structure_by_year(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(query): Query<YearQuery>,
) -> impl IntoResponse {
    services::competitions::get_structure_by_year(&state.repo, id, query.year)
        .await
        .map(Json)
}
