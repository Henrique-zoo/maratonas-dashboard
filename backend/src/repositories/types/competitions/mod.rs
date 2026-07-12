//! # `backend::repositories::types::competitions`
//!
//! ## Responsabilidade
//! Agrupa projeções tipadas retornadas pelas queries SQL.
//!
//! ## Lógica de Implementação
//! Organiza structs `FromRow` por domínio para separar claramente formato de banco e DTO externo.
//!
//! ## Submódulos
//! - `competition_events_by_year`: organiza uma parte especializada deste escopo.
//! - `competition_stats`: organiza uma parte especializada deste escopo.
//! - `competition_structure`: organiza uma parte especializada deste escopo.
//! - `competition_team_year_result`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod competition_events_by_year;
mod competition_stats;
mod competition_structure;
mod competition_team_year_result;

pub use competition_events_by_year::*;
pub use competition_stats::*;
pub use competition_structure::*;
pub use competition_team_year_result::*;
