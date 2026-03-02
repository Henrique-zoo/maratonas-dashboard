use crate::{dtos::filters::output::Filter, errors::AppResult, repositories::InstitutionRepository};

pub async fn get_options(
    repo: &dyn InstitutionRepository,
    competition_ids: Option<Vec<i32>>
) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options_by_competitions(competition_ids)
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}