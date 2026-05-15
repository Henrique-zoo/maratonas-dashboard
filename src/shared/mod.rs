//! # `backend::shared`
//!
//! ## Responsabilidade
//! Agrupa utilitários e tipos transversais compartilhados entre camadas.
//!
//! ## Lógica de Implementação
//! Expõe módulos reutilizáveis de serialização e enums de domínio para reduzir duplicação.
//!
//! ## Submódulos
//! - `serde`: organiza uma parte especializada deste escopo.
//! - `types`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
pub mod serde;
pub mod types;
