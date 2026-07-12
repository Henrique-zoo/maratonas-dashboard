//! # `backend::repositories`
//!
//! ## Responsabilidade
//! Centraliza contratos e implementação concreta de acesso a dados.
//!
//! ## Lógica de Implementação
//! Declara submódulos de persistência por domínio, reexporta traits usados pelos serviços e o `Registry` que contém conexão com banco.
//!
//! ## Submódulos
//! - `competition`: organiza uma parte especializada deste escopo.
//! - `event`: organiza uma parte especializada deste escopo.
//! - `institution`: organiza uma parte especializada deste escopo.
//! - `organizer`: organiza uma parte especializada deste escopo.
//! - `registry`: organiza uma parte especializada deste escopo.
//! - `team`: organiza uma parte especializada deste escopo.
//! - `types`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
// ============= SUB MÓDULOS =============
mod competition;
mod event;
mod institution;
mod organizer;
mod registry;
mod team;
pub(crate) mod types;

/*
*************************************************
***********   ********        *******    ********
**********     *******   ***   ******    ********
*********  ***  ******   ****   *****    ********
********  *****  *****   ***   ******    ********
*******           ****       ********    ********
******   *******   ***    ***********    ********
*****   *********   **    ***********    ********  de repositories
*************************************************
*/
// ============= STRUCTS =============
pub use registry::Registry;
// ============= TRAITS =============
pub use competition::CompetitionRepository;
pub use event::EventRepository;
pub use institution::InstitutionRepository;
pub use organizer::OrganizerRepository;
pub use team::TeamRepository;

// ============= MOCKS (only available in tests) =============
#[cfg(test)]
pub use competition::MockCompetitionRepository;
#[cfg(test)]
pub use event::MockEventRepository;
#[cfg(test)]
pub use institution::MockInstitutionRepository;
#[cfg(test)]
pub use organizer::MockOrganizerRepository;
#[cfg(test)]
pub use team::MockTeamRepository;
