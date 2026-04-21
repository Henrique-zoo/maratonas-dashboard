use crate::{
    dtos::{competitions::output::TeamCompetitionYearStructure, teams::output::EventSubStructure},
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
};

pub async fn get_structure_by_year(
    repo: &dyn CompetitionRepository,
    team_id: i32,
    competition_id: i32,
    year: Option<i32>,
) -> AppResult<TeamCompetitionYearStructure> {
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify the year.".to_string()))?;

    let results = repo
        .find_team_result_by_year(team_id, competition_id, year)
        .await?
        .into_iter()
        .fold(
            TeamCompetitionYearStructure::default(),
            |mut competition, row| {
                competition.events.push(EventSubStructure::new(
                    row.event_id,
                    row.event_name,
                    row.event_level,
                    row.event_date,
                    row.event_location,
                    row.event_scope,
                    row.team_event_rank,
                ));

                competition
            },
        );

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        repositories::{
            MockCompetitionRepository, types::competitions::CompetitionTeamYearResultRow,
        },
        shared::types::Scope,
    };

    fn row() -> CompetitionTeamYearResultRow {
        CompetitionTeamYearResultRow {
            team_total_members: 3,
            team_female_members: 1,
            event_id: 500,
            event_name: "Regional South".to_string(),
            event_level: Some(1),
            event_date: NaiveDate::from_ymd_opt(2024, 8, 10).unwrap(),
            event_location: "Brazil, Porto Alegre".to_string(),
            event_scope: Scope::Regional,
            team_event_rank: 4,
        }
    }

    #[tokio::test]
    async fn get_structure_by_year_requires_year() {
        let repo = MockCompetitionRepository::new();

        let result = get_structure_by_year(&repo, 1000, 10, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to specify the year."
        );
    }

    #[tokio::test]
    async fn get_structure_by_year_maps_events() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_team_result_by_year()
            .with(
                mockall::predicate::eq(1000),
                mockall::predicate::eq(10),
                mockall::predicate::eq(2024),
            )
            .returning(|_, _, _| {
                Ok(vec![
                    row(),
                    CompetitionTeamYearResultRow {
                        event_id: 501,
                        event_name: "Final".to_string(),
                        ..row()
                    },
                ])
            });

        let result = get_structure_by_year(&repo, 1000, 10, Some(2024))
            .await
            .unwrap();

        assert_eq!(result.events.len(), 2);
        assert_eq!(result.events[0].team_event_rank, 4);
    }

    #[tokio::test]
    async fn get_structure_by_year_returns_empty_when_repository_returns_empty() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_team_result_by_year()
            .with(
                mockall::predicate::eq(1000),
                mockall::predicate::eq(10),
                mockall::predicate::eq(2024),
            )
            .returning(|_, _, _| Ok(vec![]));

        let result = get_structure_by_year(&repo, 1000, 10, Some(2024))
            .await
            .unwrap();

        assert!(result.events.is_empty());
    }
}
