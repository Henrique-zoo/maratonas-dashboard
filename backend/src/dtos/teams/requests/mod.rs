//! # `backend::dtos::teams::requests`
//!
//! ## Responsabilidade
//! Define DTOs de entrada do domínio `teams`.
//!
//! ## Lógica de Implementação
//! Modela parâmetros deserializados de query/path para validação e tipagem forte antes da camada de serviço.
//!
//! ## Submódulos
//! - `structures_query`: organiza uma parte especializada deste escopo.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `OptionsQuery`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//! - `CompetitionStructurePath`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//!
mod structures_query;

use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

pub use structures_query::*;

/// Parâmetros de query para listar times disponíveis como opções.
///
/// Permite filtrar times por competições e instituições antes de retornar
/// itens compactos de seleção ao cliente.
#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    /// Lista opcional de competições usada como filtro da consulta.
    #[serde(default)]
    pub competition_ids: CsvOptVec<i32>,
    /// Lista opcional de instituições usada como filtro da consulta.
    #[serde(default)]
    pub institution_ids: CsvOptVec<i32>,
}

/// Parâmetros de rota para detalhar a estrutura de um time em uma competição.
///
/// Identifica o par time/competição usado pelo endpoint que retorna eventos e
/// resultados do time no recorte competitivo selecionado.
#[derive(Debug, Deserialize)]
pub struct CompetitionStructurePath {
    /// Identificador do time analisado.
    pub team_id: i32,
    /// Identificador da competição analisada.
    pub competition_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_query_accepts_missing_filters() {
        let query: OptionsQuery = serde_urlencoded::from_str("").unwrap();

        assert_eq!(query.competition_ids.into_inner(), None);
        assert_eq!(query.institution_ids.into_inner(), None);
    }

    #[test]
    fn options_query_parses_competition_and_institution_filters() {
        let query: OptionsQuery =
            serde_urlencoded::from_str("competition_ids=10,11&institution_ids=5,6").unwrap();

        assert_eq!(query.competition_ids.into_inner(), Some(vec![10, 11]));
        assert_eq!(query.institution_ids.into_inner(), Some(vec![5, 6]));
    }
}
