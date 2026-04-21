use crate::{
    dtos::institutions::output::EventPerformance,
    errors::{AppError, AppResult},
    repositories::InstitutionRepository,
};

pub async fn get_event_performance_over_time(
    repo: &dyn InstitutionRepository,
    institution_id: i32,
    event_id: i32,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> AppResult<Vec<EventPerformance>> {
    let start_year = start_year
        .ok_or_else(|| AppError::BadRequest("You need to specify the start year.".to_string()))?;
    let end_year = end_year
        .ok_or_else(|| AppError::BadRequest("You need to specify the end year.".to_string()))?;

    let rows = repo
        .find_event_performance_over_time(institution_id, event_id, start_year, end_year)
        .await?
        .into_iter()
        .map(EventPerformance::from)
        .collect();

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::repositories::{
        MockInstitutionRepository, types::institutions::EventPerformanceRow,
    };

    #[tokio::test]
    async fn get_event_performance_over_time_requires_year_range() {
        let repo = MockInstitutionRepository::new();

        assert!(
            get_event_performance_over_time(&repo, 5, 100, None, Some(2024))
                .await
                .is_err()
        );
        assert!(
            get_event_performance_over_time(&repo, 5, 100, Some(2020), None)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn get_event_performance_over_time_maps_repository_rows() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_event_performance_over_time()
            .with(
                mockall::predicate::eq(5),
                mockall::predicate::eq(100),
                mockall::predicate::eq(2020),
                mockall::predicate::eq(2024),
            )
            .returning(|_, _, _, _| {
                Ok(vec![EventPerformanceRow {
                    year: 2024,
                    best_performance_rank: 1,
                    best_performance_team_id: 1000,
                    best_performance_team_name: "Rio Coders".to_string(),
                    medium_performance_rank: 2.4,
                }])
            });

        let result = get_event_performance_over_time(&repo, 5, 100, Some(2020), Some(2024))
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].year, 2024);
        assert_eq!(result[0].best_performance_rank, 1);
        assert_eq!(result[0].best_performance_team_name, "Rio Coders");
    }

    #[tokio::test]
    async fn get_event_performance_over_time_returns_empty_when_repository_returns_empty() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_event_performance_over_time()
            .returning(|_, _, _, _| Ok(vec![]));

        let result = get_event_performance_over_time(&repo, 5, 100, Some(2020), Some(2024))
            .await
            .unwrap();

        assert!(result.is_empty());
    }
}
