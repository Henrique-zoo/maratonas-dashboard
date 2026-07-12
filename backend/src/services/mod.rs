//! # `backend::services`
//!
//! ## Responsabilidade
//! Reúne os casos de uso da aplicação por domínio.
//!
//! ## Lógica de Implementação
//! Mantém a camada de serviços como fronteira de regra de negócio entre controllers e repositórios.
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
