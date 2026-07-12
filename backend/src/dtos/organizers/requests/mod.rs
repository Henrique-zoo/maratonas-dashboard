//! # `backend::dtos::organizers::requests`
//!
//! ## Responsabilidade
//! Define DTOs de entrada do domínio `organizers`.
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

/// Parâmetros de query para recuperar estruturas de organizadores.
///
/// Modela a lista de organizadores solicitados pelo cliente, mantendo a
/// desserialização CSV fora da lógica de serviço.
#[derive(Deserialize)]
pub struct StructuresQuery {
    /// Lista opcional de organizadores que devem compor a resposta.
    #[serde(default)]
    pub organizer_ids: CsvOptVec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structures_query_accepts_missing_organizer_filter() {
        let query: StructuresQuery = serde_urlencoded::from_str("").unwrap();

        assert_eq!(query.organizer_ids.into_inner(), None);
    }

    #[test]
    fn structures_query_parses_organizer_ids_csv() {
        let query: StructuresQuery = serde_urlencoded::from_str("organizer_ids=1,2").unwrap();

        assert_eq!(query.organizer_ids.into_inner(), Some(vec![1, 2]));
    }
}
