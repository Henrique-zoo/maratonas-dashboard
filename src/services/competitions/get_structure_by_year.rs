use indexmap::IndexMap;

use crate::{
    dtos::competitions::output::{
        CompetitionYearStructure, TeamSubStructure, TempCompetitionYearStructure,
        TempEventSubStructure,
    },
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
};

pub async fn get_structure_by_year(
    repo: &dyn CompetitionRepository,
    competition_id: i32,
    year: Option<i32>,
) -> AppResult<CompetitionYearStructure> {
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify the year.".to_string()))?;

    let results = repo
        .find_structure_by_year(competition_id, year)
        .await?
        .into_iter()
        .fold(
            TempCompetitionYearStructure::default(),
            |mut competition, row| {
                if competition.events.is_empty() {
                    competition.update(row.competition_location_types);
                }

                competition
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

                competition
            },
        );

    Ok(CompetitionYearStructure::from(results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        repositories::{
            MockCompetitionRepository, types::competitions::CompetitionYearStructureRow,
        },
        shared::types::LocationType,
    };

    fn row() -> CompetitionYearStructureRow {
        CompetitionYearStructureRow {
            competition_location_types: vec![LocationType::Country, LocationType::City],
            event_id: 101,
            event_name: "Final".to_string(),
            event_level: Some(2),
            event_date: NaiveDate::from_ymd_opt(2024, 11, 10).unwrap(),
            event_location: "Brazil, Recife".to_string(),
            event_location_types: vec![LocationType::Country, LocationType::City],
            institution_name: "UFPE".to_string(),
            institution_short_name: Some("UFPE".to_string()),
            institution_location: "Recife".to_string(),
            team_id: 2000,
            team_name: "Zero Bug".to_string(),
            team_rank: 2,
            team_total_members: 3,
            team_female_members: 1,
        }
    }

    #[tokio::test]
    async fn get_structure_by_year_requires_year() {
        let repo = MockCompetitionRepository::new();

        let result = get_structure_by_year(&repo, 10, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to specify the year."
        );
    }

    #[tokio::test]
    async fn get_structure_by_year_aggregates_event_teams() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_structure_by_year()
            .with(mockall::predicate::eq(10), mockall::predicate::eq(2024))
            .returning(|_, _| {
                Ok(vec![
                    row(),
                    CompetitionYearStructureRow {
                        team_id: 2001,
                        team_name: "Recife C".to_string(),
                        team_rank: 3,
                        ..row()
                    },
                ])
            });

        let result = get_structure_by_year(&repo, 10, Some(2024)).await.unwrap();

        assert_eq!(result.location_types.len(), 2);
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].teams.len(), 2);
    }

    #[tokio::test]
    async fn get_structure_by_year_returns_empty_structure_when_repository_returns_empty() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_structure_by_year()
            .with(mockall::predicate::eq(10), mockall::predicate::eq(2024))
            .returning(|_, _| Ok(vec![]));

        let result = get_structure_by_year(&repo, 10, Some(2024)).await.unwrap();

        assert!(result.location_types.is_empty());
        assert!(result.events.is_empty());
    }
}
