use std::collections::HashMap;

use crate::{dtos::competitions::output::{CompetitionStructure, EventSubStructure, TeamSubStructure}, errors::{AppError, AppResult}, repositories::CompetitionRepository};

pub async fn get_structures(
    repo: &dyn CompetitionRepository,
    competition_ids: Option<Vec<i32>>
) -> AppResult<Vec<CompetitionStructure>> {
    let competition_ids = competition_ids
        .ok_or_else(|| AppError::BadRequest("You need to choose at least one competition.".to_string()))?;

    let competition_structures = repo.find_structures_by_ids(competition_ids)
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut competitions, row| {
            let competition = competitions
                .entry(row.competition_id)
                .or_insert_with(|| {
                    CompetitionStructure::new(
                        row.competition_id,
                        row.competition_name,
                        row.competition_website_url,
                        row.competition_gender_category
                    )
                });

            if let Some(event) = competition
                .events
                .iter_mut()
                .find(|e| e.id == row.event_id)
            {
                event.teams.push(TeamSubStructure::new(
                    row.team_id,
                    row.team_name,
                    row.team_total_members,
                    row.team_female_members
                ));
            } else {
                competition.events.push(EventSubStructure::new(
                    row.event_id,
                    row.event_name,
                    row.event_level,
                    row.event_date,
                    row.event_location,
                    row.event_total_participants,
                    row.event_female_participants,
                    vec![TeamSubStructure::new(
                        row.team_id,
                        row.team_name,
                        row.team_total_members,
                        row.team_female_members
                    )]
                ));
            }

            competitions
        })
        .into_values()
        .collect();

    Ok(competition_structures)
}

#[cfg(test)]
mod tests {
    use crate::{repositories::{MockCompetitionRepository, types::competitions::CompetitionStructureRow}, shared::types::GenderCategory};

    use super::*;
    use chrono::NaiveDate;
    use mockall::predicate::*;

    fn sample_rows() -> Vec<CompetitionStructureRow> {
        vec![
            CompetitionStructureRow {
                competition_id: 1,
                competition_name: "World Cup".to_string(),
                competition_gender_category: GenderCategory::Open,
                competition_website_url: Some("https://worldcup.com".to_string()),

                event_id: 10,
                event_name: "Final".to_string(),
                event_level: Some(1),
                event_date: NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
                event_location: "Brazil, São Paulo".to_string(),
                event_total_participants: 20,
                event_female_participants: 8,

                team_id: 100,
                team_name: "Team A".to_string(),
                team_total_members: 10,
                team_female_members: 4,
            },
            CompetitionStructureRow {
                competition_id: 1,
                competition_name: "World Cup".to_string(),
                competition_gender_category: GenderCategory::Open,
                competition_website_url: Some("https://worldcup.com".to_string()),

                event_id: 10,
                event_name: "Final".to_string(),
                event_level: Some(1),
                event_date: NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
                event_location: "Brazil, São Paulo".to_string(),
                event_total_participants: 20,
                event_female_participants: 8,

                team_id: 101,
                team_name: "Team B".to_string(),
                team_total_members: 10,
                team_female_members: 4,
            },
        ]
    }

    #[tokio::test]
    async fn should_build_competition_structure_correctly() {
        let mut repo = MockCompetitionRepository::new();

        repo.expect_find_structures_by_ids()
            .with(eq(vec![1]))
            .times(1)
            .returning(|_| Ok(sample_rows()));

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);

        let competition = &result[0];
        assert_eq!(competition.id, 1);
        assert_eq!(competition.events.len(), 1);

        let event = &competition.events[0];
        assert_eq!(event.id, 10);
        assert_eq!(event.teams.len(), 2);
        assert_eq!(event.teams[0].name, "Team A");
        assert_eq!(event.teams[1].name, "Team B");
    }

    #[tokio::test]
    async fn should_return_error_if_competition_ids_is_none() {
        let repo = MockCompetitionRepository::new();

        let result = get_structures(&repo, None).await;

        match result {
            Err(AppError::BadRequest(msg)) => {
                assert!(msg.contains("at least one competition"));
            }
            _ => panic!("Expected BadRequest error"),
        }
    }
}