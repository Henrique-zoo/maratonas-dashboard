//! # `backend::controllers::institutions::get_event_performance_over_time`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Extrai parâmetros (`Path`, `Query` e `State`), delega ao service correspondente e transforma o resultado em `Json`/`IntoResponse`.
//!
//! ## Funções
//! - `get_event_performance_over_time`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
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
    dtos::institutions::requests::{EventPerformancePath, EventPerformanceQuery},
    services,
};

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
    Path(path): Path<EventPerformancePath>,
    Query(query): Query<EventPerformanceQuery>,
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
