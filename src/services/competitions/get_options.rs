use crate::{
    dtos::filters::output::Filter, errors::AppResult, repositories::CompetitionRepository,
};

pub async fn get_options(
    repo: &dyn CompetitionRepository,
    organizer_ids: Option<Vec<i32>>,
) -> AppResult<Vec<Filter>> {
    let options = repo
        .find_options_by_organizers(organizer_ids)
        .await?
        .into_iter()
        .map(Filter::from)
        .collect();

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MockCompetitionRepository;
    use crate::repositories::types::IdNameRow;

    #[tokio::test]
    async fn get_option_returns_filters_from_repository_rows() {
        let mut repo = MockCompetitionRepository::new();

        repo.expect_find_options_by_organizers()
            .with(mockall::predicate::eq(Some(vec![1, 2])))
            .returning(|_| {
                Ok(vec![
                    IdNameRow {
                        id: 1,
                        name: "ICPC Brazil".to_string(),
                    },
                    IdNameRow {
                        id: 2,
                        name: "ICPC LatAm".to_string(),
                    },
                ])
            });

        let result = get_options(&repo, Some(vec![1, 2])).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "ICPC Brazil");
        assert_eq!(result[1].id, 2);
        assert_eq!(result[1].name, "ICPC LatAm");
    }
}
