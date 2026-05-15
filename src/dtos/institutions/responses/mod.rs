//! # `backend::dtos::institutions::responses`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Submódulos
//! - `event_performance`: organiza uma parte especializada deste escopo.
//! - `structure`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod event_performance;
mod structure;

pub use event_performance::*;
pub use structure::*;
