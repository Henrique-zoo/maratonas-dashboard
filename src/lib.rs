//! # `backend::lib`
//!
//! ## Responsabilidade
//! Organiza a crate de backend como biblioteca reaproveitável pelo binário e pelos testes.
//!
//! ## Lógica de Implementação
//! Declara os módulos internos da aplicação, expõe as rotas públicas e reexporta o estado compartilhado.
//!
//! ## Submódulos
//! - `controllers`: organiza uma parte especializada deste escopo.
//! - `dtos`: organiza uma parte especializada deste escopo.
//! - `errors`: organiza uma parte especializada deste escopo.
//! - `repositories`: organiza uma parte especializada deste escopo.
//! - `services`: organiza uma parte especializada deste escopo.
//! - `shared`: organiza uma parte especializada deste escopo.
//! - `state`: organiza uma parte especializada deste escopo.
//! - `routes`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod controllers;
mod dtos;
mod errors;
mod repositories;
mod services;
mod shared;
mod state;

pub mod routes;
pub use state::AppState;
