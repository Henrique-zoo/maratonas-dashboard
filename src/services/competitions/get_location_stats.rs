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
