//! # `backend::dtos::common`
//!
//! ## Responsabilidade
//! Agrupa DTOs do domínio `common`.
//!
//! ## Lógica de Implementação
//! Conecta módulos de request/response para compor o contrato HTTP público desse domínio.
//!
//! ## Submódulos
//! - `requests`: organiza uma parte especializada deste escopo.
//! - `responses`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
pub mod requests;
pub mod responses;
