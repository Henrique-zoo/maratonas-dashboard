use indexmap::IndexMap;

use crate::{
    dtos::institutions::output::{
        InstitutionStructure, TeamSubStructure, TempCompetitionSubStructure, TempEventSubStructure,
        TempInstitutionStructure,
    },
    errors::{AppError, AppResult},
    repositories::InstitutionRepository,
};

pub async fn get_structures(
    repo: &dyn InstitutionRepository,
    institution_ids: Option<Vec<i32>>,
) -> AppResult<Vec<InstitutionStructure>> {
    let institution_ids = institution_ids.ok_or_else(|| {
        AppError::BadRequest("You need to choose at least one institution.".to_string())
    })?;

    let structures = repo
        .find_structures_by_ids(institution_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut institutions, row| {
            institutions
                .entry(row.institution_id)
                .or_insert_with(|| {
                    TempInstitutionStructure::new(
                        row.institution_id,
                        row.institution_name,
                        row.institution_total_teams,
                        row.institution_total_contestants,
                        row.institution_female_contestants,
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
                        row.competition_total_teams,
                        row.competition_total_participants,
                        IndexMap::new(),
                    )
                })
                .events
                .entry(row.event_id)
                .or_insert_with(|| {
                    TempEventSubStructure::new(
                        row.event_id,
                        row.event_name,
                        row.event_total_teams,
                        row.event_total_participants,
                        IndexMap::new(),
                    )
                })
                .teams
                .insert(
                    row.team_id,
                    TeamSubStructure::new(
                        row.team_id,
                        row.team_name,
                        row.team_event_rank,
                        row.team_total_members,
                        row.team_female_members,
                    ),
                );

            institutions
        })
        .into_values()
        .map(InstitutionStructure::from)
        .collect();

    Ok(structures)
}
