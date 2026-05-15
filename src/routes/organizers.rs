//! # `backend::routes::organizers`
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

/// Cria o roteador do domínio de organizadores.
///
/// Endpoints registrados:
/// - `GET /organizers/options`
/// - `GET /organizers/structures`
/// - `GET /organizers/competitions/{id}/structure`
///
/// Cada rota delega para handlers em `controllers::organizers`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/organizers/options",
            get(controllers::organizers::get_options),
        )
        .route(
            "/organizers/structures",
            get(controllers::organizers::get_structures),
        )
        .route(
            "/organizers/competitions/{id}/structure",
            get(controllers::organizers::get_structure_by_year),
        )
}
