//! # `backend::repositories::types::teams`
//!
//! ## Responsabilidade
//! Agrupa projeções tipadas retornadas pelas queries SQL.
//!
//! ## Lógica de Implementação
//! Organiza structs `FromRow` por domínio para separar claramente formato de banco e DTO externo.
//!
//! ## Submódulos
//! - `team_option_row`: representa opções de equipe com contexto institucional.
//! - `team_structure_row`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod team_option_row;
mod team_structure_row;

pub use team_option_row::TeamOptionRow;
pub use team_structure_row::TeamStructureRow;
