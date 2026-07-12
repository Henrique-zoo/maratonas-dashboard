//! # `backend::routes`
//!
//! ## Responsabilidade
//! Compõe o roteamento HTTP completo da API.
//!
//! ## Lógica de Implementação
//! Importa roteadores por domínio e faz `merge` em um `Router<AppState>` único, mantendo a regra de negócio fora da camada de entrada HTTP.
//!
//! ## Submódulos
//! - `competitions`: organiza uma parte especializada deste escopo.
//! - `events`: organiza uma parte especializada deste escopo.
//! - `institutions`: organiza uma parte especializada deste escopo.
//! - `organizers`: organiza uma parte especializada deste escopo.
//! - `teams`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! - `create_router`: Monta e devolve o roteador Axum com os endpoints deste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use axum::Router;

use crate::AppState;

mod competitions;
mod events;
mod institutions;
mod organizers;
mod teams;

/// Constrói o roteador HTTP raiz da aplicação.
///
/// Esta função compõe os roteadores de cada domínio (`competitions`,
/// `organizers`, `institutions`, `teams` e `events`) em uma única árvore de
/// rotas usando `merge`. O resultado é usado no bootstrap do servidor Axum em
/// `main.rs`, já associado ao [`AppState`].
pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(competitions::router())
        .merge(organizers::router())
        .merge(institutions::router())
        .merge(teams::router())
        .merge(events::router())
}
