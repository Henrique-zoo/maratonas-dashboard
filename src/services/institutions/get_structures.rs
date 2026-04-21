use indexmap::IndexMap;

use crate::{
    dtos::institutions::output::{
        InstitutionStructure, TeamSubStructure, TempCompetitionSubStructure, TempEventSubStructure,
        TempInstitutionStructure,
    },
    errors::{AppError, AppResult},
    repositories::InstitutionRepository,
};

pub async fn get_structures(
    repo: &dyn InstitutionRepository,
    institution_ids: Option<Vec<i32>>,
) -> AppResult<Vec<InstitutionStructure>> {
    let institution_ids = institution_ids.ok_or_else(|| {
        AppError::BadRequest("You need to choose at least one institution.".to_string())
    })?;

    let structures = repo
        .find_structures_by_ids(institution_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut institutions, row| {
            institutions
                .entry(row.institution_id)
                .or_insert_with(|| {
                    TempInstitutionStructure::new(
                        row.institution_id,
                        row.institution_name,
                        row.institution_short_name,
                        row.institution_location,
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
                        IndexMap::new(),
                    )
                })
                .events
                .entry(row.event_id)
                .or_insert_with(|| {
                    TempEventSubStructure::new(
                        row.event_id,
                        row.event_name,
                        row.event_date,
                        row.event_level,
                        row.event_scope,
                        IndexMap::new(),
                    )
                })
                .teams
                .insert(
                    row.team_id,
                    TeamSubStructure::new(
                        row.team_id,
                        row.team_name,
                        row.team_event_rank,
                        row.team_total_members,
                        row.team_female_members,
                    ),
                );

            institutions
        })
        .into_values()
        .map(InstitutionStructure::from)
        .collect();

    Ok(structures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        repositories::{MockInstitutionRepository, types::institutions::InstitutionStructureRow},
        shared::types::Scope,
    };

    fn row() -> InstitutionStructureRow {
        InstitutionStructureRow {
            institution_id: 5,
            institution_name: "UFRJ".to_string(),
            institution_short_name: Some("UFRJ".to_string()),
            institution_location: "Rio de Janeiro".to_string(),
            competition_id: 10,
            competition_name: "ICPC".to_string(),
            competition_website_url: Some("https://icpc.org".to_string()),
            event_id: 100,
            event_name: "Regional".to_string(),
            event_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
            event_level: Some(1),
            event_scope: Scope::Regional,
            team_id: 1000,
            team_name: "Rio Coders".to_string(),
            team_event_rank: 2,
            team_total_members: 3,
            team_female_members: 1,
        }
    }

    #[tokio::test]
    async fn get_structures_requires_ids() {
        let repo = MockInstitutionRepository::new();

        let result = get_structures(&repo, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to choose at least one institution."
        );
    }

    #[tokio::test]
    async fn get_structures_groups_teams_under_event() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![5]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    InstitutionStructureRow {
                        team_id: 1001,
                        team_name: "Rio Bits".to_string(),
                        team_event_rank: 4,
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![5])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].competitions.len(), 1);
        assert_eq!(result[0].competitions[0].events.len(), 1);
        assert_eq!(result[0].competitions[0].events[0].teams.len(), 2);
    }

    #[tokio::test]
    async fn get_structures_supports_multiple_institutions() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![5, 6]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    InstitutionStructureRow {
                        institution_id: 6,
                        institution_name: "UFPE".to_string(),
                        institution_short_name: Some("UFPE".to_string()),
                        institution_location: "Recife".to_string(),
                        team_id: 2000,
                        team_name: "Recife Coders".to_string(),
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![5, 6])).await.unwrap();

        assert_eq!(result.len(), 2);
    }
}
