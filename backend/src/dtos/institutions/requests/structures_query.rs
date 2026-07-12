//! # `backend::dtos::institutions::requests::structures_query`
//!
//! ## Responsabilidade
//! Define DTOs de entrada do domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Modela parâmetros deserializados de query/path para validação e tipagem forte antes da camada de serviço.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `StructuresQuery`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//!
use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

/// Parâmetros de query para recuperar estruturas de instituições.
///
/// Modela a lista de instituições solicitadas pelo cliente no endpoint de
/// estruturas, preservando a desserialização CSV fora da camada de serviço.
#[derive(Deserialize)]
pub struct StructuresQuery {
    /// Lista opcional de instituições que devem compor a resposta.
    #[serde(default)]
    pub institution_ids: CsvOptVec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structures_query_accepts_missing_institution_filter() {
        let query: StructuresQuery = serde_urlencoded::from_str("").unwrap();

        assert_eq!(query.institution_ids.into_inner(), None);
    }

    #[test]
    fn structures_query_parses_institution_ids_csv() {
        let query: StructuresQuery = serde_urlencoded::from_str("institution_ids=5,6").unwrap();

        assert_eq!(query.institution_ids.into_inner(), Some(vec![5, 6]));
    }
}
