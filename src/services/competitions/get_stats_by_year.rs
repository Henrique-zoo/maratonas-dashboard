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
