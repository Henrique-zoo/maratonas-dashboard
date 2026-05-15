//! # `backend::controllers::competitions::get_options`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Extrai parâmetros (`Path`, `Query` e `State`), delega ao service correspondente e transforma o resultado em `Json`/`IntoResponse`.
//!
//! ## Funções
//! - `get_options`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{AppState, dtos::competitions::requests::OptionsQuery, services};

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
