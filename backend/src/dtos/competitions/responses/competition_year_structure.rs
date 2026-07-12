//! # `backend::dtos::competitions::responses::competition_year_structure`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis e estruturas temporárias de agregação, convertendo coleções indexadas para vetores finais da resposta.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//! - `update`: Função de transformação usada na montagem de DTOs de request/response.
//!
//! ## Tipos
//! - `CompetitionYearStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `TempCompetitionYearStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//!
use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::LocationType;

use super::EventSubStructure;

/// Estrutura anual de eventos de uma competição.
///
/// Este payload é a forma serializável retornada ao cliente após a agregação
/// dos eventos do ano e dos tipos de localização relacionados.
#[derive(Debug, Serialize)]
pub struct CompetitionYearStructure {
    /// Tipos de localização presentes nos eventos retornados.
    pub location_types: Vec<LocationType>,
    /// Eventos realizados pela competição no ano consultado.
    pub events: Vec<EventSubStructure>,
}

/// Estrutura temporária usada durante a agregação da estrutura anual.
///
/// Mantém eventos indexados por ID para evitar duplicidade enquanto as linhas
/// achatadas do repositório são dobradas no service.
#[derive(Default, Debug)]
pub struct TempCompetitionYearStructure {
    /// Tipos de localização coletados antes da ordenação final.
    pub location_types: Vec<LocationType>,
    /// Eventos intermediários indexados pelo ID do evento.
    pub events: IndexMap<i32, super::TempEventSubStructure>,
}

impl From<TempCompetitionYearStructure> for CompetitionYearStructure {
    /// Converte a estrutura anual temporária na resposta serializável.
    ///
    /// Ordena os tipos de localização e troca o mapa de eventos indexados pelo
    /// vetor final esperado no JSON.
    fn from(value: TempCompetitionYearStructure) -> Self {
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            location_types,
            events: value
                .events
                .into_values()
                .map(EventSubStructure::from)
                .collect(),
        }
    }
}

impl TempCompetitionYearStructure {
    /// Atualiza os tipos de localização coletados para a competição.
    ///
    /// # Parâmetros
    /// - `location_types`: tipos de localização derivados das linhas do
    ///   repositório.
    pub fn update(&mut self, location_types: Vec<LocationType>) {
        self.location_types = location_types;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dtos::competitions::responses::{TeamSubStructure, TempEventSubStructure},
        shared::types::LocationType,
    };
    use chrono::NaiveDate;

    #[test]
    fn competition_year_structure_sorts_locations_and_converts_events() {
        let mut teams = IndexMap::new();
        teams.insert(
            10,
            TeamSubStructure::new(
                10,
                "Bit Masters".to_string(),
                1,
                "USP".to_string(),
                None,
                "Sao Paulo".to_string(),
                3,
                1,
            ),
        );

        let mut events = IndexMap::new();
        events.insert(
            1,
            TempEventSubStructure::new(
                1,
                "Regional".to_string(),
                Some(1),
                NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(),
                "Brazil, Sao Paulo".to_string(),
                vec![LocationType::City, LocationType::Country],
                teams,
            ),
        );

        let mut temporary = TempCompetitionYearStructure::default();
        temporary.update(vec![LocationType::City, LocationType::Country]);
        temporary.events = events;

        let structure = CompetitionYearStructure::from(temporary);

        assert_eq!(
            structure.location_types,
            vec![LocationType::Country, LocationType::City]
        );
        assert_eq!(structure.events.len(), 1);
        assert_eq!(structure.events[0].teams.len(), 1);
    }
}
