//! # `backend::dtos::organizers::responses::structure`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `organizers`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis e estruturas temporárias de agregação, convertendo coleções indexadas para vetores finais da resposta.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//! - `new`: Construtor helper para normalizar campos e preparar estruturas de resposta.
//!
//! ## Tipos
//! - `OrganizerStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `CompetitionSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `EventSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `TempOrganizerStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//! - `TempCompetitionSubStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//!
use chrono::NaiveDate;
use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::{GenderCategory, LocationType};

// ======================== Response DTOs ========================
/// Estrutura completa de um organizador.
///
/// Representa a árvore pública `organizador -> competições -> eventos`
/// retornada pelo endpoint de estruturas de organizadores.
#[derive(Debug, Serialize)]
pub struct OrganizerStructure {
    /// Identificador do organizador.
    pub id: i32,
    /// Nome oficial do organizador.
    pub name: String,
    /// URL pública do organizador, quando cadastrada.
    pub website_url: Option<String>,
    /// Competições mantidas pelo organizador no recorte consultado.
    pub competitions: Vec<CompetitionSubStructure>,
}

/// Competição vinculada a um organizador.
///
/// Contém metadados da competição, anos disponíveis, tipos de localização e
/// eventos agregados para a resposta final.
#[derive(Debug, Serialize)]
pub struct CompetitionSubStructure {
    /// Identificador da competição.
    pub id: i32,
    /// Nome oficial da competição.
    pub name: String,
    /// URL pública da competição, quando cadastrada.
    pub website_url: Option<String>,
    /// Categoria de gênero atendida pela competição.
    pub gender_category: GenderCategory,
    /// Anos em que a competição possui eventos no recorte consultado.
    pub years: Vec<u32>,
    /// Tipos de localização presentes nos eventos da competição.
    pub location_types: Vec<LocationType>,
    /// Eventos associados à competição.
    pub events: Vec<EventSubStructure>,
}

/// Evento de uma competição organizada.
///
/// Expõe metadados, localização e totais consolidados do evento no contrato
/// público de organizadores.
#[derive(Debug, Serialize)]
pub struct EventSubStructure {
    /// Identificador do evento.
    pub id: i32,
    /// Nome do evento.
    pub name: String,
    /// Nível competitivo do evento, quando informado.
    pub level: Option<u32>,
    /// Data de realização do evento.
    pub date: NaiveDate,
    /// Localização textual derivada da árvore de localização.
    pub location: String,
    /// Total de instituições participantes no evento.
    pub total_institutions: u32,
    /// Total de times participantes no evento.
    pub total_teams: u32,
    /// Total de participantes no evento.
    pub total_participants: u32,
    /// Total de participantes femininas no evento.
    pub female_participants: u32,
    /// Tipos de localização associados ao evento.
    pub location_types: Vec<LocationType>,
}

// ======================== Intermediate structures ========================
// Used while aggregating organizer -> competitions -> events
// before converting to the final serializable payload.
/// Estrutura temporária usada para agregar um organizador.
///
/// Mantém competições indexadas por ID enquanto o service transforma linhas
/// achatadas do repositório na resposta hierárquica.
#[derive(Debug)]
pub struct TempOrganizerStructure {
    /// Identificador do organizador.
    pub id: i32,
    /// Nome oficial do organizador.
    pub name: String,
    /// URL pública do organizador, quando cadastrada.
    pub website_url: Option<String>,
    /// Competições intermediárias indexadas pelo ID da competição.
    pub competitions: IndexMap<i32, TempCompetitionSubStructure>,
}

/// Estrutura temporária usada para agregar uma competição do organizador.
///
/// Mantém eventos indexados por ID até a conversão para
/// [`CompetitionSubStructure`].
#[derive(Debug)]
pub struct TempCompetitionSubStructure {
    /// Identificador da competição.
    pub id: i32,
    /// Nome oficial da competição.
    pub name: String,
    /// URL pública da competição, quando cadastrada.
    pub website_url: Option<String>,
    /// Categoria de gênero atendida pela competição.
    pub gender_category: GenderCategory,
    /// Anos coletados para a competição antes da serialização.
    pub years: Vec<u32>,
    /// Tipos de localização coletados antes da ordenação final.
    pub location_types: Vec<LocationType>,
    /// Eventos intermediários indexados pelo ID do evento.
    pub events: IndexMap<i32, EventSubStructure>,
}

