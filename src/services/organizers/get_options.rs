use crate::{dtos::filters::output::Filter, errors::AppResult, repositories::OrganizerRepository};

pub async fn get_options(repo: &dyn OrganizerRepository) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options()
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::types::IdNameRow;
    use crate::{errors::AppError, repositories::MockOrganizerRepository};

    #[tokio::test]
    async fn get_options_returns_filters_from_repository_rows() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_options().returning(|| {
            Ok(vec![
                IdNameRow {
                    id: 1,
                    name: "ICPC".to_string(),
                },
                IdNameRow {
                    id: 2,
                    name: "OBI".to_string(),
                },
                IdNameRow {
                    id: 3,
                    name: "Maratona".to_string(),
                },
            ])
        });

        let result = get_options(&repo).await.unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "ICPC");
        assert_eq!(result[1].id, 2);
        assert_eq!(result[1].name, "OBI");
        assert_eq!(result[2].id, 3);
        assert_eq!(result[2].name, "Maratona");
    }

    #[tokio::test]
    async fn get_options_returns_empty_when_no_organizers() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_options().returning(|| Ok(vec![]));

        let result = get_options(&repo).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_options_propagates_repository_error() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_options()
            .returning(|| Err(AppError::BadRequest("repository fail".to_string())));

        let result = get_options(&repo).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Bad request: repository fail"
        );
    }
}
