use crate::{dtos::filters::output::Filter, errors::AppResult, repositories::OrganizerRepository};

pub async fn get_options(
    repo: &dyn OrganizerRepository
) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options()
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}