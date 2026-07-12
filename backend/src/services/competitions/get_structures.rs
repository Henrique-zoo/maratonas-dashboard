//! # `backend::services::competitions::get_structures`
//!
//! ## Responsabilidade
//! Implementa casos de uso do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Valida entrada, consulta o repositório, agrega linhas achatadas com `IndexMap` e converte para estruturas hierárquicas de resposta.
//!
//! ## Funções
//! - `get_structures`: Caso de uso de domínio que valida parâmetros e orquestra consulta/transformação de dados.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use indexmap::IndexMap;

use crate::{
    dtos::competitions::responses::{
        CompetitionStructure, TeamSubStructure, TempCompetitionStructure, TempEventSubStructure,
    },
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
};

/// Retorna a estrutura hierárquica das competições solicitadas.
///
/// A função valida se ao menos um identificador de competição foi informado,
/// consulta o repositório e reorganiza as linhas achatadas em uma árvore no
/// formato `competicao -> eventos -> times`.
///
/// # Parâmetros
/// - `repo`: contrato de acesso a dados de competições.
/// - `competition_ids`: lista de IDs de competição a serem buscadas.
///
/// # Retorno
/// - `Ok(Vec<CompetitionStructure>)` com as estruturas agregadas para cada
///   competição encontrada.
///
/// # Erros
/// - Retorna `AppError::BadRequest` quando `competition_ids` é `None`.
/// - Propaga erros do repositório.
///
/// # Exemplos
/// ```ignore
/// use backend::services;
/// use backend::errors::AppResult;
/// use backend::repositories::CompetitionRepository;
///
/// async fn run(repo: &dyn CompetitionRepository) -> AppResult<()> {
///     let structures = services::competitions::get_structures(repo, Some(vec![10, 11])).await?;
///     assert!(!structures.is_empty());
///     Ok(())
/// }
/// ```
pub async fn get_structures(
    repo: &dyn CompetitionRepository,
    competition_ids: Option<Vec<i32>>,
) -> AppResult<Vec<CompetitionStructure>> {
    let competition_ids = competition_ids.ok_or_else(|| {
        AppError::BadRequest("You need to choose at least one competition.".to_string())
    })?;

    let structures = repo
        .find_structures_by_ids(competition_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut competitions, row| {
            competitions
                .entry(row.competition_id)
                .or_insert_with(|| {
                    TempCompetitionStructure::new(
                        row.competition_id,
                        row.competition_name,
                        row.competition_website_url,
                        row.competition_gender_category,
                        row.competition_years,
                        row.competition_location_types,
                        IndexMap::new(),
                    )
                })
                .events
                .entry(row.event_id)
                .or_insert_with(|| {
                    TempEventSubStructure::new(
                        row.event_id,
                        row.event_name,
                        row.event_level,
                        row.event_date,
                        row.event_location,
                        row.event_location_types,
                        IndexMap::new(),
                    )
                })
                .teams
                .insert(
                    row.team_id,
                    TeamSubStructure::new(
                        row.team_id,
                        row.team_name,
                        row.team_rank,
                        row.institution_name,
                        row.institution_short_name,
                        row.institution_location,
                        row.team_total_members,
                        row.team_female_members,
                    ),
                );

            competitions
        })
        .into_values()
        .map(CompetitionStructure::from)
        .collect();

    Ok(structures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        errors::AppError,
        repositories::{MockCompetitionRepository, types::competitions::CompetitionStructureRow},
        shared::types::{GenderCategory, LocationType},
    };

    fn row() -> CompetitionStructureRow {
        CompetitionStructureRow {
            competition_id: 10,
            competition_name: "ICPC".to_string(),
            competition_website_url: Some("https://icpc.org".to_string()),
            competition_gender_category: GenderCategory::Open,
            competition_years: vec![2023, 2024],
            competition_location_types: vec![LocationType::Country, LocationType::City],
            event_id: 100,
            event_name: "Regional".to_string(),
            event_level: Some(1),
            event_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
            event_location: "Brazil, Sao Paulo".to_string(),
            event_location_types: vec![LocationType::Country, LocationType::City],
            institution_name: "USP".to_string(),
            institution_short_name: Some("USP".to_string()),
            institution_location: "Sao Paulo".to_string(),
            team_id: 1000,
            team_name: "Bit Masters".to_string(),
            team_rank: 1,
            team_total_members: 3,
            team_female_members: 1,
        }
    }

    #[tokio::test]
    async fn get_structures_requires_competition_ids() {
        let repo = MockCompetitionRepository::new();

        let result = get_structures(&repo, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to choose at least one competition."
        );
    }

    #[tokio::test]
    async fn get_structures_groups_teams_under_event() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![10]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    CompetitionStructureRow {
                        team_id: 1001,
                        team_name: "Stack Smash".to_string(),
                        team_rank: 2,
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![10])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].events.len(), 1);
        assert_eq!(result[0].events[0].teams.len(), 2);
    }

    #[tokio::test]
    async fn get_structures_supports_multiple_competitions() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![10, 11]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    CompetitionStructureRow {
                        competition_id: 11,
                        competition_name: "OBI".to_string(),
                        event_id: 200,
                        event_name: "Nacional".to_string(),
                        team_id: 3000,
                        team_name: "OBI Team".to_string(),
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![10, 11])).await.unwrap();

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn get_structures_returns_empty_when_repository_returns_empty() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![10]))
            .returning(|_| Ok(vec![]));

        let result = get_structures(&repo, Some(vec![10])).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_structures_propagates_repository_error() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![10]))
            .returning(|_| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_structures(&repo, Some(vec![10])).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
