use crate::{dtos::filters::output::Filter, errors::AppResult, repositories::CompetitionRepository};

pub async fn get_option(
    repo: &dyn CompetitionRepository,
    organizer_ids: Option<Vec<i32>>
) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options_by_organizers(organizer_ids)
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}