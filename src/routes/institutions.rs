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

use crate::{AppState, controllers};

/// Cria o roteador do domínio de instituições.
///
/// Endpoints registrados:
/// - `GET /institutions/options`
/// - `GET /institutions/structures`
/// - `GET /institutions/{institution_id}/events/{event_id}`
///
/// Cada rota delega para handlers em `controllers::institutions`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/institutions/options",
            get(controllers::institutions::get_options),
        )
        .route(
            "/institutions/structures",
            get(controllers::institutions::get_structures),
        )
        .route(
            "/institutions/{institution_id}/events/{event_id}",
            get(controllers::institutions::get_event_performance_over_time),
        )
}
