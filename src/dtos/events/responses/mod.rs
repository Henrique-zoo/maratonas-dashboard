//! # `backend::dtos::events::responses`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `events`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Submódulos
//! - `event_location_stats`: organiza uma parte especializada deste escopo.
//! - `event_year_stats`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod event_location_stats;
mod event_year_stats;

pub use event_location_stats::*;
pub use event_year_stats::*;
