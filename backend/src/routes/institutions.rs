//! # `backend::routes::institutions`
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

/// Cria o roteador do domínio de instituições.
///
/// Endpoints registrados:
/// - `GET /institutions/options`
/// - `GET /institutions/structures`
/// - `GET /institutions/{id}/events/options`
/// - `GET /institutions/{institution_id}/events/{event_id}/performance`
///
/// Cada rota delega para handlers em `controllers::institutions`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/institutions/options",
            get(handlers::institutions::get_options),
        )
        .route(
            "/institutions/structures",
            get(handlers::institutions::get_structures),
        )
        .route(
            "/institutions/{id}/events/options",
            get(handlers::institutions::get_event_options),
        )
        .route(
            "/institutions/{institution_id}/events/{event_id}/performance",
            get(handlers::institutions::get_event_performance_over_time),
        )
}
