use crate::{
    dtos::filters::output::Filter, errors::AppResult, repositories::InstitutionRepository,
};

pub async fn get_options(
    repo: &dyn InstitutionRepository,
    competition_ids: Option<Vec<i32>>,
) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options_by_competitions(competition_ids)
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::repositories::{MockInstitutionRepository, types::IdNameRow};

    #[tokio::test]
    async fn get_options_maps_repository_rows() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_options_by_competitions()
            .with(mockall::predicate::eq(Some(vec![10])))
            .returning(|_| {
                Ok(vec![IdNameRow {
                    id: 5,
                    name: "UFRJ".to_string(),
                }])
            });

        let result = get_options(&repo, Some(vec![10])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 5);
        assert_eq!(result[0].name, "UFRJ");
    }

    #[tokio::test]
    async fn get_options_returns_empty_when_repository_returns_empty() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_options_by_competitions()
            .with(mockall::predicate::eq(None))
            .returning(|_| Ok(vec![]));

        let result = get_options(&repo, None).await.unwrap();

        assert!(result.is_empty());
    }
}
