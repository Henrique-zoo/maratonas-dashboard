//! # `backend::controllers::events`
//!
//! ## Responsabilidade
//! Agrupa os controllers por domínio da API.
//!
//! ## Lógica de Implementação
//! Expõe handlers HTTP pequenos e orientados a caso de uso.
//!
//! ## Funções
//! - `get_location_stats`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
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
    dtos::common::requests::{IdPath, LocationYearQuery, YearQuery},
    services,
};

/// Retorna estatísticas de evento agrupadas por localização.
///
/// Extrai o ID do evento do path e `location_type`/`year` da query string,
/// delegando a validação e consulta ao service de eventos.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `path`: path com o identificador do evento.
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
    services::events::get_location_stats(&state.repo, path.id, query.location_type, query.year)
        .await
        .map(Json)
}

/// Retorna estatísticas anuais consolidadas de um evento.
///
/// Extrai o ID do evento do path e o ano da query string, delegando a
/// validação ao service de eventos.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `path`: path com o identificador do evento.
/// - `query`: query com o ano de referência.
///
/// # Retorno
/// Resposta JSON com totais anuais ou erro convertido por `IntoResponse`.
pub async fn get_stats_by_year(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<YearQuery>,
) -> impl IntoResponse {
    services::events::get_stats_by_year(&state.repo, path.id, query.year)
        .await
        .map(Json)
}
