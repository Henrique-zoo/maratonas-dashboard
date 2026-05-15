//! # `backend::controllers::institutions::get_options`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `institutions`.
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

use crate::{AppState, dtos::institutions::requests::OptionsQuery, services};

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
    Query(filter): Query<OptionsQuery>,
) -> impl IntoResponse {
    services::institutions::get_options(&state.repo, filter.competition_ids.into_inner())
        .await
        .map(Json)
}
