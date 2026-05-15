//! # `backend::dtos::organizers::responses`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `organizers`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Submódulos
//! - `competition_year_structure`: organiza uma parte especializada deste escopo.
//! - `structure`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod competition_year_structure;
mod structure;

pub use competition_year_structure::*;
pub use structure::*;
