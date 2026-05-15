//! # `backend::controllers::institutions::get_structures`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `institutions`.
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

use crate::{AppState, dtos::institutions::requests::StructuresQuery, services};

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
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::institutions::get_structures(&state.repo, filter.institution_ids.into_inner())
        .await
        .map(Json)
}
