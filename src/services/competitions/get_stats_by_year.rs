use crate::{
    dtos::competitions::output::CompetitionYearStats,
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
};

pub async fn get_stats_by_year(
    repo: &dyn CompetitionRepository,
    competition_id: i32,
    year: Option<i32>,
) -> AppResult<CompetitionYearStats> {
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify the year.".to_string()))?;

    repo.find_competition_stats_by_year(competition_id, year)
        .await
        .map(CompetitionYearStats::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        errors::AppError,
        repositories::{MockCompetitionRepository, types::competitions::CompetitionYearStatsRow},
    };

    #[tokio::test]
    async fn get_stats_by_year_requires_year() {
        let repo = MockCompetitionRepository::new();

        let result = get_stats_by_year(&repo, 10, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to specify the year."
        );
    }

    #[tokio::test]
    async fn get_stats_by_year_maps_repository_row() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_competition_stats_by_year()
            .with(mockall::predicate::eq(10), mockall::predicate::eq(2024))
            .returning(|_, _| {
                Ok(CompetitionYearStatsRow {
                    total_institutions: 30,
                    total_teams: 90,
                    total_participants: 270,
                    female_participants: 81,
                })
            });

        let result = get_stats_by_year(&repo, 10, Some(2024)).await.unwrap();

        assert_eq!(result.total_institutions, 30);
        assert_eq!(result.total_teams, 90);
        assert_eq!(result.total_participants, 270);
        assert_eq!(result.female_participants, 81);
    }

    #[tokio::test]
    async fn get_stats_by_year_propagates_repository_error() {
        let mut repo = MockCompetitionRepository::new();
        repo.expect_find_competition_stats_by_year()
            .returning(|_, _| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_stats_by_year(&repo, 10, Some(2024)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
