use crate::{
    dtos::events::output::EventYearStats,
    errors::{AppError, AppResult},
    repositories::EventRepository,
};

pub async fn get_stats_by_year(
    repo: &dyn EventRepository,
    event_id: i32,
    year: Option<i32>,
) -> AppResult<EventYearStats> {
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify the year.".to_string()))?;

    repo.find_event_stats_by_year(event_id, year)
        .await
        .map(EventYearStats::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        errors::AppError,
        repositories::{MockEventRepository, types::events::EventYearStatsRow},
    };

    #[tokio::test]
    async fn get_stats_by_year_requires_year() {
        let repo = MockEventRepository::new();

        let result = get_stats_by_year(&repo, 20, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Bad request: You need to specify the year."
        );
    }

    #[tokio::test]
    async fn get_stats_by_year_maps_repository_row() {
        let mut repo = MockEventRepository::new();
        repo.expect_find_event_stats_by_year()
            .with(mockall::predicate::eq(20), mockall::predicate::eq(2024))
            .returning(|_, _| {
                Ok(EventYearStatsRow {
                    total_institutions: 10,
                    total_teams: 25,
                    total_participants: 75,
                    female_participants: 20,
                })
            });

        let result = get_stats_by_year(&repo, 20, Some(2024)).await.unwrap();

        assert_eq!(result.total_institutions, 10);
        assert_eq!(result.total_teams, 25);
        assert_eq!(result.total_participants, 75);
        assert_eq!(result.female_participants, 20);
    }

    #[tokio::test]
    async fn get_stats_by_year_propagates_repository_error() {
        let mut repo = MockEventRepository::new();
        repo.expect_find_event_stats_by_year()
            .returning(|_, _| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_stats_by_year(&repo, 20, Some(2024)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
