use crate::{
    dtos::{competitions::output::CompetitionYearStructure, organizers::output::EventSubStructure},
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
};

pub async fn get_structure_by_year(
    repo: &dyn CompetitionRepository,
    competition_id: i32,
    year: Option<i32>,
) -> AppResult<CompetitionYearStructure> {
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify the year.".to_string()))?;

    let structure = repo
        .find_competition_structure_by_year(competition_id, year)
        .await?
        .into_iter()
        .fold(
            CompetitionYearStructure::default(),
            |mut competition, row| {
                if competition.events.is_empty() {
                    competition.update(row.competition_location_types)
                }

                competition.events.push(EventSubStructure::new(
                    row.event_id,
                    row.event_name,
                    row.event_level,
                    row.event_date,
                    row.event_total_institutions,
                    row.event_total_teams,
                    row.event_total_participants,
                    row.event_female_participants,
                    row.event_location_types,
                ));

                competition
            },
        );

    Ok(structure)
}
