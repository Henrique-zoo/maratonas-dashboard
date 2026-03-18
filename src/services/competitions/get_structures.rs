use indexmap::IndexMap;

use crate::{
    dtos::competitions::output::{
        CompetitionStructure, TeamSubStructure, TempCompetitionStructure, TempEventSubStructure,
    },
    errors::{AppError, AppResult},
    repositories::CompetitionRepository,
};

pub async fn get_structures(
    repo: &dyn CompetitionRepository,
    competition_ids: Option<Vec<i32>>,
) -> AppResult<Vec<CompetitionStructure>> {
    let competition_ids = competition_ids.ok_or_else(|| {
        AppError::BadRequest("You need to choose at least one competition.".to_string())
    })?;

    let structures = repo
        .find_structures_by_ids(competition_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut competitions, row| {
            competitions
                .entry(row.competition_id)
                .or_insert_with(|| {
                    TempCompetitionStructure::new(
                        row.competition_id,
                        row.competition_name,
                        row.competition_website_url,
                        row.competition_gender_category,
                        row.competition_years,
                        row.competition_location_types,
                        IndexMap::new(),
                    )
                })
                .events
                .entry(row.event_id)
                .or_insert_with(|| {
                    TempEventSubStructure::new(
                        row.event_id,
                        row.event_name,
                        row.event_level,
                        row.event_date,
                        row.event_location,
                        row.event_location_types,
                        IndexMap::new(),
                    )
                })
                .teams
                .insert(
                    row.team_id,
                    TeamSubStructure::new(
                        row.team_id,
                        row.team_name,
                        row.team_rank,
                        row.institution_name,
                        row.institution_short_name,
                        row.institution_location,
                        row.team_total_members,
                        row.team_female_members,
                    ),
                );

            competitions
        })
        .into_values()
        .map(CompetitionStructure::from)
        .collect();

    Ok(structures)
}
