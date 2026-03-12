use indexmap::IndexMap;

use crate::{
    dtos::organizers::output::{
        EventSubStructure, OrganizerStructure, TempCompetitionSubStructure, TempOrganizerStructure,
    },
    errors::{AppError, AppResult},
    repositories::OrganizerRepository,
};

pub async fn get_structures(
    repo: &dyn OrganizerRepository,
    organizer_ids: Option<Vec<i32>>,
) -> AppResult<Vec<OrganizerStructure>> {
    let organizer_ids = organizer_ids.ok_or_else(|| {
        AppError::BadRequest("You need to choose at least one organizer.".to_string())
    })?;

    let structures = repo
        .find_structures_by_ids(organizer_ids)
        .await?
        .into_iter()
        .fold(IndexMap::new(), |mut organizers, row| {
            organizers
                .entry(row.organizer_id)
                .or_insert_with(|| {
                    TempOrganizerStructure::new(
                        row.organizer_id,
                        row.organizer_name,
                        row.organizer_website_url,
                        IndexMap::new(),
                    )
                })
                .competitions
                .entry(row.competition_id)
                .or_insert_with(|| {
                    TempCompetitionSubStructure::new(
                        row.competition_id,
                        row.competition_name,
                        row.competition_website_url,
                        row.competition_gender_category,
                        row.competition_total_teams,
                        row.competition_total_participants,
                        row.competition_female_participants,
                        IndexMap::new(),
                    )
                })
                .events
                .insert(
                    row.event_id,
                    EventSubStructure::new(
                        row.event_id,
                        row.event_name,
                        row.event_total_teams,
                        row.event_total_participants,
                        row.event_female_participants,
                    ),
                );

            organizers
        })
        .into_values()
        .map(OrganizerStructure::from)
        .collect();

    Ok(structures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::MockOrganizerRepository;
    use crate::repositories::types::organizers::OrganizerStructureRow;
    use crate::shared::types::GenderCategory;

    fn assert_f32_eq(left: f32, right: f32) {
        assert!((left - right).abs() < 1e-6);
    }

    fn base_row() -> OrganizerStructureRow {
        OrganizerStructureRow {
            organizer_id: 1,
            organizer_name: "ICPC Brazil".to_string(),
            organizer_website_url: "https://icpc.org".to_string(),
            competition_id: 10,
            competition_name: "ICPC 2024".to_string(),
            competition_gender_category: GenderCategory::Open,
            competition_website_url: "https://icpc2024.org".to_string(),
            competition_total_teams: 120,
            competition_total_participants: 360,
            competition_female_participants: 90,
            event_id: 100,
            event_name: "Regional".to_string(),
            event_total_teams: 30,
            event_total_participants: 90,
            event_female_participants: 22,
        }
    }

    #[tokio::test]
    async fn get_structure_returns_error_when_no_organizers_selected() {
        let repo = MockOrganizerRepository::new();

        let result = get_structures(&repo, None).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Bad request: You need to choose at least one organizer."
        );
    }

    #[tokio::test]
    async fn get_structure_returns_single_organizer_with_single_competition() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| Ok(vec![base_row()]));

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "ICPC Brazil");
        assert_eq!(result[0].competitions.len(), 1);
        assert_eq!(result[0].competitions[0].id, 10);
        assert_eq!(result[0].competitions[0].name, "ICPC 2024");
        assert_eq!(
            result[0].competitions[0].website_url,
            "https://icpc2024.org"
        );
        assert!(matches!(
            result[0].competitions[0].gender_category,
            GenderCategory::Open
        ));
        assert_eq!(result[0].competitions[0].total_teams, 120);
        assert_eq!(result[0].competitions[0].total_participants, 360);
        assert_f32_eq(result[0].competitions[0].female_percentage, 0.25);
        assert_eq!(result[0].competitions[0].events.len(), 1);
        assert_eq!(result[0].competitions[0].events[0].id, 100);
        assert_eq!(result[0].competitions[0].events[0].name, "Regional");
        assert_eq!(result[0].competitions[0].events[0].total_teams, 30);
        assert_eq!(result[0].competitions[0].events[0].total_participants, 90);
        assert_f32_eq(
            result[0].competitions[0].events[0].female_percentage,
            22.0 / 90.0,
        );
    }

    #[tokio::test]
    async fn get_structure_groups_events_by_competition() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| {
                Ok(vec![
                    OrganizerStructureRow {
                        organizer_name: "ICPC".to_string(),
                        event_name: "Regional South".to_string(),
                        ..base_row()
                    },
                    OrganizerStructureRow {
                        organizer_name: "ICPC".to_string(),
                        event_id: 101,
                        event_name: "Regional Northeast".to_string(),
                        event_total_teams: 32,
                        event_total_participants: 96,
                        event_female_participants: 24,
                        ..base_row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].competitions.len(), 1);
        assert_eq!(result[0].competitions[0].events.len(), 2);
        assert_eq!(result[0].competitions[0].events[0].name, "Regional South");
        assert_eq!(result[0].competitions[0].events[0].total_teams, 30);
        assert_eq!(result[0].competitions[0].events[0].total_participants, 90);
        assert_f32_eq(
            result[0].competitions[0].events[0].female_percentage,
            22.0 / 90.0,
        );
        assert_eq!(
            result[0].competitions[0].events[1].name,
            "Regional Northeast"
        );
        assert_eq!(result[0].competitions[0].events[1].total_teams, 32);
        assert_eq!(result[0].competitions[0].events[1].total_participants, 96);
        assert_f32_eq(result[0].competitions[0].events[1].female_percentage, 0.25);
    }

    #[tokio::test]
    async fn get_structure_groups_competitions_by_organizer() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1]))
            .returning(|_| {
                Ok(vec![
                    base_row(),
                    OrganizerStructureRow {
                        competition_id: 11,
                        competition_name: "ICPC Latin America".to_string(),
                        competition_website_url: "https://icpclatam.org".to_string(),
                        competition_total_teams: 200,
                        competition_total_participants: 600,
                        competition_female_participants: 180,
                        event_id: 200,
                        event_name: "Finals".to_string(),
                        event_total_teams: 50,
                        event_total_participants: 150,
                        event_female_participants: 45,
                        ..base_row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].competitions.len(), 2);

        let icpc_2024 = result[0].competitions.iter().find(|c| c.id == 10).unwrap();
        let icpc_latam = result[0].competitions.iter().find(|c| c.id == 11).unwrap();

        assert_eq!(icpc_2024.name, "ICPC 2024");
        assert_eq!(icpc_2024.total_teams, 120);
        assert_eq!(icpc_2024.total_participants, 360);
        assert_f32_eq(icpc_2024.female_percentage, 0.25);

        assert_eq!(icpc_latam.name, "ICPC Latin America");
        assert_eq!(icpc_latam.total_teams, 200);
        assert_eq!(icpc_latam.total_participants, 600);
        assert_f32_eq(icpc_latam.female_percentage, 0.3);
    }

    #[tokio::test]
    async fn get_structure_handles_multiple_organizers() {
        let mut repo = MockOrganizerRepository::new();

        repo.expect_find_structures_by_ids()
            .with(mockall::predicate::eq(vec![1, 2]))
            .returning(|_| {
                Ok(vec![
                    OrganizerStructureRow {
                        organizer_name: "ICPC".to_string(),
                        ..base_row()
                    },
                    OrganizerStructureRow {
                        organizer_id: 2,
                        organizer_name: "OBI".to_string(),
                        organizer_website_url: "https://obi.org".to_string(),
                        competition_id: 20,
                        competition_name: "OBI 2024".to_string(),
                        competition_website_url: "https://obi2024.org".to_string(),
                        competition_total_teams: 80,
                        competition_total_participants: 240,
                        competition_female_participants: 100,
                        event_id: 200,
                        event_name: "Fase 1".to_string(),
                        event_total_teams: 20,
                        event_total_participants: 60,
                        event_female_participants: 25,
                        ..base_row()
                    },
                ])
            });

        let result = get_structures(&repo, Some(vec![1, 2])).await.unwrap();

        assert_eq!(result.len(), 2);
        let icpc = result.iter().find(|o| o.id == 1).unwrap();
        let obi = result.iter().find(|o| o.id == 2).unwrap();

        assert_eq!(icpc.name, "ICPC");
        assert_eq!(icpc.competitions[0].total_teams, 120);
        assert_eq!(icpc.competitions[0].events[0].total_participants, 90);
        assert_eq!(obi.name, "OBI");
        assert_eq!(obi.competitions[0].total_teams, 80);
        assert_eq!(obi.competitions[0].events[0].total_participants, 60);
    }
}
