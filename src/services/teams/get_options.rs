use crate::{dtos::filters::output::Filter, errors::AppResult, repositories::TeamRepository};

pub async fn get_options(
    repo: &dyn TeamRepository,
    competition_ids: Option<Vec<i32>>,
    institution_ids: Option<Vec<i32>>
) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options_by_competitions_and_instructions(competition_ids, institution_ids)
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}