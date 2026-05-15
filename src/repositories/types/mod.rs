//! # `backend::repositories::types`
//!
//! ## Responsabilidade
//! Agrupa projeções tipadas retornadas pelas queries SQL.
//!
//! ## Lógica de Implementação
//! Organiza structs `FromRow` por domínio para separar claramente formato de banco e DTO externo.
//!
//! ## Submódulos
//! - `competitions`: organiza uma parte especializada deste escopo.
//! - `events`: organiza uma parte especializada deste escopo.
//! - `id_name_row`: organiza uma parte especializada deste escopo.
//! - `institutions`: organiza uma parte especializada deste escopo.
//! - `organizers`: organiza uma parte especializada deste escopo.
//! - `teams`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
pub mod competitions;
pub mod events;
mod id_name_row;
pub mod institutions;
pub mod organizers;
pub mod teams;

pub use id_name_row::IdNameRow;
