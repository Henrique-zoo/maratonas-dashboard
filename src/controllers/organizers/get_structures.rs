//! # `backend::controllers::organizers::get_structures`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `organizers`.
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

use crate::{AppState, dtos::organizers::requests::StructuresQuery, services};

/// Retorna estruturas completas dos organizadores solicitados.
///
/// Extrai os IDs de organizadores da query string e delega a montagem da
/// árvore ao service de organizadores.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filter`: query com a lista opcional de organizadores.
///
/// # Retorno
/// Resposta JSON com as estruturas de organizadores ou erro convertido por
/// `IntoResponse`.
pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::organizers::get_structures(&state.repo, filter.organizer_ids.into_inner())
        .await
        .map(Json)
}
