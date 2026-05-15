//! # `backend::dtos::competitions::requests`
//!
//! ## Responsabilidade
//! Define DTOs de entrada do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Modela parâmetros deserializados de query/path para validação e tipagem forte antes da camada de serviço.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `OptionsQuery`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//! - `StructuresQuery`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//!
use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

/// Parâmetros de query para listar competições disponíveis como opções.
///
/// Permite restringir o catálogo de competições aos organizadores informados
/// antes que os dados sejam transformados em [`crate::dtos::common::responses::OptionItem`].
#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    /// Lista opcional de organizadores usada como filtro da consulta.
    #[serde(default)]
    pub organizer_ids: CsvOptVec<i32>,
}

/// Parâmetros de query para recuperar estruturas de competições.
///
/// Modela a lista de competições solicitadas pelo cliente no endpoint de
/// estruturas, mantendo a desserialização CSV isolada da camada de serviço.
#[derive(Debug, Deserialize)]
pub struct StructuresQuery {
    /// Lista opcional de competições que devem compor a resposta.
    #[serde(default)]
    pub competition_ids: CsvOptVec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_query_accepts_missing_organizer_filter() {
        let query: OptionsQuery = serde_urlencoded::from_str("").unwrap();

        assert_eq!(query.organizer_ids.into_inner(), None);
    }

    #[test]
    fn structures_query_parses_competition_ids_csv() {
        let query: StructuresQuery = serde_urlencoded::from_str("competition_ids=10,11").unwrap();

        assert_eq!(query.competition_ids.into_inner(), Some(vec![10, 11]));
    }
}
