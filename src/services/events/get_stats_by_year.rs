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
