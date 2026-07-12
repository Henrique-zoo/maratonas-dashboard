//! # `backend::controllers`
//!
//! ## Responsabilidade
//! Agrupa os controllers por domínio da API.
//!
//! ## Lógica de Implementação
//! Expõe submódulos especializados para manter handlers pequenos e orientados a caso de uso.
//!
//! ## Submódulos
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
pub mod competitions;
pub mod events;
pub mod institutions;
pub mod organizers;
pub mod teams;
