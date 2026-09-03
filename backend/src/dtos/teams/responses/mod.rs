//! # `backend::dtos::teams::responses`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `teams`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Submódulos
//! - `competition_year_structure`: organiza uma parte especializada deste escopo.
//! - `structure`: organiza uma parte especializada deste escopo.
//! - `team_option`: define opções de equipe com contexto institucional.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod competition_year_structure;
mod structure;
mod team_option;

pub use competition_year_structure::*;
pub use structure::*;
pub use team_option::*;
