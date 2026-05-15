//! # `backend::repositories::event`
//!
//! ## Responsabilidade
//! Conecta o trait de `event` ao `Registry`.
//!
//! ## Lógica de Implementação
//! Implementa métodos do trait delegando para submódulos focados em SQL (`options`, `stats`, `structures` e variações).
//!
//! ## Submódulos
//! - `stats`: organiza uma parte especializada deste escopo.
//! - `trait_def`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod stats;
mod trait_def;

pub use trait_def::EventRepository;
#[cfg(test)]
pub use trait_def::MockEventRepository;
