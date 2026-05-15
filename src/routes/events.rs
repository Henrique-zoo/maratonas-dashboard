//! # `backend::routes::events`
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

use crate::{AppState, controllers};

/// Cria o roteador do domínio de eventos.
///
/// Endpoints registrados:
/// - `GET /events/{id}/location_stats`
/// - `GET /events/{id}/stats`
///
/// Cada rota delega para handlers em `controllers::events`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/events/{id}/location_stats",
            get(controllers::events::get_location_stats),
        )
        .route(
            "/events/{id}/stats",
            get(controllers::events::get_stats_by_year),
        )
}
