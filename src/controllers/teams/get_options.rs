//! # `backend::controllers::teams::get_options`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `teams`.
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

use crate::{AppState, dtos::teams::requests::OptionsQuery, services};

/// Retorna opções de times para filtros da API.
///
/// Extrai filtros opcionais de competições e instituições da query string e
/// delega a regra de obtenção ao service de times.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `filters`: query com filtros opcionais de competições e instituições.
///
/// # Retorno
/// Resposta JSON com a lista de opções ou erro convertido por `IntoResponse`.
pub async fn get_options(
    State(state): State<AppState>,
    Query(filters): Query<OptionsQuery>,
) -> impl IntoResponse {
    services::teams::get_options(
        &state.repo,
        filters.competition_ids.into_inner(),
        filters.institution_ids.into_inner(),
    )
    .await
    .map(Json)
}
