use indexmap::IndexMap;

use crate::{
    dtos::organizers::output::{
        EventSubStructure, OrganizerStructure, TempCompetitionSubStructure, TempOrganizerStructure,
    },
    errors::{AppError, AppResult},
    repositories::OrganizerRepository,
};

pub async fn get_structures(
    repo: &dyn OrganizerRepository,
    organizer_ids: Option<Vec<i32>>,
) -> AppResult<Vec<OrganizerStructure>> {
    let organizer_ids = organizer_ids.ok_or_else(|| {
        AppError::BadRequest("You need to choose at least one organizer.".to_string())
    })?;

    let structures = repo
        .find_structures_by_ids(organizer_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut organizers, row| {
            organizers
                .entry(row.organizer_id)
                .or_insert_with(|| {
                    TempOrganizerStructure::new(
                        row.organizer_id,
                        row.organizer_name,
                        row.organizer_website_url,
                        IndexMap::new(),
                    )
                })
                .competitions
                .entry(row.competition_id)
                .or_insert_with(|| {
                    TempCompetitionSubStructure::new(
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
                .insert(
                    row.event_id,
                    EventSubStructure::new(
                        row.event_id,
                        row.event_name,
                        row.event_level,
                        row.event_date,
                        row.event_total_institutions,
                        row.event_total_teams,
                        row.event_total_participants,
                        row.event_female_participants,
                        row.event_location_types,
                    ),
                );

            organizers
        })
        .into_values()
        .map(OrganizerStructure::from)
        .collect();

    Ok(structures)
}
