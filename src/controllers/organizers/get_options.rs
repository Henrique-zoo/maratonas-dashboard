//! # `backend::controllers::organizers::get_options`
//!
//! ## Responsabilidade
//! Implementa handlers HTTP do domínio `organizers`.
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
use axum::{Json, debug_handler, extract::State, response::IntoResponse};

use crate::{AppState, services};

/// Retorna opções de organizadores para filtros da API.
///
/// Usa o registry do estado compartilhado e delega a consulta ao service de
/// organizadores.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
///
/// # Retorno
/// Resposta JSON com a lista de opções ou erro convertido por `IntoResponse`.
#[debug_handler]
pub async fn get_options(State(state): State<AppState>) -> impl IntoResponse {
    services::organizers::get_options(&state.repo)
        .await
        .map(Json)
}
