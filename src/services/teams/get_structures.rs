use indexmap::IndexMap;

use crate::{
    dtos::teams::output::{
        EventSubStructure, TeamStructure, TempCompetitionSubStructure, TempTeamStructure,
    },
    errors::{AppError, AppResult},
    repositories::TeamRepository,
};

pub async fn get_structures(
    repo: &dyn TeamRepository,
    team_ids: Option<Vec<i32>>,
) -> AppResult<Vec<TeamStructure>> {
    let team_ids = team_ids
        .ok_or_else(|| AppError::BadRequest("You need to choose at least one team.".to_string()))?;

    let rows = repo
        .find_structures_by_ids(team_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut teams, row| {
            teams
                .entry(row.team_id)
                .or_insert_with(|| {
                    TempTeamStructure::new(row.team_id, row.team_name, IndexMap::new())
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
                        row.team_total_members,
                        row.team_female_members,
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
                        row.event_scope,
                        row.team_event_rank,
                    ),
                );

            teams
        })
        .into_values()
        .map(TeamStructure::from)
        .collect();

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        repositories::{MockTeamRepository, types::teams::TeamStructureRow},
        shared::types::{GenderCategory, Scope},
    };

    fn row() -> TeamStructureRow {
        TeamStructureRow {
            team_id: 1000,
            team_name: "Bit Masters".to_string(),
            team_total_members: 3,
            team_female_members: 1,
            competition_id: 10,
            competition_name: "ICPC".to_string(),
            competition_website_url: Some("https://icpc.org".to_string()),
            competition_gender_category: GenderCategory::Open,
            competition_years: vec![2023, 2024],
            event_id: 100,
            event_name: "Regional".to_string(),
            event_level: Some(1),
            event_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
            event_location: "Brazil, Sao Paulo".to_string(),
            event_scope: Scope::Regional,
            team_event_rank: 2,
        }
    }

    #[tokio::test]
    async fn get_structures_requires_team_ids() {
        let repo = MockTeamRepository::new();

        let result = get_structures(&repo, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to choose at least one team."
        );
    }

    #[tokio::test]
    async fn get_structures_aggregates_events_per_competition() {
        let mut repo = MockTeamRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1000]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    TeamStructureRow {
                        event_id: 101,
                        event_name: "Final".to_string(),
                        team_event_rank: 1,
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1000])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].competitions.len(), 1);
        assert_eq!(result[0].competitions[0].events.len(), 2);
    }

    #[tokio::test]
    async fn get_structures_supports_multiple_teams() {
        let mut repo = MockTeamRepository::new();
        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1000, 1001]))
            .returning(|_| {
                Ok(vec![
                    row(),
                    TeamStructureRow {
                        team_id: 1001,
                        team_name: "Zero Day".to_string(),
                        event_id: 200,
                        event_name: "Nacional".to_string(),
                        ..row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1000, 1001])).await.unwrap();

        assert_eq!(result.len(), 2);
    }
}
