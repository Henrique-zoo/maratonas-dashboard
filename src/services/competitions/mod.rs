//! # `backend::services::competitions`
//!
//! ## Responsabilidade
//! Reúne os casos de uso da aplicação por domínio.
//!
//! ## Lógica de Implementação
//! Mantém a camada de serviços como fronteira de regra de negócio entre controllers e repositórios.
//!
//! ## Submódulos
//! - `get_location_stats`: organiza uma parte especializada deste escopo.
//! - `get_options`: organiza uma parte especializada deste escopo.
//! - `get_stats_by_year`: organiza uma parte especializada deste escopo.
//! - `get_structure_by_year`: organiza uma parte especializada deste escopo.
//! - `get_structures`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
mod get_location_stats;
mod get_options;
mod get_stats_by_year;
mod get_structure_by_year;
mod get_structures;

pub use get_location_stats::get_location_stats;
pub use get_options::get_options;
pub use get_stats_by_year::get_stats_by_year;
pub use get_structure_by_year::get_structure_by_year;
pub use get_structures::get_structures;
