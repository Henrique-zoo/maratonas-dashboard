//! # `backend::repositories::registry`
//!
//! ## Responsabilidade
//! Define o repositório concreto da aplicação.
//!
//! ## Lógica de Implementação
//! Encapsula o `PgPool` em `Registry`, servindo como dependência única para as implementações dos traits de repositório.
//!
//! ## Funções
//! - `new`: Construtor utilitário para inicializar a estrutura com dependências obrigatórias.
//!
//! ## Tipos
//! - `Registry`: Struct utilizada para modelar dados deste domínio.
//!
use sqlx::PgPool;

/// Repositório concreto da aplicação.
///
/// O `Registry` encapsula o pool PostgreSQL e implementa os traits de
/// repositório usados pelos services. Ele mantém o acesso à infraestrutura de
/// persistência fora das regras de negócio.
#[derive(Debug, Clone)]
pub struct Registry {
    /// Pool PostgreSQL compartilhado pelas consultas dos repositórios.
    pub(super) pool: PgPool,
}

impl Registry {
    /// Cria um repositório concreto a partir de um pool PostgreSQL.
    ///
    /// O `Registry` centraliza a dependência de banco usada pelas
    /// implementações dos traits de repositório. Clones do registry
    /// compartilham o mesmo [`PgPool`], conforme a semântica do próprio
    /// pool do `sqlx`.
    ///
    /// # Parâmetros
    /// - `pool`: pool de conexões PostgreSQL já configurado pela camada de
    ///   inicialização da aplicação.
    ///
    /// # Retorno
    /// Um [`Registry`] pronto para ser injetado nos handlers e services.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
