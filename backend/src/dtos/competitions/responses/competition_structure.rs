//! # `backend::dtos::competitions::responses::competition_structure`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis e estruturas temporárias de agregação, convertendo coleções indexadas para vetores finais da resposta.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//! - `new`: Construtor helper para normalizar campos e preparar estruturas de resposta.
//!
//! ## Tipos
//! - `CompetitionStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `EventSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `TeamSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `TempCompetitionStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//! - `TempEventSubStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//!
use chrono::NaiveDate;
use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::{GenderCategory, LocationType};

/// Estrutura completa de uma competição.
///
/// Representa o contrato público do endpoint de estruturas de competições,
/// incluindo metadados da competição, anos, tipos de localização e eventos.
#[derive(Debug, Serialize)]
pub struct CompetitionStructure {
    /// Identificador da competição.
    pub id: i32,
    /// Nome oficial da competição.
    pub name: String,
    /// URL pública da competição, quando cadastrada.
    pub website_url: Option<String>,
    /// Categoria de gênero atendida pela competição.
    pub gender_category: GenderCategory,
    /// Anos em que a competição possui eventos ou resultados no recorte consultado.
    pub years: Vec<u32>,
    /// Tipos de localização presentes nos eventos da competição.
    pub location_types: Vec<LocationType>,
    /// Eventos associados à competição.
    pub events: Vec<EventSubStructure>,
}

/// Evento de uma competição dentro da estrutura pública.
///
/// Agrupa os metadados do evento e a lista de times que participaram dele no
/// contexto da competição consultada.
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
    /// Tipos de localização associados ao evento.
    pub location_types: Vec<LocationType>,
    /// Times participantes e seus resultados no evento.
    pub teams: Vec<TeamSubStructure>,
}

/// Resultado de um time em um evento de competição.
///
/// Contém a classificação do time e dados da instituição usados pela tela de
/// detalhe de competição.
#[derive(Debug, Serialize)]
pub struct TeamSubStructure {
    /// Identificador do time.
    pub id: i32,
    /// Nome do time.
    pub name: String,
    /// Posição do time no evento.
    pub rank: u32,
    /// Nome da instituição vinculada ao time.
    pub institution_name: String,
    /// Nome curto da instituição, quando cadastrado.
    pub institution_short_name: Option<String>,
    /// Localização textual da instituição.
    pub institution_location: String,
    /// Total de integrantes do time.
    pub total_members: u32,
    /// Total de integrantes femininas do time.
    pub female_participants: u32,
}

/// Estrutura temporária usada para agregar uma competição.
///
/// Mantém os eventos indexados enquanto o service transforma linhas SQL
/// achatadas na árvore final serializável.
#[derive(Debug)]
pub struct TempCompetitionStructure {
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
    pub events: IndexMap<i32, TempEventSubStructure>,
}

/// Estrutura temporária usada para agregar um evento de competição.
///
/// Mantém os times indexados por ID até que a conversão para
/// [`EventSubStructure`] produza o vetor final da resposta.
#[derive(Debug)]
pub struct TempEventSubStructure {
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
    /// Tipos de localização coletados antes da ordenação final.
    pub location_types: Vec<LocationType>,
    /// Times intermediários indexados pelo ID do time.
    pub teams: IndexMap<i32, TeamSubStructure>,
}

impl From<TempCompetitionStructure> for CompetitionStructure {
    /// Converte uma competição temporária na estrutura pública.
    ///
    /// Ordena os tipos de localização e transforma eventos indexados em vetor
    /// serializável.
    fn from(value: TempCompetitionStructure) -> Self {
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            gender_category: value.gender_category,
            years: value.years,
            location_types,
            events: value
                .events
                .into_values()
                .map(EventSubStructure::from)
                .collect(),
        }
    }
}

impl From<TempEventSubStructure> for EventSubStructure {
    /// Converte um evento temporário na estrutura pública.
    ///
    /// Ordena os tipos de localização e transforma times indexados em vetor
    /// serializável.
    fn from(value: TempEventSubStructure) -> Self {
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            id: value.id,
            name: value.name,
            level: value.level,
            date: value.date,
            location: value.location,
            location_types,
            teams: value.teams.into_values().collect(),
        }
    }
}

impl TempCompetitionStructure {
    /// Cria uma competição temporária durante a agregação de linhas SQL.
    ///
    /// Converte anos vindos do banco como `i32` para `u32` e preserva os
    /// eventos indexados até a conversão final.
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        gender_category: GenderCategory,
        years: Vec<i32>,
        location_types: Vec<LocationType>,
        events: IndexMap<i32, TempEventSubStructure>,
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

impl TempEventSubStructure {
    /// Cria um evento temporário durante a agregação de linhas SQL.
    ///
    /// Normaliza o nível opcional para `u32` e preserva o mapa de times até a
    /// conversão final.
    pub fn new(
        id: i32,
        name: String,
        level: Option<i32>,
        date: NaiveDate,
        location: String,
        location_types: Vec<LocationType>,
        teams: IndexMap<i32, TeamSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            level: level.map(|l| l as u32),
            date,
            location,
            location_types,
            teams,
        }
    }
}

impl TeamSubStructure {
    /// Cria o resultado público de um time em evento de competição.
    ///
    /// Normaliza ranking e totais vindos do repositório para inteiros sem
    /// sinal usados no contrato serializado.
    pub fn new(
        id: i32,
        name: String,
        rank: i32,
        institution_name: String,
        institution_short_name: Option<String>,
        institution_location: String,
        total_members: i32,
        female_members: i32,
    ) -> Self {
        Self {
            id,
            name,
            rank: rank as u32,
            institution_name,
            institution_short_name,
            institution_location,
            total_members: total_members as u32,
            female_participants: female_members as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 10, 1).unwrap()
    }

    #[test]
    fn team_sub_structure_normalizes_rank_and_totals() {
        let team = TeamSubStructure::new(
            1000,
            "Bit Masters".to_string(),
            2,
            "USP".to_string(),
            Some("USP".to_string()),
            "Sao Paulo".to_string(),
            3,
            1,
        );

        assert_eq!(team.id, 1000);
        assert_eq!(team.rank, 2);
        assert_eq!(team.total_members, 3);
        assert_eq!(team.female_participants, 1);
        assert_eq!(team.institution_short_name.as_deref(), Some("USP"));
    }

    #[test]
    fn competition_structure_conversion_sorts_location_types_and_keeps_children() {
        let mut teams = IndexMap::new();
        teams.insert(
            1000,
            TeamSubStructure::new(
                1000,
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
            50,
            TempEventSubStructure::new(
                50,
                "Regional".to_string(),
                Some(2),
                date(),
                "Brazil, Sao Paulo".to_string(),
                vec![LocationType::City, LocationType::Country],
                teams,
            ),
        );

        let competition = CompetitionStructure::from(TempCompetitionStructure::new(
            10,
            "ICPC".to_string(),
            Some("https://icpc.org".to_string()),
            GenderCategory::Open,
            vec![2023, 2024],
            vec![LocationType::City, LocationType::Country],
            events,
        ));

        assert_eq!(competition.id, 10);
        assert_eq!(competition.years, vec![2023, 2024]);
        assert_eq!(
            competition.location_types,
            vec![LocationType::Country, LocationType::City]
        );
        assert_eq!(competition.events.len(), 1);
        assert_eq!(competition.events[0].level, Some(2));
        assert_eq!(
            competition.events[0].location_types,
            vec![LocationType::Country, LocationType::City]
        );
        assert_eq!(competition.events[0].teams[0].name, "Bit Masters");
    }
}
