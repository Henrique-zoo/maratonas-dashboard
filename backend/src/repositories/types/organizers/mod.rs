//! # `backend::repositories::types::organizers`
//!
//! ## Responsabilidade
//! Agrupa projeções tipadas retornadas pelas queries SQL.
//!
//! ## Lógica de Implementação
//! Organiza structs `FromRow` por domínio para separar claramente formato de banco e DTO externo.
//!
//! ## Submódulos
//! - `organizer_structure`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod organizer_structure;

pub use organizer_structure::OrganizerStructureRow;
