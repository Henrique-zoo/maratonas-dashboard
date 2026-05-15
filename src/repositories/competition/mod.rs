//! # `backend::repositories::competition`
//!
//! ## Responsabilidade
//! Conecta o trait de `competition` ao `Registry`.
//!
//! ## Lógica de Implementação
//! Implementa métodos do trait delegando para submódulos focados em SQL (`options`, `stats`, `structures` e variações).
//!
//! ## Submódulos
//! - `options`: organiza uma parte especializada deste escopo.
//! - `stats`: organiza uma parte especializada deste escopo.
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
mod stats;
mod structures;
mod trait_def;

pub use trait_def::CompetitionRepository;

#[cfg(test)]
pub use trait_def::MockCompetitionRepository;
