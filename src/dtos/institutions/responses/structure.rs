//! # `backend::dtos::institutions::responses::structure`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis e estruturas temporárias de agregação, convertendo coleções indexadas para vetores finais da resposta.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//! - `new`: Construtor helper para normalizar campos e preparar estruturas de resposta.
//!
//! ## Tipos
//! - `InstitutionStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `CompetitionSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `EventSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `TeamSubStructure`: DTO de saída serializado em JSON no contrato público da API.
//! - `TempInstitutionStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//! - `TempCompetitionSubStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//! - `TempEventSubStructure`: Estrutura intermediária de agregação antes da conversão para DTO serializável.
//!
use chrono::NaiveDate;
use indexmap::IndexMap;

use serde::Serialize;

use crate::shared::types::Scope;

// ======================== Response DTOs ========================
/// Estrutura completa de uma instituição.
///
/// Representa a árvore pública `instituição -> competições -> eventos -> times`
/// retornada pelo endpoint de estruturas de instituições.
#[derive(Serialize, Debug)]
pub struct InstitutionStructure {
    /// Identificador da instituição.
    pub id: i32,
    /// Nome oficial da instituição.
    pub name: String,
    /// Nome curto da instituição, quando cadastrado.
    pub short_name: Option<String>,
    /// Localização textual da instituição.
    pub location: String,
    /// Competições em que a instituição possui participação no recorte consultado.
    pub competitions: Vec<CompetitionSubStructure>,
}

/// Competição vinculada a uma instituição na estrutura pública.
///
/// Agrupa os eventos em que a instituição teve participação dentro da
/// competição.
#[derive(Serialize, Debug)]
pub struct CompetitionSubStructure {
    /// Identificador da competição.
    pub id: i32,
    /// Nome oficial da competição.
    pub name: String,
    /// URL pública da competição, quando cadastrada.
    pub website_url: Option<String>,
    /// Eventos da competição com participação da instituição.
    pub events: Vec<EventSubStructure>,
}

/// Evento de uma competição dentro da estrutura de instituição.
///
/// Contém os metadados do evento e os times da instituição que participaram
/// dele.
#[derive(Serialize, Debug)]
pub struct EventSubStructure {
    /// Identificador do evento.
    pub id: i32,
    /// Nome do evento.
    pub name: String,
    /// Data de realização do evento.
    pub date: NaiveDate,
    /// Nível competitivo do evento, quando informado.
    pub level: Option<u32>,
    /// Escopo competitivo do evento.
    pub scope: Scope,
    /// Times da instituição no evento.
    pub teams: Vec<TeamSubStructure>,
}

/// Time de uma instituição dentro de um evento.
///
/// Expõe a classificação e os totais de membros do time no contexto do evento
/// em que ele participou.
#[derive(Serialize, Debug)]
pub struct TeamSubStructure {
    /// Identificador do time.
    pub id: i32,
    /// Nome do time.
    pub name: String,
    /// Posição do time no evento.
    pub rank: u32,
    /// Total de integrantes do time.
    pub total_members: u32,
    /// Total de integrantes femininas do time.
    pub female_participants: u32,
}

// ======================== Intermediate structures ========================
// Used while aggregating institution -> competitions -> events -> teams
// before converting to the final serializable payload.
/// Estrutura temporária usada para agregar uma instituição.
///
/// Mantém competições indexadas por ID enquanto o service dobra linhas SQL
/// achatadas na árvore final de resposta.
#[derive(Debug)]
pub struct TempInstitutionStructure {
    /// Identificador da instituição.
    pub id: i32,
    /// Nome oficial da instituição.
    pub name: String,
    /// Nome curto da instituição, quando cadastrado.
    pub short_name: Option<String>,
    /// Localização textual da instituição.
    pub location: String,
    /// Competições intermediárias indexadas pelo ID da competição.
    pub competitions: IndexMap<i32, TempCompetitionSubStructure>,
}

/// Estrutura temporária usada para agregar uma competição da instituição.
///
/// Mantém eventos indexados por ID até a conversão para a forma serializável.
#[derive(Debug)]
pub struct TempCompetitionSubStructure {
    /// Identificador da competição.
    pub id: i32,
    /// Nome oficial da competição.
    pub name: String,
    /// URL pública da competição, quando cadastrada.
    pub website_url: Option<String>,
    /// Eventos intermediários indexados pelo ID do evento.
    pub events: IndexMap<i32, TempEventSubStructure>,
}

/// Estrutura temporária usada para agregar um evento da instituição.
///
/// Mantém times indexados por ID enquanto a resposta hierárquica é montada.
#[derive(Debug)]
pub struct TempEventSubStructure {
    /// Identificador do evento.
    pub id: i32,
    /// Nome do evento.
    pub name: String,
    /// Data de realização do evento.
    pub date: NaiveDate,
    /// Nível competitivo do evento, quando informado.
    pub level: Option<u32>,
    /// Escopo competitivo do evento.
    pub scope: Scope,
    /// Times intermediários indexados pelo ID do time.
    pub teams: IndexMap<i32, TeamSubStructure>,
}

