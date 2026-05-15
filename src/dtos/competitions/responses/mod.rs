//! # `backend::dtos::competitions::responses`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Submódulos
//! - `competition_structure`: organiza uma parte especializada deste escopo.
//! - `competition_year_location_stats`: organiza uma parte especializada deste escopo.
//! - `competition_year_stats`: organiza uma parte especializada deste escopo.
//! - `competition_year_structure`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod competition_structure;
mod competition_year_location_stats;
mod competition_year_stats;
mod competition_year_structure;

pub use competition_structure::*;
pub use competition_year_location_stats::*;
pub use competition_year_stats::*;
pub use competition_year_structure::*;
