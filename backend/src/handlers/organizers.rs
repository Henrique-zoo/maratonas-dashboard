//! # `backend::controllers::organizers`
//!
//! ## Responsabilidade
//! Agrupa os controllers por domínio da API.
//!
//! ## Lógica de Implementação
//! Expõe handlers HTTP pequenos e orientados a caso de uso.
//!
//! ## Funções
//! - `get_options`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_structures`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//! - `get_structure_by_year`: Handler HTTP que extrai dados da requisição, delega ao service e retorna payload serializável.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!

use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    AppState,
    dtos::{
        common::requests::{IdPath, YearQuery},
        organizers::requests::StructuresQuery,
    },
    services,
};

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

/// Retorna a estrutura anual de uma competição na visão de organizadores.
///
/// Extrai o ID da competição do path e o ano da query string, delegando a
/// validação e montagem da resposta ao service de organizadores.
///
/// # Parâmetros
/// - `state`: estado compartilhado da aplicação, contendo o registry.
/// - `path`: path com o identificador da competição.
/// - `query`: query com o ano de referência.
///
/// # Retorno
/// Resposta JSON com a estrutura anual ou erro convertido por `IntoResponse`.
pub async fn get_structure_by_year(
    State(state): State<AppState>,
    Path(path): Path<IdPath>,
    Query(query): Query<YearQuery>,
) -> impl IntoResponse {
    services::organizers::get_structure_by_year(&state.repo, path.id, query.year)
        .await
        .map(Json)
}
