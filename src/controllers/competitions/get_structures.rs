//! # `backend::controllers::competitions::get_structures`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `competitions`.
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

use crate::{AppState, dtos::competitions::requests::StructuresQuery, services};

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
