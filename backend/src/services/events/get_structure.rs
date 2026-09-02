//! Caso de uso que expõe a estrutura autoritativa de um evento por ano.

use crate::{
    dtos::events::responses::EventYearStructure,
    errors::{AppError, AppResult},
    repositories::EventRepository,
};

pub async fn get_structure(
    repo: &dyn EventRepository,
    event_id: i32,
    year: Option<i32>,
) -> AppResult<EventYearStructure> {
    let rows = repo.find_structure(event_id, year).await?;

    EventYearStructure::from_rows(rows).ok_or_else(|| {
        let scope = year.map(|value| format!(" in {value}")).unwrap_or_default();
        AppError::NotFound(format!(
            "Event {event_id} has no registered instance{scope}."
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::{
        repositories::{MockEventRepository, types::events::EventYearStructureRow},
        shared::types::{LocationType, Scope},
    };

    fn row() -> EventYearStructureRow {
        EventYearStructureRow {
            event_id: 10,
            event_name: "Regional".to_string(),
            event_level: Some(1),
            event_scope: Scope::Regional,
            competition_id: 1,
            competition_name: "ICPC".to_string(),
            selected_year: 2025,
            available_years: vec![2024, 2025],
            event_location_types: vec![LocationType::Country],
            event_instance_id: 100,
            event_date: NaiveDate::from_ymd_opt(2025, 5, 10).unwrap(),
            event_location: "Brazil".to_string(),
        }
    }

    #[tokio::test]
    async fn defaults_to_repository_selected_latest_year() {
        let mut repo = MockEventRepository::new();
        repo.expect_find_structure()
            .with(mockall::predicate::eq(10), mockall::predicate::eq(None))
            .returning(|_, _| Ok(vec![row()]));

        let structure = get_structure(&repo, 10, None).await.unwrap();

        assert_eq!(structure.year, 2025);
        assert_eq!(structure.instances.len(), 1);
    }

    #[tokio::test]
    async fn returns_not_found_for_an_unknown_year() {
        let mut repo = MockEventRepository::new();
        repo.expect_find_structure().returning(|_, _| Ok(vec![]));

        let error = get_structure(&repo, 10, Some(1999)).await.unwrap_err();

        assert!(matches!(error, AppError::NotFound(_)));
    }
}