// ======================== Conversion to final DTO ========================
impl From<TempInstitutionStructure> for InstitutionStructure {
    /// Converte uma instituição temporária na estrutura pública.
    ///
    /// Troca o mapa de competições indexadas pelo vetor serializável esperado
    /// no contrato JSON.
    fn from(value: TempInstitutionStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            short_name: value.short_name,
            location: value.location,
            competitions: {
                value
                    .competitions
                    .into_values()
                    .map(CompetitionSubStructure::from)
                    .collect()
            },
        }
    }
}

impl From<TempCompetitionSubStructure> for CompetitionSubStructure {
    /// Converte uma competição temporária da instituição na estrutura pública.
    ///
    /// Transforma eventos indexados em vetor serializável, preservando os
    /// metadados da competição.
    fn from(value: TempCompetitionSubStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            events: {
                value
                    .events
                    .into_values()
                    .map(EventSubStructure::from)
                    .collect()
            },
        }
    }
}

impl From<TempEventSubStructure> for EventSubStructure {
    /// Converte um evento temporário da instituição na estrutura pública.
    ///
    /// Transforma times indexados em vetor serializável, preservando escopo e
    /// nível já normalizados.
    fn from(value: TempEventSubStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            date: value.date,
            level: value.level,
            scope: value.scope,
            teams: { value.teams.into_values().collect() },
        }
    }
}

// ======================== Helper constructors ========================
impl TempInstitutionStructure {
    /// Cria um acumulador temporário para uma instituição.
    ///
    /// Mantém competições indexadas por ID durante a agregação das linhas do
    /// repositório.
    pub fn new(
        id: i32,
        name: String,
        short_name: Option<String>,
        location: String,
        competitions: IndexMap<i32, TempCompetitionSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            short_name,
            location,
            competitions,
        }
    }
}

impl TempCompetitionSubStructure {
    /// Cria um acumulador temporário para uma competição da instituição.
    ///
    /// Mantém eventos indexados por ID até a conversão para o payload público.
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        events: IndexMap<i32, TempEventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            events,
        }
    }
}

impl TempEventSubStructure {
    /// Cria um acumulador temporário para um evento da instituição.
    ///
    /// Normaliza o nível opcional para `u32` e mantém times indexados até a
    /// conversão final.
    pub fn new(
        id: i32,
        name: String,
        date: NaiveDate,
        level: Option<i32>,
        scope: Scope,
        teams: IndexMap<i32, TeamSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            date,
            level: level.map(|l| l as u32),
            scope,
            teams,
        }
    }
}

impl TeamSubStructure {
    /// Cria a estrutura pública de um time de instituição.
    ///
    /// Normaliza ranking e totais vindos do repositório para inteiros sem
    /// sinal usados no contrato serializado.
    pub fn new(id: i32, name: String, rank: i32, total_members: i32, female_members: i32) -> Self {
        Self {
            id,
            name,
            rank: rank as u32,
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
    fn institution_team_sub_structure_normalizes_rank_and_totals() {
        let team = TeamSubStructure::new(100, "Rio Coders".to_string(), 4, 3, 1);

        assert_eq!(team.id, 100);
        assert_eq!(team.name, "Rio Coders");
        assert_eq!(team.rank, 4);
        assert_eq!(team.total_members, 3);
        assert_eq!(team.female_participants, 1);
    }

    #[test]
    fn institution_structure_conversion_preserves_nested_order() {
        let mut teams = IndexMap::new();
        teams.insert(
            100,
            TeamSubStructure::new(100, "Rio Coders".to_string(), 2, 3, 1),
        );

        let mut events = IndexMap::new();
        events.insert(
            10,
            TempEventSubStructure::new(
                10,
                "Regional".to_string(),
                date(),
                Some(1),
                Scope::Regional,
                teams,
            ),
        );

        let mut competitions = IndexMap::new();
        competitions.insert(
            5,
            TempCompetitionSubStructure::new(
                5,
                "ICPC".to_string(),
                Some("https://icpc.org".to_string()),
                events,
            ),
        );

        let institution = InstitutionStructure::from(TempInstitutionStructure::new(
            1,
            "Universidade Federal do Rio de Janeiro".to_string(),
            Some("UFRJ".to_string()),
            "Rio de Janeiro".to_string(),
            competitions,
        ));

        assert_eq!(institution.id, 1);
        assert_eq!(institution.short_name.as_deref(), Some("UFRJ"));
        assert_eq!(institution.competitions.len(), 1);
        assert_eq!(institution.competitions[0].events.len(), 1);
        assert_eq!(institution.competitions[0].events[0].level, Some(1));
        assert_eq!(institution.competitions[0].events[0].teams[0].rank, 2);
    }
}
