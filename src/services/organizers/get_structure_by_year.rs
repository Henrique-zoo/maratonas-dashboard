use crate::{
    dtos::{
        competitions::output::OrganizerCompetitionYearStructure,
        organizers::output::EventSubStructure,
    },
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
};

pub async fn get_structure_by_year(
    repo: &dyn CompetitionRepository,
    competition_id: i32,
    year: Option<i32>,
) -> AppResult<OrganizerCompetitionYearStructure> {
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify the year.".to_string()))?;

    let structure = repo
        .find_events_by_year(competition_id, year)
        .await?
        .into_iter()
        .fold(
            OrganizerCompetitionYearStructure::default(),
            |mut competition, row| {
                if competition.events.is_empty() {
                    competition.update(row.competition_location_types)
                }

                competition.events.push(EventSubStructure::new(
                    row.event_id,
                    row.event_name,
                    row.event_level,
                    row.event_date,
                    row.event_total_institutions,
                    row.event_total_teams,
                    row.event_total_participants,
                    row.event_female_participants,
                    row.event_location_types,
                ));

                competition
            },
        );

    Ok(structure)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        repositories::{
            MockCompetitionRepository, types::competitions::CompetitionEventsByYearRow,
        },
        shared::types::LocationType,
    };

    fn row() -> CompetitionEventsByYearRow {
        CompetitionEventsByYearRow {
            competition_location_types: vec![LocationType::Country, LocationType::City],
            event_id: 111,
            event_name: "Semi".to_string(),
            event_level: Some(1),
            event_date: NaiveDate::from_ymd_opt(2024, 9, 20).unwrap(),
            event_location: "Brazil, Salvador".to_string(),
            event_total_institutions: 10,
            event_total_teams: 30,
            event_total_participants: 90,
            event_female_participants: 30,
            event_location_types: vec![LocationType::Country, LocationType::City],
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
    async fn get_structure_by_year_maps_events() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_events_by_year()
            .with(mockall::predicate::eq(10), mockall::predicate::eq(2024))
            .returning(|_, _| {
                Ok(vec![
                    row(),
                    CompetitionEventsByYearRow {
                        event_id: 112,
                        event_name: "Final".to_string(),
                        ..row()
                    },
                ])
            });

        let result = get_structure_by_year(&repo, 10, Some(2024)).await.unwrap();

        assert_eq!(result.location_types.len(), 2);
        assert_eq!(result.events.len(), 2);
    }

    #[tokio::test]
    async fn get_structure_by_year_returns_empty_structure_when_repository_returns_empty() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_events_by_year()
            .with(mockall::predicate::eq(10), mockall::predicate::eq(2024))
            .returning(|_, _| Ok(vec![]));

        let result = get_structure_by_year(&repo, 10, Some(2024)).await.unwrap();

        assert!(result.location_types.is_empty());
        assert!(result.events.is_empty());
    }
}
