//! # `backend::dtos::institutions::requests`
//!
//! ## Responsabilidade
//! Define DTOs de entrada do domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Modela parâmetros deserializados de query/path para validação e tipagem forte antes da camada de serviço.
//!
//! ## Submódulos
//! - `performance_over_time_query`: organiza uma parte especializada deste escopo.
//! - `structures_query`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `OptionsQuery`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//!
mod performance_over_time_query;
mod structures_query;

use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

pub use performance_over_time_query::{EventPerformancePath, EventPerformanceQuery};
pub use structures_query::StructuresQuery;

/// Parâmetros de query para listar instituições disponíveis como opções.
///
/// Permite filtrar instituições pelas competições informadas antes da
/// conversão para itens compactos de seleção.
#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    /// Lista opcional de competições usada como filtro da consulta.
    #[serde(default)]
    pub competition_ids: CsvOptVec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_query_accepts_missing_competition_filter() {
        let query: OptionsQuery = serde_urlencoded::from_str("").unwrap();

        assert_eq!(query.competition_ids.into_inner(), None);
    }

    #[test]
    fn options_query_parses_competition_ids_csv() {
        let query: OptionsQuery = serde_urlencoded::from_str("competition_ids=10,11").unwrap();

        assert_eq!(query.competition_ids.into_inner(), Some(vec![10, 11]));
    }
}
