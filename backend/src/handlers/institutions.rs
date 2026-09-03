//! # `backend::controllers::institutions`
//!
//! ## Responsabilidade
//! Agrupa os controllers por domínio da API.
//!
//! ## Lógica de Implementação
//! Expõe handlers HTTP pequenos e orientados a caso de uso.
//!
//! ## Funções
//! - `get_event_options`: Handler HTTP que lista os eventos e anos disponíveis para a análise histórica da instituição.
//! - `get_event_performance_over_time`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_options`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_structures`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!

use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    AppState,
    dtos::common::requests::IdPath,
    dtos::institutions::requests::{
        EventPerformancePath, EventPerformanceQuery, OptionsQuery, StructuresQuery,
    },
    extractors::{ApiPath, ApiQuery},
    services,
};

/// Retorna os eventos históricos que podem alimentar a análise da instituição.
pub async fn get_event_options(
    State(state): State<AppState>,
    ApiPath(path): ApiPath<IdPath>,
) -> impl IntoResponse {
    services::institutions::get_event_options(&state.repo, path.id)
        .await
        .map(Json)
}

/// Retorna a série histórica de desempenho de uma instituição em um evento.
///
/// Extrai instituição e evento do path e o intervalo de anos da query string,
/// delegando validação e consulta ao service de instituições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `path`: path com `institution_id` e `event_id`.
/// - `query`: query com `start_year` e `end_year`.
///
/// # Retorno
/// Resposta JSON com a série de desempenho ou erro convertido por
/// `IntoResponse`.
pub async fn get_event_performance_over_time(
    State(state): State<AppState>,
    ApiPath(path): ApiPath<EventPerformancePath>,
    ApiQuery(query): ApiQuery<EventPerformanceQuery>,
) -> impl IntoResponse {
    services::institutions::get_event_performance_over_time(
        &state.repo,
        path.institution_id,
        path.event_id,
        query.start_year,
        query.end_year,
    )
    .await
    .map(Json)
}

/// Retorna opções de instituições para filtros da API.
///
/// Extrai filtros de competições da query string e delega a regra de obtenção
/// ao service de instituições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filter`: query com filtros opcionais de competições.
///
/// # Retorno
/// Resposta JSON com a lista de opções ou erro convertido por `IntoResponse`.
pub async fn get_options(
    State(state): State<AppState>,
    ApiQuery(filter): ApiQuery<OptionsQuery>,
) -> impl IntoResponse {
    services::institutions::get_options(&state.repo, filter.competition_ids.into_inner())
        .await
        .map(Json)
}

/// Retorna estruturas completas das instituições solicitadas.
///
/// Extrai os IDs de instituições da query string e delega a montagem da árvore
/// ao service de instituições.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filter`: query com a lista opcional de instituições.
///
/// # Retorno
/// Resposta JSON com as estruturas de instituições ou erro convertido por
/// `IntoResponse`.
pub async fn get_structures(
    State(state): State<AppState>,
    ApiQuery(filter): ApiQuery<StructuresQuery>,
) -> impl IntoResponse {
    services::institutions::get_structures(&state.repo, filter.institution_ids.into_inner())
        .await
        .map(Json)
}