// ======================== Conversion to final DTO ========================
impl From<TempOrganizerStructure> for OrganizerStructure {
    /// Converte um organizador temporário na estrutura pública.
    ///
    /// Troca o mapa de competições indexadas pelo vetor serializável esperado
    /// no contrato JSON.
    fn from(value: TempOrganizerStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            competitions: value
                .competitions
                .into_values()
                .map(CompetitionSubStructure::from)
                .collect(),
        }
    }
}

impl From<TempCompetitionSubStructure> for CompetitionSubStructure {
    /// Converte uma competição temporária na estrutura pública.
    ///
    /// Ordena os tipos de localização e transforma eventos indexados em vetor
    /// serializável.
    fn from(value: TempCompetitionSubStructure) -> Self {
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            gender_category: value.gender_category,
            years: value.years,
            location_types,
            events: value.events.into_values().collect(),
        }
    }
}

// ======================== Helper constructors ========================
impl TempOrganizerStructure {
    /// Cria um acumulador temporário para um organizador.
    ///
    /// Preserva o mapa de competições para que o service possa agrupar linhas
    /// do repositório sem duplicar nós na árvore.
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        competitions: IndexMap<i32, TempCompetitionSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            competitions,
        }
    }
}

impl TempCompetitionSubStructure {
    /// Cria um acumulador temporário para uma competição do organizador.
    ///
    /// Converte anos vindos do banco como `i32` para `u32` e preserva eventos
    /// indexados até a resposta final.
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        gender_category: GenderCategory,
        years: Vec<i32>,
        location_types: Vec<LocationType>,
        events: IndexMap<i32, EventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            gender_category,
            years: years.into_iter().map(|y| y as u32).collect(),
            location_types,
            events,
        }
    }
}

impl EventSubStructure {
    /// Cria a estrutura pública de um evento organizado.
    ///
    /// Normaliza nível e totais para `u32`, ordenando os tipos de localização
    /// antes da serialização.
    pub fn new(
        id: i32,
        name: String,
        level: Option<i32>,
        date: NaiveDate,
        location: String,
        total_institutions: i32,
        total_teams: i32,
        total_participants: i32,
        female_participants: i32,
        location_types: Vec<LocationType>,
    ) -> Self {
        let mut location_types = location_types;
        location_types.sort();
        Self {
            id,
            name,
            level: level.map(|l| l as u32),
            date,
            location,
            total_institutions: total_institutions as u32,
            total_teams: total_teams as u32,
            total_participants: total_participants as u32,
            female_participants: female_participants as u32,
            location_types,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 11, 10).unwrap()
    }

    #[test]
    fn organizer_event_sub_structure_normalizes_totals_and_sorts_locations() {
        let event = EventSubStructure::new(
            100,
            "Final".to_string(),
            Some(2),
            date(),
            "Brazil, Recife".to_string(),
            20,
            40,
            120,
            32,
            vec![LocationType::City, LocationType::Country],
        );

        assert_eq!(event.level, Some(2));
        assert_eq!(event.total_institutions, 20);
        assert_eq!(event.total_teams, 40);
        assert_eq!(event.total_participants, 120);
        assert_eq!(event.female_participants, 32);
        assert_eq!(
            event.location_types,
            vec![LocationType::Country, LocationType::City]
        );
    }

    #[test]
    fn organizer_structure_conversion_converts_competition_children() {
        let mut events = IndexMap::new();
        events.insert(
            100,
            EventSubStructure::new(
                100,
                "Regional".to_string(),
                Some(1),
                date(),
                "Brazil, Salvador".to_string(),
                12,
                24,
                72,
                18,
                vec![LocationType::City, LocationType::Country],
            ),
        );

        let mut competitions = IndexMap::new();
        competitions.insert(
            10,
            TempCompetitionSubStructure::new(
                10,
                "ICPC".to_string(),
                Some("https://icpc.org".to_string()),
                GenderCategory::Open,
                vec![2023, 2024],
                vec![LocationType::City, LocationType::Country],
                events,
            ),
        );

        let organizer = OrganizerStructure::from(TempOrganizerStructure::new(
            1,
            "ICPC Foundation".to_string(),
            Some("https://icpc.global".to_string()),
            competitions,
        ));

        assert_eq!(organizer.id, 1);
        assert_eq!(organizer.competitions.len(), 1);
        assert_eq!(organizer.competitions[0].years, vec![2023, 2024]);
        assert_eq!(
            organizer.competitions[0].location_types,
            vec![LocationType::Country, LocationType::City]
        );
        assert_eq!(organizer.competitions[0].events[0].name, "Regional");
    }
}
