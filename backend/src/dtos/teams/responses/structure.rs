//! # `backend::dtos::teams::responses::structure`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `teams`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis e estruturas temporárias de agregação, convertendo coleções indexadas para vetores finais da resposta.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//! - `new`: Construtor helper para normalizar campos e preparar estruturas de resposta.
//!
//! ## Tipos
//! - `TeamStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `CompetitionSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `EventSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `TempTeamStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//! - `TempCompetitionSubStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//!
use chrono::NaiveDate;
use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::{GenderCategory, Scope};

// ======================== Response DTOs ========================
/// Estrutura completa de um time.
///
/// Representa a árvore pública `time -> competições -> eventos` retornada pelo
/// endpoint de estruturas de times.
#[derive(Debug, Serialize)]
pub struct TeamStructure {
    /// Identificador do time.
    pub id: i32,
    /// Nome do time.
    pub name: String,
    /// Competições em que o time participou no recorte consultado.
    pub competitions: Vec<CompetitionSubStructure>,
}

/// Competição vinculada a um time.
///
/// Contém metadados da competição, anos de participação, totais do time e
/// eventos em que ele competiu.
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
    /// Anos em que o time participou da competição.
    pub years: Vec<u32>,
    /// Total de integrantes do time no recorte da competição.
    pub total_members: u32,
    /// Total de integrantes femininas do time no recorte da competição.
    pub female_participants: u32,
    /// Eventos da competição disputados pelo time.
    pub events: Vec<EventSubStructure>,
}

/// Evento disputado por um time.
///
/// Expõe metadados do evento e a classificação do time nele.
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
    /// Escopo competitivo do evento.
    pub scope: Scope,
    /// Posição do time no evento.
    pub team_event_rank: u32,
}

// ======================== Intermediate structures ========================
// Used while aggregating teams -> competitions -> events
// before converting to the final serializable payload.
/// Estrutura temporária usada para agregar um time.
///
/// Mantém competições indexadas por ID enquanto o service transforma linhas
/// achatadas do repositório no payload final.
#[derive(Debug)]
pub struct TempTeamStructure {
    /// Identificador do time.
    pub id: i32,
    /// Nome do time.
    pub name: String,
    /// Competições intermediárias indexadas pelo ID da competição.
    pub competitions: IndexMap<i32, TempCompetitionSubStructure>,
}

/// Estrutura temporária usada para agregar uma competição do time.
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
    /// Total de integrantes do time no recorte da competição.
    pub total_members: u32,
    /// Total de integrantes femininas do time no recorte da competição.
    pub female_participants: u32,
    /// Eventos intermediários indexados pelo ID do evento.
    pub events: IndexMap<i32, EventSubStructure>,
}

impl From<TempTeamStructure> for TeamStructure {
    /// Converte um time temporário na estrutura pública.
    ///
    /// Troca o mapa de competições indexadas pelo vetor serializável esperado
    /// no contrato JSON.
    fn from(value: TempTeamStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            competitions: value
                .competitions
                .into_values()
                .map(CompetitionSubStructure::from)
                .collect(),
        }
    }
}

impl From<TempCompetitionSubStructure> for CompetitionSubStructure {
    /// Converte uma competição temporária do time na estrutura pública.
    ///
    /// Preserva métricas normalizadas e transforma eventos indexados em vetor
    /// serializável.
    fn from(value: TempCompetitionSubStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            gender_category: value.gender_category,
            years: value.years,
            total_members: value.total_members,
            female_participants: value.female_participants,
            events: value.events.into_values().collect(),
        }
    }
}

impl TempTeamStructure {
    /// Cria um acumulador temporário para um time.
    ///
    /// Mantém competições indexadas por ID durante a agregação das linhas do
    /// repositório.
    pub fn new(
        id: i32,
        name: String,
        competitions: IndexMap<i32, TempCompetitionSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            competitions,
        }
    }
}

impl TempCompetitionSubStructure {
    /// Cria um acumulador temporário para uma competição do time.
    ///
    /// Converte anos e totais vindos do banco para `u32` e preserva os eventos
    /// indexados até a conversão final.
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        gender_category: GenderCategory,
        years: Vec<i32>,
        total_members: i32,
        female_members: i32,
        events: IndexMap<i32, EventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            gender_category,
            years: years.into_iter().map(|y| y as u32).collect(),
            total_members: total_members as u32,
            female_participants: female_members as u32,
            events,
        }
    }
}

impl EventSubStructure {
    /// Cria a estrutura pública de um evento disputado pelo time.
    ///
    /// Normaliza nível e ranking vindos do repositório para inteiros sem sinal
    /// usados no contrato serializado.
    pub fn new(
        id: i32,
        name: String,
        level: Option<i32>,
        date: NaiveDate,
        location: String,
        scope: Scope,
        team_event_rank: i32,
    ) -> Self {
        Self {
            id,
            name,
            level: level.map(|l| l as u32),
            date,
            location,
            scope,
            team_event_rank: team_event_rank as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 9, 15).unwrap()
    }

    #[test]
    fn team_event_sub_structure_normalizes_level_and_rank() {
        let event = EventSubStructure::new(
            10,
            "Regional".to_string(),
            Some(3),
            date(),
            "Brazil, Porto Alegre".to_string(),
            Scope::Regional,
            4,
        );

        assert_eq!(event.id, 10);
        assert_eq!(event.level, Some(3));
        assert_eq!(event.team_event_rank, 4);
    }

    #[test]
    fn team_structure_conversion_preserves_competition_totals() {
        let mut events = IndexMap::new();
        events.insert(
            10,
            EventSubStructure::new(
                10,
                "Regional".to_string(),
                Some(1),
                date(),
                "Brazil, Porto Alegre".to_string(),
                Scope::Regional,
                2,
            ),
        );

        let mut competitions = IndexMap::new();
        competitions.insert(
            5,
            TempCompetitionSubStructure::new(
                5,
                "ICPC".to_string(),
                Some("https://icpc.org".to_string()),
                GenderCategory::Open,
                vec![2023, 2024],
                3,
                1,
                events,
            ),
        );

        let team = TeamStructure::from(TempTeamStructure::new(
            1000,
            "Bit Masters".to_string(),
            competitions,
        ));

        assert_eq!(team.id, 1000);
        assert_eq!(team.competitions.len(), 1);
        assert_eq!(team.competitions[0].years, vec![2023, 2024]);
        assert_eq!(team.competitions[0].total_members, 3);
        assert_eq!(team.competitions[0].female_participants, 1);
        assert_eq!(team.competitions[0].events[0].team_event_rank, 2);
    }
}
