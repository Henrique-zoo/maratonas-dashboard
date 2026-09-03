//! Caso de uso que lista os eventos históricos de uma instituição.

use crate::{
    dtos::institutions::responses::InstitutionEventOption, errors::AppResult,
    repositories::InstitutionRepository,
};

pub async fn get_event_options(
    repo: &dyn InstitutionRepository,
    institution_id: i32,
) -> AppResult<Vec<InstitutionEventOption>> {
    Ok(repo
        .find_event_options(institution_id)
        .await?
        .into_iter()
        .map(InstitutionEventOption::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        MockInstitutionRepository, types::institutions::InstitutionEventOptionRow,
    };

    #[tokio::test]
    async fn maps_historical_event_options() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_event_options().returning(|_| {
            Ok(vec![InstitutionEventOptionRow {
                event_id: 10,
                event_name: "Regional".to_string(),
                competition_id: 1,
                competition_name: "ICPC".to_string(),
                years: vec![2023, 2025],
            }])
        });

        let options = get_event_options(&repo, 100).await.unwrap();

        assert_eq!(options[0].id, 10);
        assert_eq!(options[0].years, vec![2023, 2025]);
    }
}
