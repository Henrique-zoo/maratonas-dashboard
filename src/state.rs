//! # `backend::state`
//!
//! ## Responsabilidade
//! Define o estado global injetado nos handlers Axum.
//!
//! ## Lógica de Implementação
//! Encapsula o [`Registry`] de repositórios e oferece um construtor a partir de [`PgPool`] para centralizar inicialização de dependências.
//!
//! ## Funções
//! - `new`: Construtor utilitário para inicializar a estrutura com dependências obrigatórias.
//!
//! ## Tipos
//! - `AppState`: Estado compartilhado injetado nos handlers, contendo acesso aos repositórios.
//!
use sqlx::PgPool;

use crate::repositories::Registry;

/// Estado compartilhado pelos handlers HTTP.
///
/// O `AppState` é injetado pelo Axum em cada rota e centraliza o acesso aos
/// repositórios concretos da aplicação.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Registry de repositórios usado pelos controllers para chamar os services.
    pub repo: Registry,
}

impl AppState {
    /// Cria o estado da aplicação a partir do pool PostgreSQL.
    ///
    /// # Parâmetros
    /// - `pool`: pool de conexões configurado no boot da aplicação.
    ///
    /// # Retorno
    /// Um [`AppState`] contendo o `Registry` construído sobre o pool recebido.
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: Registry::new(pool),
        }
    }
}
