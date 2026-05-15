//! # `backend::repositories::types::institutions`
//!
//! ## Responsabilidade
//! Agrupa projeções tipadas retornadas pelas queries SQL.
//!
//! ## Lógica de Implementação
//! Organiza structs `FromRow` por domínio para separar claramente formato de banco e DTO externo.
//!
//! ## Submódulos
//! - `event_performance_row`: organiza uma parte especializada deste escopo.
//! - `institution_structure_row`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod event_performance_row;
mod institution_structure_row;

pub use event_performance_row::EventPerformanceRow;
pub use institution_structure_row::InstitutionStructureRow;
