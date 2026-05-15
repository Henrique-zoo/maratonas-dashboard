//! # `backend::controllers::events::get_location_stats`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `events`.
//!
//! ## Lógica de Implementação
//! Extrai parâmetros (`Path`, `Query` e `State`), delega ao service correspondente e transforma o resultado em `Json`/`IntoResponse`.
//!
//! ## Funções
//! - `get_location_stats`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
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
    dtos::common::requests::{IdPath, LocationYearQuery},
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
