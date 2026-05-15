//! # `backend::controllers::teams::get_structures`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `teams`.
//!
//! ## Lógica de Implementação
//! Extrai parâmetros (`Path`, `Query` e `State`), delega ao service correspondente e transforma o resultado em `Json`/`IntoResponse`.
//!
//! ## Funções
//! - `get_structures`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::teams::requests::StructuresQuery, services};

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
