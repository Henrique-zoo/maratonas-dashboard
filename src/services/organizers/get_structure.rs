use std::collections::HashMap;

use crate::{dtos::organizers::output::{CompetitionSubStructure, EventSubStructure, OrganizerStructure}, errors::{AppError, AppResult}, repositories::OrganizerRepository};

pub async fn get_structure(
    repo: &dyn OrganizerRepository,
    organizer_ids: Option<Vec<i32>>
) -> AppResult<Vec<OrganizerStructure>> {
    let organizer_ids = organizer_ids
        .ok_or_else(|| AppError::BadRequest("You need to chose at least one organizer.".to_string()))?;
    
    let rows = repo
        .find_structure_by_organizer_ids(organizer_ids)
        .await?;

    let organizer_structures = rows.into_iter()
        .fold(HashMap::new(), |mut organizers, row| {
            let organizer = organizers.entry(row.organizer_id).or_insert_with(|| {
                OrganizerStructure {
                    id: row.organizer_id,
                    name: row.organizer_name,
                    website_url: row.organizer_website_url,
                    competitions: Vec::new()
                }
            });

            if let Some(comp) = organizer
                .competitions
                .iter_mut()
                .find(|c| c.id == row.competition_id)
            {
                comp.events.push(EventSubStructure{
                    id: row.event_id,
                    name: row.event_name 
                });
            } else {
                organizer.competitions.push(CompetitionSubStructure{
                    id: row.competition_id,
                    name: row.competition_name,
                    website_url: row.competition_website_url,
                    gender_category: row.competition_gender_category,
                    events: vec![EventSubStructure{
                        id: row.event_id,
                        name: row.event_name
                    }]
                })
            }

            organizers
        }
    )
    .into_values()
    .collect();

    Ok(organizer_structures)
}