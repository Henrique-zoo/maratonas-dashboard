//! # `backend::routes::teams`
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

/// Cria o roteador do domínio de times.
///
/// Endpoints registrados:
/// - `GET /teams/options`
/// - `GET /teams/structures`
/// - `GET /teams/{team_id}/competitions/{competition_id}`
///
/// Cada rota delega para handlers em `controllers::teams`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/teams/options", get(controllers::teams::get_options))
        .route("/teams/structures", get(controllers::teams::get_structures))
        .route(
            "/teams/{team_id}/competitions/{competition_id}",
            get(controllers::teams::get_structure_by_year),
        )
}
