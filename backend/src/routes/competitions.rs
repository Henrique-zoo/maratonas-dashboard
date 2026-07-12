//! # `backend::routes::competitions`
//!
//! ## Responsabilidade
//! Define as rotas HTTP do domínio `core`.
//!
//! ## Lógica de Implementação
//! Constrói um `Router` de domínio, registra paths/métodos e delega o processamento para controllers específicos.
//!
//! ## Funções
//! - `router`: Monta e devolve o roteador Axum com os endpoints deste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use axum::{Router, routing::get};

use crate::{AppState, handlers};

/// Cria o roteador do domínio de competições.
///
/// Endpoints registrados:
/// - `GET /competitions/options`
/// - `GET /competitions/structures`
/// - `GET /competitions/{id}/structure`
/// - `GET /competitions/{id}/stats`
/// - `GET /competitions/{id}/location_stats`
///
/// Cada rota delega para handlers em `controllers::competitions`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/competitions/options",
            get(handlers::competitions::get_options),
        )
        .route(
            "/competitions/structures",
            get(handlers::competitions::get_structures),
        )
        .route(
            "/competitions/{id}/structure",
            get(handlers::competitions::get_structure_by_year),
        )
        .route(
            "/competitions/{id}/stats",
            get(handlers::competitions::get_stats_by_year),
        )
        .route(
            "/competitions/{id}/location_stats",
            get(handlers::competitions::get_location_stats),
        )
}
