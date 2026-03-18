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
