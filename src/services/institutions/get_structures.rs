use crate::{dtos::institutions::output::InstitutionStructure, errors::{AppError, AppResult}, repositories::InstitutionRepository};

pub async fn get_structures(
    repo: &dyn InstitutionRepository,
    institution_ids: Option<Vec<i32>>
) -> AppResult<Vec<InstitutionStructure>> {
    let institution_ids = institution_ids
        .ok_or_else(|| AppError::BadRequest("You need to choose at least one institution.".to_string()))?;

    let structures = repo
        .find_structures_by_ids(institution_ids)
        .await?;

    let temp = vec![InstitutionStructure::new()];
    Ok(temp)
}