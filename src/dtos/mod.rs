//! # `backend::dtos`
//!
//! ## Responsabilidade
//! Organiza contratos HTTP da API.
//!
//! ## Lógica de Implementação
//! Agrupa DTOs de entrada/saída por domínio para manter a borda HTTP desacoplada dos modelos de persistência.
//!
//! ## Submódulos
//! - `common`: organiza uma parte especializada deste escopo.
//! - `competitions`: organiza uma parte especializada deste escopo.
//! - `events`: organiza uma parte especializada deste escopo.
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
pub mod common;
pub mod competitions;
pub mod events;
pub mod institutions;
pub mod organizers;
pub mod teams;
