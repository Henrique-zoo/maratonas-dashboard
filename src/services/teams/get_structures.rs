use indexmap::IndexMap;

use crate::{
    dtos::teams::output::{
        EventSubStructure, TeamStructure, TempCompetitionSubStructure, TempTeamStructure,
    },
    errors::{AppError, AppResult},
    repositories::TeamRepository,
};

pub async fn get_structures(
    repo: &dyn TeamRepository,
    team_ids: Option<Vec<i32>>,
) -> AppResult<Vec<TeamStructure>> {
    let team_ids = team_ids
        .ok_or_else(|| AppError::BadRequest("You need to choose at least one team.".to_string()))?;

    let rows = repo
        .find_structures_by_ids(team_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut teams, row| {
            teams
                .entry(row.team_id)
                .or_insert_with(|| {
                    TempTeamStructure::new(
                        row.team_id,
                        row.team_name,
                        row.team_total_members,
                        row.team_female_members,
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
                        row.event_location,
                        row.event_scope,
                        row.team_event_rank,
                    ),
                );

            teams
        })
        .into_values()
        .map(TeamStructure::from)
        .collect();

    Ok(rows)
}
