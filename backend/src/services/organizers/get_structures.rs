//! # `backend::services::organizers::get_structures`
//!
//! ## Responsabilidade
//! Implementa casos de uso do domínio `organizers`.
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
    dtos::organizers::responses::{
        EventSubStructure, OrganizerStructure, TempCompetitionSubStructure, TempOrganizerStructure,
    },
    errors::{AppError, AppResult},
    repositories::OrganizerRepository,
};

/// Retorna estruturas completas dos organizadores informados.
///
/// Reorganiza o resultado de consulta em uma árvore
/// `organizador -> competicoes -> eventos`.
///
/// # Parâmetros
/// - `repo`: contrato de acesso a dados de organizadores.
/// - `organizer_ids`: IDs dos organizadores alvo.
///
/// # Retorno
/// - `Ok(Vec<OrganizerStructure>)` com competições e eventos por organizador.
///
/// # Erros
/// - Retorna `AppError::BadRequest` quando `organizer_ids` é `None`.
/// - Propaga erros do repositório.
///
/// # Exemplos
/// ```ignore
/// use backend::services;
/// use backend::errors::AppResult;
/// use backend::repositories::OrganizerRepository;
///
/// async fn run(repo: &dyn OrganizerRepository) -> AppResult<()> {
///     let structures = services::organizers::get_structures(repo, Some(vec![1, 2])).await?;
///     println!("Organizadores retornados: {}", structures.len());
///     Ok(())
/// }
/// ```
pub async fn get_structures(
    repo: &dyn OrganizerRepository,
    organizer_ids: Option<Vec<i32>>,
) -> AppResult<Vec<OrganizerStructure>> {
    let organizer_ids = organizer_ids.ok_or_else(|| {
        AppError::BadRequest("You need to choose at least one organizer.".to_string())
    })?;

    let structures = repo
        .find_structures_by_ids(organizer_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut organizers, row| {
            organizers
                .entry(row.organizer_id)
                .or_insert_with(|| {
                    TempOrganizerStructure::new(
                        row.organizer_id,
                        row.organizer_name,
                        row.organizer_website_url,
                        IndexMap::new(),
                    )
                })
                .competitions
                .entry(row.competition_id)
                .or_insert_with(|| {
                    TempCompetitionSubStructure::new(
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
                .insert(
                    row.event_id,
                    EventSubStructure::new(
                        row.event_id,
                        row.event_name,
                        row.event_level,
                        row.event_date,
                        row.event_location,
                        row.event_total_institutions,
                        row.event_total_teams,
                        row.event_total_participants,
                        row.event_female_participants,
                        row.event_location_types,
                    ),
                );

            organizers
        })
        .into_values()
        .map(OrganizerStructure::from)
        .collect();

    Ok(structures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        errors::AppError,
        repositories::{MockOrganizerRepository, types::organizers::OrganizerStructureRow},
        shared::types::{GenderCategory, LocationType},
    };

    fn row() -> OrganizerStructureRow {
        OrganizerStructureRow {
            organizer_id: 1,
            organizer_name: "ICPC Foundation".to_string(),
            organizer_website_url: Some("https://icpc.org".to_string()),
            competition_id: 10,
            competition_name: "ICPC".to_string(),
            competition_website_url: Some("https://icpc.org/2024".to_string()),
            competition_gender_category: GenderCategory::Open,
            competition_years: vec![2023, 2024],
            competition_location_types: vec![LocationType::Country, LocationType::City],
            event_id: 100,
            event_name: "Regional".to_string(),
            event_level: Some(1),
            event_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
            event_location: "Brazil, Salvador".to_string(),
            event_total_institutions: 30,
            event_total_teams: 60,
            event_total_participants: 180,
            event_female_participants: 45,
            event_location_types: vec![LocationType::Country, LocationType::City],
        }
    }

    #[tokio::test]
    async fn get_structures_requires_organizer_ids() {
        let repo = MockOrganizerRepository::new();

        let result = get_structures(&repo, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to choose at least one organizer."
        );
    }

    #[tokio::test]
    async fn get_structures_groups_events_under_competition() {
        let mut repo = MockOrganizerRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    OrganizerStructureRow {
                        event_id: 101,
                        event_name: "Final".to_string(),
                        event_total_teams: 40,
                        event_total_participants: 120,
                        event_female_participants: 36,
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].competitions.len(), 1);
        assert_eq!(result[0].competitions[0].events.len(), 2);
        assert_eq!(
            result[0].competitions[0].events[0].location,
            "Brazil, Salvador"
        );
    }

    #[tokio::test]
    async fn get_structures_supports_multiple_organizers() {
        let mut repo = MockOrganizerRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1, 2]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    OrganizerStructureRow {
                        organizer_id: 2,
                        organizer_name: "OBI Org".to_string(),
                        competition_id: 20,
                        competition_name: "OBI".to_string(),
                        event_id: 200,
                        event_name: "Fase Final".to_string(),
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1, 2])).await.unwrap();

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn get_structures_returns_empty_when_repository_returns_empty() {
        let mut repo = MockOrganizerRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| Ok(vec![]));

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_structures_propagates_repository_error() {
        let mut repo = MockOrganizerRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_structures(&repo, Some(vec![1])).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
