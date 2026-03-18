use crate::{
    dtos::events::output::EventLocationStats,
    errors::{AppError, AppResult},
    repositories::EventRepository,
    shared::types::LocationType,
};

pub async fn get_location_stats(
    repo: &dyn EventRepository,
    event_id: i32,
    location_type: Option<LocationType>,
    year: Option<i32>,
) -> AppResult<Vec<EventLocationStats>> {
    let location_type = location_type
        .ok_or_else(|| AppError::BadRequest("You need to specify a location type.".to_string()))?;
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify a year.".to_string()))?;

    let stats = repo
        .find_location_stats_by_event(event_id, location_type, year)
        .await?
        .into_iter()
        .map(EventLocationStats::from)
        .collect();

    Ok(stats)
}
