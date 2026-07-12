//! # `backend::repositories::team`
//!
//! ## Responsabilidade
//! Conecta o trait de `team` ao `Registry`.
//!
//! ## Lógica de Implementação
//! Implementa métodos do trait delegando para submódulos focados em SQL (`options`, `stats`, `structures` e variações).
//!
//! ## Submódulos
//! - `options`: organiza uma parte especializada deste escopo.
//! - `structures`: organiza uma parte especializada deste escopo.
//! - `trait_def`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod options;
mod structures;
mod trait_def;

#[cfg(test)]
pub use trait_def::MockTeamRepository;
pub use trait_def::TeamRepository;
