use crate::{dtos::filters::output::Filter, errors::AppResult, repositories::TeamRepository};

pub async fn get_options(
    repo: &dyn TeamRepository,
    competition_ids: Option<Vec<i32>>,
    institution_ids: Option<Vec<i32>>,
) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options_by_competitions_and_instructions(competition_ids, institution_ids)
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::repositories::{MockTeamRepository, types::IdNameRow};

    #[tokio::test]
    async fn get_options_maps_repository_rows() {
        let mut repo = MockTeamRepository::new();
        repo.expect_find_options_by_competitions_and_instructions()
            .with(
                mockall::predicate::eq(Some(vec![10])),
                mockall::predicate::eq(Some(vec![5])),
            )
            .returning(|_, _| {
                Ok(vec![IdNameRow {
                    id: 1000,
                    name: "Bit Masters".to_string(),
                }])
            });

        let result = get_options(&repo, Some(vec![10]), Some(vec![5]))
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1000);
        assert_eq!(result[0].name, "Bit Masters");
    }

    #[tokio::test]
    async fn get_options_returns_empty_when_repository_returns_empty() {
        let mut repo = MockTeamRepository::new();
        repo.expect_find_options_by_competitions_and_instructions()
            .with(mockall::predicate::eq(None), mockall::predicate::eq(None))
            .returning(|_, _| Ok(vec![]));

        let result = get_options(&repo, None, None).await.unwrap();

        assert!(result.is_empty());
    }
}
