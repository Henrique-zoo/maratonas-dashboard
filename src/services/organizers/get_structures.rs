use std::collections::HashMap;

use crate::{dtos::organizers::output::{CompetitionSubStructure, EventSubStructure, OrganizerStructure}, errors::{AppError, AppResult}, repositories::OrganizerRepository};

pub async fn get_structures(
    repo: &dyn OrganizerRepository,
    organizer_ids: Option<Vec<i32>>
) -> AppResult<Vec<OrganizerStructure>> {
    let organizer_ids = organizer_ids
        .ok_or_else(|| AppError::BadRequest("You need to choose at least one organizer.".to_string()))?;

    let organizer_structures = repo
        .find_structures_by_ids(organizer_ids)
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut organizers, row| {
            let organizer = organizers
                .entry(row.organizer_id)
                .or_insert_with(|| {
                    OrganizerStructure::new(
                        row.organizer_id,
                        row.organizer_name,
                        row.organizer_website_url
                    )
                });

            if let Some(comp) = organizer
                .competitions
                .iter_mut()
                .find(|c| c.id == row.competition_id)
            {
                comp.events.push(EventSubStructure::new(row.event_id, row.event_name));
            } else {
                organizer.competitions.push(CompetitionSubStructure::new(
                    row.competition_id,
                    row.competition_name,
                    row.competition_website_url,
                    row.competition_gender_category,
                    vec![EventSubStructure::new(
                        row.event_id,
                        row.event_name
                    )]
                ));
            }

            organizers
        })
        .into_values()
        .collect();

    Ok(organizer_structures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MockOrganizerRepository;
    use crate::repositories::types::organizers::OrganizerStructureRow;
    use crate::shared::types::GenderCategory;

    #[tokio::test]
    async fn get_structure_returns_error_when_no_organizers_selected() {
        let repo = MockOrganizerRepository::new();

        let result = get_structures(&repo, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Bad request: You need to chose at least one organizer."
        );
    }

    #[tokio::test]
    async fn get_structure_returns_single_organizer_with_single_competition() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| {
                Ok(vec![
                    OrganizerStructureRow {
                        organizer_id: 1,
                        organizer_name: "ICPC Brazil".to_string(),
                        organizer_website_url: "https://icpc.org".to_string(),
                        competition_id: 10,
                        competition_name: "ICPC 2024".to_string(),
                        competition_gender_category: GenderCategory::Open,
                        competition_website_url: "https://icpc2024.org".to_string(),
                        event_id: 100,
                        event_name: "Regional".to_string(),
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "ICPC Brazil");
        assert_eq!(result[0].competitions.len(), 1);
        assert_eq!(result[0].competitions[0].id, 10);
        assert_eq!(result[0].competitions[0].events.len(), 1);
        assert_eq!(result[0].competitions[0].events[0].id, 100);
    }

    #[tokio::test]
    async fn get_structure_groups_events_by_competition() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| {
                Ok(vec![
                    OrganizerStructureRow {
                        organizer_id: 1,
                        organizer_name: "ICPC".to_string(),
                        organizer_website_url: "https://icpc.org".to_string(),
                        competition_id: 10,
                        competition_name: "ICPC 2024".to_string(),
                        competition_gender_category: GenderCategory::Open,
                        competition_website_url: "https://icpc2024.org".to_string(),
                        event_id: 100,
                        event_name: "Regional South".to_string(),
                    },
                    OrganizerStructureRow {
                        organizer_id: 1,
                        organizer_name: "ICPC".to_string(),
                        organizer_website_url: "https://icpc.org".to_string(),
                        competition_id: 10,
                        competition_name: "ICPC 2024".to_string(),
                        competition_gender_category: GenderCategory::Open,
                        competition_website_url: "https://icpc2024.org".to_string(),
                        event_id: 101,
                        event_name: "Regional Northeast".to_string(),
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].competitions.len(), 1);
        assert_eq!(result[0].competitions[0].events.len(), 2);
        assert_eq!(result[0].competitions[0].events[0].name, "Regional South");
        assert_eq!(result[0].competitions[0].events[1].name, "Regional Northeast");
    }

    #[tokio::test]
    async fn get_structure_groups_competitions_by_organizer() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| {
                Ok(vec![
                    OrganizerStructureRow {
                        organizer_id: 1,
                        organizer_name: "ICPC Brazil".to_string(),
                        organizer_website_url: "https://icpc.org".to_string(),
                        competition_id: 10,
                        competition_name: "ICPC 2024".to_string(),
                        competition_gender_category: GenderCategory::Open,
                        competition_website_url: "https://icpc2024.org".to_string(),
                        event_id: 100,
                        event_name: "Regional".to_string(),
                    },
                    OrganizerStructureRow {
                        organizer_id: 1,
                        organizer_name: "ICPC Brazil".to_string(),
                        organizer_website_url: "https://icpc.org".to_string(),
                        competition_id: 11,
                        competition_name: "ICPC Latin America".to_string(),
                        competition_gender_category: GenderCategory::Open,
                        competition_website_url: "https://icpclatam.org".to_string(),
                        event_id: 200,
                        event_name: "Finals".to_string(),
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].competitions.len(), 2);
        assert_eq!(result[0].competitions[0].name, "ICPC 2024");
        assert_eq!(result[0].competitions[1].name, "ICPC Latin America");
    }

    #[tokio::test]
    async fn get_structure_handles_multiple_organizers() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1, 2]))
            .returning(|_| {
                Ok(vec![
                    OrganizerStructureRow {
                        organizer_id: 1,
                        organizer_name: "ICPC".to_string(),
                        organizer_website_url: "https://icpc.org".to_string(),
                        competition_id: 10,
                        competition_name: "ICPC 2024".to_string(),
                        competition_gender_category: GenderCategory::Open,
                        competition_website_url: "https://icpc2024.org".to_string(),
                        event_id: 100,
                        event_name: "Regional".to_string(),
                    },
                    OrganizerStructureRow {
                        organizer_id: 2,
                        organizer_name: "OBI".to_string(),
                        organizer_website_url: "https://obi.org".to_string(),
                        competition_id: 20,
                        competition_name: "OBI 2024".to_string(),
                        competition_gender_category: GenderCategory::Open,
                        competition_website_url: "https://obi2024.org".to_string(),
                        event_id: 200,
                        event_name: "Fase 1".to_string(),
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1, 2])).await.unwrap();

        assert_eq!(result.len(), 2);
        let icpc = result.iter().find(|o| o.id == 1).unwrap();
        let obi = result.iter().find(|o| o.id == 2).unwrap();
        
        assert_eq!(icpc.name, "ICPC");
        assert_eq!(obi.name, "OBI");
    }
}