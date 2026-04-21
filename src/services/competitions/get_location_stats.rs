use crate::{
    dtos::competitions::output::CompetitionYearLocationStats,
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
    shared::types::LocationType,
};

pub async fn get_location_stats(
    repo: &dyn CompetitionRepository,
    competition_id: i32,
    location_type: Option<LocationType>,
    year: Option<i32>,
) -> AppResult<Vec<CompetitionYearLocationStats>> {
    let location_type = location_type.ok_or_else(|| {
        AppError::BadRequest("You need to specify the location type.".to_string())
    })?;
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify the year.".to_string()))?;

    let stats = repo
        .find_location_stats_by_competition(competition_id, location_type, year)
        .await?
        .into_iter()
        .map(CompetitionYearLocationStats::from)
        .collect();

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        errors::AppError,
        repositories::{
            MockCompetitionRepository, types::competitions::CompetitionLocationStatsRow,
        },
    };

    fn assert_f32_eq(left: f32, right: f32) {
        assert!((left - right).abs() < 1e-6);
    }

    #[tokio::test]
    async fn get_location_stats_requires_location_type_and_year() {
        let repo = MockCompetitionRepository::new();

        assert!(
            get_location_stats(&repo, 10, None, Some(2024))
                .await
                .is_err()
        );
        assert!(
            get_location_stats(&repo, 10, Some(LocationType::Country), None)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn get_location_stats_maps_rows_to_dto() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_location_stats_by_competition()
            .with(
                mockall::predicate::eq(10),
                mockall::predicate::eq(LocationType::Country),
                mockall::predicate::eq(2024),
            )
            .returning(|_, _, _| {
                Ok(vec![CompetitionLocationStatsRow {
                    location_id: 1,
                    location_name: "Brazil".to_string(),
                    total_institutions: 20,
                    total_teams: 40,
                    total_participants: 120,
                    female_participants: 36,
                }])
            });

        let result = get_location_stats(&repo, 10, Some(LocationType::Country), Some(2024))
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "Brazil");
        assert_f32_eq(result[0].female_percentage, 0.3);
    }

    #[tokio::test]
    async fn get_location_stats_returns_empty_when_repository_returns_empty() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_location_stats_by_competition()
            .returning(|_, _, _| Ok(vec![]));

        let result = get_location_stats(&repo, 10, Some(LocationType::Country), Some(2024))
            .await
            .unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_location_stats_propagates_repository_error() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_location_stats_by_competition()
            .returning(|_, _, _| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_location_stats(&repo, 10, Some(LocationType::Country), Some(2024)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
