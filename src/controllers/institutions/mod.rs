//! # `backend::controllers::institutions`
//!
//! ## Responsabilidade
//! Agrupa os controllers por domínio da API.
//!
//! ## Lógica de Implementação
//! Expõe submódulos especializados para manter handlers pequenos e orientados a caso de uso.
//!
//! ## Submódulos
//! - `get_event_performance_over_time`: organiza uma parte especializada deste escopo.
//! - `get_options`: organiza uma parte especializada deste escopo.
//! - `get_structures`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod get_event_performance_over_time;
mod get_options;
mod get_structures;

pub use get_event_performance_over_time::get_event_performance_over_time;
pub use get_options::get_options;
pub use get_structures::get_structures;
